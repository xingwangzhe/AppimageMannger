#!/usr/bin/env bun
/** Native Linux packages from the same release binary. Run with Bun, never as root. */
import { chmodSync, copyFileSync, cpSync, existsSync, mkdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { createHash } from "node:crypto";

const root = resolve(import.meta.dir, "..");
process.chdir(root);
const args = process.argv.slice(2);
const option = (name: string, fallback: string) => { const i = args.indexOf(name); return i < 0 ? fallback : args[i + 1] ?? fallback; };
const formats = option("--formats", "deb").split(",");
const manifest = readFileSync("Cargo.toml", "utf8");
const version = manifest.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
if (!version) throw new Error("Cargo.toml package version missing");
const binary = resolve(option("--binary", "target/release/appimage-mannger"));
if (!existsSync(binary)) throw new Error(`Missing ${binary}; run cargo build --release --locked first`);
const uname = run(["uname", "-m"]).trim();
const arch = option("--arch", uname);
if (!["x86_64", "aarch64"].includes(arch)) throw new Error(`Unsupported architecture ${arch}`);
if (uname !== arch) throw new Error("Package on a native runner for the requested architecture");
const packageVersion = version.replace(/-/g, "~");
const debArch = arch === "x86_64" ? "amd64" : "arm64";
const out = resolve(option("--out", "dist"));
const work = resolve("work", "packaging", arch);
const stage = join(work, "root");
rmSync(work, { recursive: true, force: true });
mkdirSync(out, { recursive: true });
mkdirSync(stage, { recursive: true });
function run(command: string[], cwd = root, extraEnv: Record<string, string> = {}): string {
  const result = Bun.spawnSync(command, { cwd, env: { ...process.env, ...extraEnv }, stdout: "pipe", stderr: "pipe" });
  if (result.exitCode !== 0) throw new Error(`${command.join(" ")}\n${result.stdout.toString()}\n${result.stderr.toString()}`);
  return result.stdout.toString();
}
function install(source: string, destination: string, mode = 0o644) {
  mkdirSync(dirname(destination), { recursive: true }); copyFileSync(source, destination); chmodSync(destination, mode);
}
function text(path: string, content: string, mode = 0o644) {
  mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, content); chmodSync(path, mode);
}
install(binary, join(stage, "usr/bin/appimage-mannger"), 0o755);
install("packaging/appimage-mannger.desktop", join(stage, "usr/share/applications/appimage-mannger.desktop"));
install("packaging/appimage-mannger.svg", join(stage, "usr/share/icons/hicolor/scalable/apps/appimage-mannger.svg"));
install("LICENSE", join(stage, "usr/share/licenses/appimage-mannger/LICENSE"));
if (existsSync("README.md")) install("README.md", join(stage, "usr/share/doc/appimage-mannger/README.md"));
const name = `AppimageMannger-${version}-${arch}`;
const made: string[] = [];
if (formats.includes("deb")) {
  // dpkg-shlibdeps computes symbol-versioned glibc/other runtime requirements for THIS build.
  text(join(work, "debian/control"), `Source: appimage-mannger\nSection: utils\nPriority: optional\nMaintainer: xingwangzhe <xingwangzhe@outlook.com>\nStandards-Version: 4.7.2\n\nPackage: appimage-mannger\nArchitecture: any\nDescription: Native AppImage manager\n`);
  const deps = run(["dpkg-shlibdeps", "--ignore-missing-info", "-O", "-e" + binary], work).trim().replace(/^shlibs:Depends=/, "");
  text(join(stage, "DEBIAN/control"), `Package: appimage-mannger\nVersion: ${packageVersion}\nSection: utils\nPriority: optional\nArchitecture: ${debArch}\nMaintainer: xingwangzhe <xingwangzhe@outlook.com>\nHomepage: https://github.com/xingwangzhe/AppimageMannger\nDepends: ${deps || "libc6, libfontconfig1, libxkbcommon0, libxkbcommon-x11-0"}\nDescription: AppimageMannger native Linux AppImage manager\n Manage AppImage applications, desktop integration, updates, and installation folders.\n`);
  // The package manager's normal triggers update icon/desktop caches; no privileges at runtime.
  text(join(stage, "DEBIAN/triggers"), "interest-noawait /usr/share/applications\ninterest-noawait /usr/share/icons/hicolor\n");
  text(join(stage, "DEBIAN/postinst"), "#!/bin/sh\nset -e\nif command -v update-desktop-database >/dev/null 2>&1; then update-desktop-database /usr/share/applications || true; fi\nif command -v gtk-update-icon-cache >/dev/null 2>&1; then gtk-update-icon-cache -q -t /usr/share/icons/hicolor || true; fi\n", 0o755);
  const file = join(out, `appimage-mannger_${packageVersion}_${debArch}.deb`);
  run(["dpkg-deb", "--root-owner-group", "--build", stage, file]); made.push(file);
  rmSync(join(stage, "DEBIAN"), { recursive: true });
}
if (formats.includes("rpm")) {
  const top = join(work, "rpmbuild");
  for (const dir of ["BUILD", "BUILDROOT", "RPMS", "SOURCES", "SPECS", "SRPMS"]) mkdirSync(join(top, dir), { recursive: true });
  const spec = join(top, "SPECS/appimage-mannger.spec");
  text(spec, `Name: appimage-mannger\nVersion: ${packageVersion}\nRelease: 1\nSummary: Native Linux AppImage manager built with Ely and GPUI\nLicense: MIT\nURL: https://github.com/xingwangzhe/AppimageMannger\nBuildArch: ${arch}\n%global __os_install_post %{nil}\n%global debug_package %{nil}\n\n%description\nManage AppImage applications, desktop integration, updates, and installation folders.\n\n%install\nmkdir -p %{buildroot}\ncp -a "${stage}/usr" %{buildroot}/\n\n%files\n/usr/bin/appimage-mannger\n/usr/share/applications/appimage-mannger.desktop\n/usr/share/icons/hicolor/scalable/apps/appimage-mannger.svg\n%license /usr/share/licenses/appimage-mannger/LICENSE\n${existsSync("README.md") ? "%doc /usr/share/doc/appimage-mannger/README.md\n" : ""}`);
  run(["rpmbuild", "--define", `_topdir ${top}`, "-bb", spec]);
  const file = join(out, `appimage-mannger-${packageVersion}-1.${arch}.rpm`);
  install(join(top, `RPMS/${arch}/appimage-mannger-${packageVersion}-1.${arch}.rpm`), file); made.push(file);
}
if (formats.includes("tar")) {
  const archiveRoot = join(work, name); mkdirSync(archiveRoot, { recursive: true });
  cpSync(join(stage, "usr"), join(archiveRoot, "usr"), { recursive: true });
  text(join(archiveRoot, "install.sh"), '#!/bin/sh\nset -eu\nprefix="${PREFIX:-$HOME/.local}"\ncd "$(dirname "$0")"\nmkdir -p "$prefix"\ncp -R usr/. "$prefix/"\nif command -v update-desktop-database >/dev/null 2>&1; then update-desktop-database "$prefix/share/applications" || true; fi\nprintf "Installed in %s; ensure %s/bin is in PATH.\\n" "$prefix" "$prefix"\n', 0o755);
  const file = join(out, name + ".tar.gz"); run(["tar", "-czf", file, "-C", work, name]); made.push(file);
}
if (formats.includes("appimage")) {
  const appdir = join(work, "AppimageMannger.AppDir"); cpSync(stage, appdir, { recursive: true });
  install("packaging/appimage-mannger.desktop", join(appdir, "appimage-mannger.desktop"));
  install("packaging/appimage-mannger.svg", join(appdir, "appimage-mannger.svg"));
  install("packaging/appimage-mannger.svg", join(appdir, ".DirIcon"));
  // ldd returns the complete transitive NEEDED graph. Keep the host's libc and GPU drivers.
  const excluded = /^(ld-linux|ld64|lib(c|m|dl|pthread|rt|resolv|util|nss_.*|anl)\.so|lib(GLX_mesa|EGL_mesa|nvidia|vulkan_(intel|radeon|lvp))).*/;
  const libs: string[] = [];
  for (const line of run(["ldd", binary]).split("\n")) {
    if (line.includes("not found")) throw new Error(`Cannot bundle missing library: ${line}`);
    const match = line.trim().match(/^(\S+) => (\/\S+) /); if (!match || excluded.test(match[1])) continue;
    install(realpathSync(match[2]), join(appdir, "usr/lib", match[1])); libs.push(match[1]);
  }
  if (existsSync("/usr/share/X11/xkb")) cpSync("/usr/share/X11/xkb", join(appdir, "usr/share/X11/xkb"), { recursive: true });
  text(join(appdir, "AppRun"), '#!/bin/sh\nset -eu\nAPPDIR="${APPDIR:-$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)}"\nexport LD_LIBRARY_PATH="$APPDIR/usr/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"\nif [ -d "$APPDIR/usr/share/X11/xkb" ]; then export XKB_CONFIG_ROOT="$APPDIR/usr/share/X11/xkb"; fi\nexec "$APPDIR/usr/bin/appimage-mannger" "$@"\n', 0o755);
  // Resolve latest official appimagetool at build time and verify GitHub's SHA-256 asset digest.
  const token = process.env.GH_TOKEN || process.env.GITHUB_TOKEN;
  const headers: Record<string,string> = { Accept: "application/vnd.github+json", "User-Agent": "AppimageMannger-build" };
  if (token) headers.Authorization = `Bearer ${token}`;
  const response = await fetch("https://api.github.com/repos/AppImage/appimagetool/releases/latest", { headers });
  if (!response.ok) throw new Error(`appimagetool latest API: ${response.status}`);
  const release: any = await response.json();
  const asset = release.assets.find((a: any) => a.name === `appimagetool-${arch}.AppImage`);
  if (!asset?.digest?.startsWith("sha256:")) throw new Error("Official appimagetool asset has no SHA-256 digest");
  const download = await fetch(asset.browser_download_url); if (!download.ok) throw new Error(`appimagetool download: ${download.status}`);
  const bytes = new Uint8Array(await download.arrayBuffer());
  const digest = createHash("sha256").update(bytes).digest("hex");
  if (`sha256:${digest}` !== asset.digest) throw new Error("appimagetool digest mismatch");
  const tool = join(work, "appimagetool.AppImage"); writeFileSync(tool, bytes); chmodSync(tool, 0o755);
  const file = join(out, name + ".AppImage");
  run([tool, "--no-appstream", appdir, file], work, { APPIMAGE_EXTRACT_AND_RUN: "1", ARCH: arch, VERSION: version });
  chmodSync(file, 0o755); made.push(file);
  text(join(out, name + ".build-info.json"), JSON.stringify({ version, architecture: arch, appimagetool: { release: release.tag_name, asset: asset.name, sha256: digest }, bundledLibraries: libs, host: run(["ldd", "--version"]).split("\n")[0], note: "Uses host libc, GPU drivers and fonts; latest-build glibc requirements apply." }, null, 2) + "\n");
}
for (const file of made) console.log(file);
if (made.length === 0) throw new Error("No package formats requested (deb,rpm,appimage,tar)");
