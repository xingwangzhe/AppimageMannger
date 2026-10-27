#!/usr/bin/env bun
/** Generate AUR-ready source and binary recipes with real SHA-256 checksums. */
import { archPackageVersion } from "./packaging-common";
import { createHash } from "node:crypto";
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve, join } from "node:path";
const root = resolve(import.meta.dir, ".."); process.chdir(root);
const out = resolve(process.argv[2] || "dist"); mkdirSync(out, { recursive: true });
const version = readFileSync("Cargo.toml", "utf8").match(/^version\s*=\s*"([^"]+)"/m)?.[1];
if (!version) throw new Error("No package version");
if (!/license\s*=\s*"AGPL-3\.0-only"/.test(readFileSync("Cargo.toml", "utf8"))) throw new Error("AUR metadata requires AGPL-3.0-only");
const aurVersion = archPackageVersion(version);
const recipes = resolve("work/aur");
const repo = "https://github.com/xingwangzhe/AppimageMannger";
const sourceName = `AppimageMannger-${version}-source.tar.gz`;
function run(command: string[]) { const p = Bun.spawnSync(command, { stdout: "pipe", stderr: "pipe" }); if (p.exitCode) throw new Error(p.stderr.toString()); return p.stdout.toString(); }
run(["git", "archive", "--format=tar.gz", `--prefix=AppimageMannger-${version}/`, "-o", join(out, sourceName), "HEAD"]);
const sum = (file: string) => createHash("sha256").update(readFileSync(join(out,file))).digest("hex");
const dependencies = ["fontconfig", "libxkbcommon", "libxkbcommon-x11", "libxcb", "libx11", "wayland", "vulkan-icd-loader", "gcc-libs", "qt6-base", "qt6-wayland", "gtk4", "libadwaita", "json-glib", "squashfs-tools", "xdg-utils"];
const sourceSum = sum(sourceName);
const sourcePkg = `# Maintainer: xingwangzhe <xingwangzhe@outlook.com>\npkgname=appimage-mannger\npkgver=${aurVersion}\n_release=${version}\npkgrel=1\npkgdesc='Native Linux AppImage manager built with Ely and GPUI'\narch=('x86_64' 'aarch64')\nurl='${repo}'\nlicense=('AGPL-3.0-only')\ndepends=(${dependencies.map(d=>`'${d}'`).join(' ')})\nmakedepends=('rust' 'cargo' 'clang' 'pkgconf' 'cmake' 'bun')\nsource=("${repo}/releases/download/v$_release/${sourceName}")\nsha256sums=('${sourceSum}')\n\nprepare() {\n  cd "AppimageMannger-$_release"\n  export CARGO_HOME="$srcdir/cargo-home"\n  cargo fetch --locked\n}\n\nbuild() {\n  cd "AppimageMannger-$_release"\n  export CARGO_HOME="$srcdir/cargo-home"\n  cargo build --release --locked --offline\n}\n\ncheck() {\n  cd "AppimageMannger-$_release"\n  export CARGO_HOME="$srcdir/cargo-home"\n  cargo test --locked --offline\n}\n\npackage() {\n  cd "AppimageMannger-$_release"\n  install -Dm755 target/release/appimage-mannger "$pkgdir/usr/bin/appimage-mannger"\n  target/release/appimage-mannger --export-frontends "$pkgdir/usr/lib/appimage-mannger"\n  install -Dm644 packaging/appimage-mannger.desktop "$pkgdir/usr/share/applications/appimage-mannger.desktop"\n  install -Dm644 packaging/appimage-mannger.svg "$pkgdir/usr/share/icons/hicolor/scalable/apps/appimage-mannger.svg"\n  export CARGO_HOME="$srcdir/cargo-home"\n  bun run scripts/package-licenses.ts --destination "$pkgdir/usr/share/licenses/$pkgname"\n  mkdir -p "$pkgdir/usr/share/doc/appimage-mannger"\n  cp -R docs/i18n "$pkgdir/usr/share/doc/appimage-mannger/i18n"\n  install -Dm644 README.md "$pkgdir/usr/share/doc/appimage-mannger/README.md"\n}\n`;
const sourceInfo = `pkgbase = appimage-mannger\n\tpkgdesc = Native Linux AppImage manager built with Ely and GPUI\n\tpkgver = ${aurVersion}\n\tpkgrel = 1\n\turl = ${repo}\n\tarch = x86_64\n\tarch = aarch64\n\tlicense = AGPL-3.0-only\n${["rust","cargo","clang","pkgconf","cmake","bun"].map(d=>`\tmakedepends = ${d}\n`).join('')}${dependencies.map(d=>`\tdepends = ${d}\n`).join('')}\tsource = ${repo}/releases/download/v${version}/${sourceName}\n\tsha256sums = ${sourceSum}\n\npkgname = appimage-mannger\n`;
const binaryNames = Object.fromEntries(["x86_64", "aarch64"].map(a => [a, `AppimageMannger-${version}-${a}.tar.gz`]));
const binarySums = Object.fromEntries(Object.entries(binaryNames).map(([a,n]) => [a,sum(n)]));
const binaryPkg = `# Maintainer: xingwangzhe <xingwangzhe@outlook.com>\npkgname=appimage-mannger-bin\npkgver=${aurVersion}\n_release=${version}\npkgrel=1\npkgdesc='Native Linux AppImage manager built with Ely and GPUI (binary)'\narch=('x86_64' 'aarch64')\nurl='${repo}'\nlicense=('AGPL-3.0-only')\ndepends=(${dependencies.map(d=>`'${d}'`).join(' ')})\nprovides=("appimage-mannger=$pkgver")\nconflicts=('appimage-mannger')\n${Object.entries(binaryNames).map(([a,n])=>`source_${a}=("${repo}/releases/download/v$_release/${n}")\nsha256sums_${a}=('${binarySums[a]}')\n`).join('')}\npackage() {\n  cd "AppimageMannger-$_release-$CARCH"\n  install -Dm755 usr/bin/appimage-mannger "$pkgdir/usr/bin/appimage-mannger"\n  mkdir -p "$pkgdir/usr/lib"\n  cp -R usr/lib/appimage-mannger "$pkgdir/usr/lib/"\n  install -Dm644 usr/share/applications/appimage-mannger.desktop "$pkgdir/usr/share/applications/appimage-mannger.desktop"\n  install -Dm644 usr/share/icons/hicolor/scalable/apps/appimage-mannger.svg "$pkgdir/usr/share/icons/hicolor/scalable/apps/appimage-mannger.svg"\n  mkdir -p "$pkgdir/usr/share/licenses"\n  cp -R usr/share/licenses/appimage-mannger "$pkgdir/usr/share/licenses/$pkgname"\n  mkdir -p "$pkgdir/usr/share/doc"\n  cp -R usr/share/doc/appimage-mannger "$pkgdir/usr/share/doc/"\n}\n`;
const binaryInfo = `pkgbase = appimage-mannger-bin\n\tpkgdesc = Native Linux AppImage manager built with Ely and GPUI (binary)\n\tpkgver = ${aurVersion}\n\tpkgrel = 1\n\turl = ${repo}\n\tarch = x86_64\n\tarch = aarch64\n\tlicense = AGPL-3.0-only\n${dependencies.map(d=>`\tdepends = ${d}\n`).join('')}\tprovides = appimage-mannger=${aurVersion}\n\tconflicts = appimage-mannger\n${Object.entries(binaryNames).map(([a,n])=>`\tsource_${a} = ${repo}/releases/download/v${version}/${n}\n\tsha256sums_${a} = ${binarySums[a]}\n`).join('')}\npkgname = appimage-mannger-bin\n`;
for (const [name, pkg, info] of [["appimage-mannger", sourcePkg,sourceInfo],["appimage-mannger-bin",binaryPkg,binaryInfo]]) {
  const dir = join(recipes,name); mkdirSync(dir,{recursive:true});
  writeFileSync(join(dir,"PKGBUILD"),pkg); writeFileSync(join(dir,".SRCINFO"),info);
  run(["bash", "-n", join(dir,"PKGBUILD")]);
  run(["tar", "-czf", join(out, `${name}-${version}-aur.tar.gz`), "-C",recipes,name]);
}
console.log(`Generated source archive and checksummed AUR source/binary recipes in ${out}`);
