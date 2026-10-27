#!/usr/bin/env bun
/** Validate catalog coverage, formatting tokens and localized QuickStart guides. */
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';

const root = resolve(import.meta.dir, '..');
const locales = ['en','zh-CN','zh-TW','ja','ko','de','fr','es','pt-BR','ru','it','nl','pl','tr','uk','vi','id','th','ar','hi','bn','fa','he','sv','cs','ro'];
const sources = ['src/ui.rs', 'native/qt.cpp', 'native/gnome.c'];
const requiredCore = ['设置','导入','卸载应用','检查更新','安装更新','语言','外观','保存更新源','应用信息','安装位置','打开目录','取消','关闭','浅色','深色','界面后端','跟随系统','保存并扫描','项目主页','选择本地 .AppImage 文件','确认','保存'];
const intentionallyEnglish = /失败|无效|无法|超出|通讯|没有返回|超时|不能包含|必须是|请输入有效|请选择绝对|找不到|未正常完成/;
const errors: string[] = [];

function literals(source: string): Set<string> {
  const found = new Set<string>();
  for (let i = 0; i < source.length; i++) {
    if (source.slice(i, i + 2) === '//') { const end = source.indexOf('\n', i); if (end < 0) break; i = end; continue; }
    if (source.slice(i, i + 2) === '/*') { const end = source.indexOf('*/', i + 2); i = end < 0 ? source.length : end + 1; continue; }
    // Skip generated C++ raw-string catalog snapshots, not actual UI calls.
    if (source.slice(i, i + 2) === 'R"') {
      const open = source.indexOf('(', i + 2);
      if (open > i && open - i <= 20) {
        const delimiter = source.slice(i + 2, open); const endToken = ')' + delimiter + '"';
        const end = source.indexOf(endToken, open + 1);
        if (end >= 0) { i = end + endToken.length - 1; continue; }
      }
    }
    if (source[i] !== '"') continue;
    let value = '';
    for (i++; i < source.length; i++) {
      if (source[i] === '"') break;
      if (source[i] === '\\') { value += source[i]; i++; if (i < source.length) value += source[i]; }
      else value += source[i];
    }
    if (!/\p{Script=Han}/u.test(value)) continue;
    try { found.add(JSON.parse('"' + value + '"')); }
    catch { errors.push('Unparsed source literal: ' + value.slice(0, 120)); }
  }
  return found;
}
function tokens(value: string): string[] {
  return [...value.matchAll(/%(?:\d+\$)?(?:\d+|[sduif])|\{[^{}]*\}/g)].map(m => m[0]).sort();
}
function json(path: string): Record<string,string> {
  const value = JSON.parse(readFileSync(path, 'utf8'));
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw Error('Catalog is not an object: ' + path);
  return value;
}
const en = json(resolve(root, 'resources/i18n/en.json'));
const keys = Object.keys(en);
const sourceKeys = new Set<string>();
for (const source of sources) for (const key of literals(readFileSync(resolve(root, source), 'utf8'))) sourceKeys.add(key);
for (const key of sourceKeys) if (!(key in en)) errors.push('Missing English source msgid: ' + key);
for (const [key, value] of Object.entries(en)) {
  if (typeof value !== 'string' || !value.trim()) errors.push('Invalid English value: ' + key);
  else {
    if (/\p{Script=Han}/u.test(value)) errors.push('Untranslated Chinese in English value: ' + key);
    if (JSON.stringify(tokens(key)) !== JSON.stringify(tokens(value))) errors.push('English placeholder mismatch: ' + key);
    if (value.includes('MIT License')) errors.push('Stale project license in translated value: ' + key);
  }
}
const report: {locale:string;keys:number;coreKeys:number;localizedValues:number;englishDiagnosticFallbacks:string[];otherEnglishIdentical:string[];quickStart:boolean}[] = [];
for (const locale of locales) {
  const file = resolve(root, 'resources/i18n', locale + '.json');
  if (!existsSync(file)) { errors.push('Missing locale catalog: ' + locale); continue; }
  const catalog = json(file);
  const ownKeys = Object.keys(catalog);
  for (const key of keys) {
    const value = catalog[key];
    if (typeof value !== 'string' || !value.trim()) { errors.push(locale + ' missing/empty key: ' + key); continue; }
    if (JSON.stringify(tokens(key)) !== JSON.stringify(tokens(value))) errors.push(locale + ' placeholder mismatch: ' + key);
    if (value.includes('MIT License')) errors.push(locale + ' stale project license: ' + key);
    if (!locale.startsWith('zh') && /\p{Script=Han}/u.test(value) && !['ja'].includes(locale)) errors.push(locale + ' contains untranslated Chinese: ' + key);
  }
  for (const key of ownKeys) if (!(key in en)) errors.push(locale + ' unknown msgid: ' + key);
  const core = requiredCore.filter(key => typeof catalog[key] === 'string' && catalog[key].trim());
  if (core.length !== requiredCore.length) errors.push(locale + ' incomplete core coverage');
  // Terms such as Version legitimately match English in some languages. This
  // threshold catches an English copy while allowing shared technical names.
  const localized = keys.filter(key => catalog[key] !== en[key]).length;
  if (locale !== 'en' && localized < Math.floor(keys.length * 0.65)) errors.push(locale + ' has insufficient localized text: ' + localized + '/' + keys.length);
  const fallback = locale === 'en' || locale === 'zh-CN' ? [] : keys.filter(key => catalog[key] === en[key] && intentionallyEnglish.test(key));
  const identical = locale === 'en' ? [] : keys.filter(key => catalog[key] === en[key] && !intentionallyEnglish.test(key));
  const quickStartPath = resolve(root, 'docs/i18n', locale + '.md');
  const quickStart = existsSync(quickStartPath);
  if (!quickStart) errors.push('Missing localized QuickStart: ' + locale);
  else {
    const doc = readFileSync(quickStartPath, 'utf8');
    for (const required of ['~/Applications','apt install','dnf install','pacman -U','chmod +x','GPUI','Qt','GTK','owner/repository','SHA','AGPL','Error','Info','Warning','https://github.com/xingwangzhe/AppimageMannger']) {
      if (!doc.includes(required)) errors.push(locale + ' QuickStart lacks: ' + required);
    }
    if (/TODO|\{\{[^}]+\}\}/.test(doc)) errors.push(locale + ' QuickStart contains an unfinished marker');
  }
  report.push({locale,keys:ownKeys.length,coreKeys:core.length,localizedValues:localized,englishDiagnosticFallbacks:fallback,otherEnglishIdentical:identical,quickStart});
}
const summary = {localeCount:report.length,sourceMsgids:sourceKeys.size,catalogMsgids:keys.length,requiredCoreKeys:requiredCore.length,placeholderErrors:errors.filter(e=>e.includes('placeholder')).length,errors,locales:report};
if (process.argv.includes('--write')) {
  writeFileSync(resolve(root,'docs/i18n/coverage.json'), JSON.stringify(summary,null,2)+'\n');
  const lines = ['# Localization coverage', '',
    'The application and QuickStart guides support 26 locales. Catalog keys are exact Chinese UI msgids shared by GPUI, Qt, and GTK. Equivalent wording is intentionally normalized to concise, native-language text.', '',
    '`Error`, `Info`, and `Warning` severity labels remain English. Thirty uncommon diagnostic messages intentionally use English fallback outside Simplified Chinese. All core actions, settings, application details, import, updates, uninstall, and About are localized. Technical names and words shared by a language and English may be identical; they are reported separately from fallback.', '',
    `Source UI msgids: **${sourceKeys.size}**. Catalog msgids: **${keys.length}**, including compatibility aliases. Required core keys: **${requiredCore.length}**. Placeholder mismatches: **${summary.placeholderErrors}**.`, '',
    '| Locale | Catalog keys | Core keys | Values differing from English | English diagnostic fallbacks | QuickStart |',
    '| --- | ---: | ---: | ---: | ---: | --- |',
    ...report.map(r=>`| ${r.locale} | ${r.keys} | ${r.coreKeys}/${requiredCore.length} | ${r.localizedValues} | ${r.englishDiagnosticFallbacks.length} | ${r.quickStart?'Yes':'Missing'} |`), '',
    'Run `bun run scripts/check-i18n.ts` to verify source coverage, catalog consistency, formatting placeholders, core coverage, and guide contents. Use `--write` to refresh this report. The machine-readable report lists every English fallback and shared value.', '',
    '## English diagnostic fallback messages', '',
    ...report.find(r=>r.locale==='ja')!.englishDiagnosticFallbacks.map(key=>'- '+en[key]), '',
    errors.length?'## Validation errors\n\n'+errors.map(e=>'- '+e).join('\n'):'Validation passed without errors.', ''];
  writeFileSync(resolve(root,'docs/i18n/COVERAGE.md'),lines.join('\n'));
}
if (errors.length) { for (const error of errors) console.error(error); process.exit(1); }
console.log(`i18n: ${report.length} locales, ${keys.length} keys, ${sourceKeys.size} source msgids, ${requiredCore.length}/${requiredCore.length} core keys per locale, no placeholder mismatches; all 26 QuickStarts validated.`);
