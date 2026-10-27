#!/usr/bin/env bun
/** Native Linux packages from the same release binary. Run with Bun, never as root. */
import { chmodSync, copyFileSync, cpSync, existsSync, mkdirSync, readdirSync, readFileSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve, sep } from "node:path";
import { archPackageVersion } from "./packaging-common";
import { collectProjectLicenses, systemLicenseCollector } from "./package-licenses";
import { createHash } from "node:crypto";

const root = resolve(import.meta.dir, "..");
process.chdir(root);
const args = process.argv.slice(2);
const option = (name: string, fallback: string) => { const i = args.indexOf(name); return i < 0 ? fallback : args[i + 1] ?? fallback; };
const formats = option("--formats", "deb").split(",");
const manifest = readFileSync("Cargo.toml", "utf8");
const version = manifest.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
if (!version) throw new Error("Cargo.toml package version missing");
const projectLicense = manifest.match(/^license\s*=\s*"([^"]+)"/m)?.[1];
if (projectLicense !== "AGPL-3.0-only") throw new Error("Package metadata expects the project AGPL-3.0-only license");
const binary = resolve(option("--binary", "target/release/appimage-mannger"));
if (!existsSync(binary)) throw new Error(`Missing ${binary}; run cargo build --release --locked first`);
if (run([binary, "--version"]).trim() !== `AppimageMannger ${version}`) throw new Error("Binary version does not match Cargo.toml; rebuild before packaging");
const uname = run(["uname", "-m"]).trim();
const arch = option("--arch", uname);
if (!["x86_64", "aarch64"].includes(arch)) throw new Error(`Unsupported architecture ${arch}`);
if (uname !== arch) throw new Error("Package on a native runner for the requested architecture");
const archVersion = archPackageVersion(version);
const packageVersion = version.replace(/-/g, "~");
const debArch = arch === "x86_64" ? "amd64" : "arm64";
const out = resolve(option("--out", "dist"));
const work = resolve(option("--work-dir", join("work", "packaging", arch)));
if (!work.startsWith(join(root, "work") + sep)) throw new Error("Packaging work directory must be a subdirectory of this repository's work/ folder");
if (binary.startsWith(work + sep)) throw new Error("Keep the input binary outside the packaging scratch directory");
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
run([binary,"--export-frontends",join(stage,"usr/lib/appimage-mannger")]);
const frontendElfs = ["frontend-qt", "frontend-gnome"].map(name => join(stage, "usr/lib/appimage-mannger", name));
for (const file of frontendElfs) {
  if (!existsSync(file) || statSync(file).size < 1024) throw new Error(`Native frontend was not exported: ${file}`);
  chmodSync(file, 0o755);
}
const applicationElfs = [binary, ...frontendElfs];
install("packaging/appimage-mannger.desktop", join(stage, "usr/share/applications/appimage-mannger.desktop"));
install("packaging/appimage-mannger.svg", join(stage, "usr/share/icons/hicolor/scalable/apps/appimage-mannger.svg"));
install("packaging/appimage-mannger.png", join(stage, "usr/share/icons/hicolor/256x256/apps/appimage-mannger.png"));
install("packaging/appimage-mannger.xml", join(stage, "usr/share/mime/packages/appimage-mannger.xml"));
text(join(stage, "usr/share/applications/io.github.xingwangzhe.AppimageMannger.desktop"), readFileSync("packaging/appimage-mannger.desktop", "utf8").split("\n").filter(line => !line.startsWith("MimeType=")).join("\n") + "\nNoDisplay=true\n");
collectProjectLicenses(join(stage, "usr/share/licenses/appimage-mannger"), root);
install("docs/THIRD-PARTY-NOTICES.md", join(stage, "usr/share/doc/appimage-mannger/THIRD-PARTY-NOTICES.md"));
text(join(stage, "usr/share/doc/appimage-mannger/copyright"), `AppimageMannger ${version}\nCopyright 2026 xingwangzhe\nProject license: ${projectLicense}\nDependency notices are retained under /usr/share/licenses/appimage-mannger/.\n\n${readFileSync("LICENSE", "utf8")}`);
if (existsSync("README.md")) install("README.md", join(stage, "usr/share/doc/appimage-mannger/README.md"));
if (existsSync("docs/i18n")) cpSync("docs/i18n", join(stage, "usr/share/doc/appimage-mannger/i18n"), { recursive: true });
const name = `AppimageMannger-${version}-${arch}`;
const made: string[] = [];
if (formats.includes("deb")) {
  // dpkg-shlibdeps computes symbol-versioned glibc/other runtime requirements for THIS build.
  text(join(work, "debian/control"), `Source: appimage-mannger\nSection: utils\nPriority: optional\nMaintainer: xingwangzhe <xingwangzhe@outlook.com>\nStandards-Version: 4.7.2\n\nPackage: appimage-mannger\nArchitecture: any\nDescription: Native AppImage manager\n`);
  const deps = run(["dpkg-shlibdeps", "--ignore-missing-info", "-O", "-e" + binary, "-e"+join(stage,"usr/lib/appimage-mannger/frontend-qt"), "-e"+join(stage,"usr/lib/appimage-mannger/frontend-gnome")], work).trim().replace(/^shlibs:Depends=/, "");
  text(join(stage, "DEBIAN/control"), `Package: appimage-mannger\nVersion: ${packageVersion}\nSection: utils\nPriority: optional\nX-License: AGPL-3.0-only\nArchitecture: ${debArch}\nMaintainer: xingwangzhe <xingwangzhe@outlook.com>\nHomepage: https://github.com/xingwangzhe/AppimageMannger\nDepends: ${deps || "libc6, libfontconfig1, libxkbcommon0, libxkbcommon-x11-0"}, libvulkan1, squashfs-tools, xdg-utils, wl-clipboard, xclip, qt6-qpa-plugins, qt6-wayland\nDescription: AppimageMannger native Linux AppImage manager\n Manage AppImage applications, desktop integration, updates, and installation folders.\n Includes native Qt Widgets and GTK4/libadwaita frontends and the Ely/GPUI interface.\n`);
  // The package manager's normal triggers update icon/desktop caches; no privileges at runtime.
  text(join(stage, "DEBIAN/triggers"), "interest-noawait /usr/share/applications\ninterest-noawait /usr/share/icons/hicolor\ninterest-noawait /usr/share/mime\n");
  text(join(stage, "DEBIAN/postinst"), "#!/bin/sh\nset -e\nif command -v update-desktop-database >/dev/null 2>&1; then update-desktop-database /usr/share/applications || true; fi\nif command -v update-mime-database >/dev/null 2>&1; then update-mime-database /usr/share/mime || true; fi\nif command -v gtk-update-icon-cache >/dev/null 2>&1; then gtk-update-icon-cache -q -t /usr/share/icons/hicolor || true; fi\n", 0o755);
  const file = join(out, `appimage-mannger_${packageVersion}_${debArch}.deb`);
  run(["dpkg-deb", "--root-owner-group", "--build", stage, file]); made.push(file);
  rmSync(join(stage, "DEBIAN"), { recursive: true });
}
if (formats.includes("rpm")) {
  const top = join(work, "rpmbuild");
  for (const dir of ["BUILD", "BUILDROOT", "RPMS", "SOURCES", "SPECS", "SRPMS"]) mkdirSync(join(top, dir), { recursive: true });
  const spec = join(top, "SPECS/appimage-mannger.spec");
  text(spec, `Name: appimage-mannger\nVersion: ${packageVersion}\nRelease: 1\nSummary: Native Linux AppImage manager built with Ely and GPUI\nLicense: AGPL-3.0-only\nURL: https://github.com/xingwangzhe/AppimageMannger\nBuildArch: ${arch}\nRequires: squashfs-tools\nRequires: xdg-utils\nRequires: wl-clipboard\nRequires: xclip\nRequires: libQt6WaylandClient.so.6()(64bit)\nRequires: libvulkan.so.1()(64bit)\n%global __os_install_post %{nil}\n%global debug_package %{nil}\n\n%description\nManage AppImage applications, desktop integration, updates, and installation folders.\n\n%install\nmkdir -p %{buildroot}\ncp -a "${stage}/usr" %{buildroot}/\n\n%files\n/usr/bin/appimage-mannger\n/usr/lib/appimage-mannger/\n/usr/share/applications/appimage-mannger.desktop\n/usr/share/applications/io.github.xingwangzhe.AppimageMannger.desktop\n/usr/share/mime/packages/appimage-mannger.xml\n/usr/share/icons/hicolor/scalable/apps/appimage-mannger.svg\n/usr/share/icons/hicolor/256x256/apps/appimage-mannger.png\n%license /usr/share/licenses/appimage-mannger/\n${existsSync("README.md") ? "%doc /usr/share/doc/appimage-mannger/\n" : ""}`);
  run(["rpmbuild", "--define", `_topdir ${top}`, "-bb", spec]);
  const file = join(out, `appimage-mannger-${packageVersion}-1.${arch}.rpm`);
  install(join(top, `RPMS/${arch}/appimage-mannger-${packageVersion}-1.${arch}.rpm`), file); made.push(file);
}
if (formats.includes("tar")) {
  const archiveRoot = join(work, name); mkdirSync(archiveRoot, { recursive: true });
  cpSync(join(stage, "usr"), join(archiveRoot, "usr"), { recursive: true });
  text(join(archiveRoot, "install.sh"), '#!/bin/sh\nset -eu\nprefix="${PREFIX:-$HOME/.local}"\ncd "$(dirname "$0")"\nmkdir -p "$prefix"\ncp -R usr/. "$prefix/"\nif command -v update-desktop-database >/dev/null 2>&1; then update-desktop-database "$prefix/share/applications" || true; fi\nif command -v update-mime-database >/dev/null 2>&1; then update-mime-database "$prefix/share/mime" || true; fi\nprintf "Installed in %s; ensure %s/bin is in PATH.\\n" "$prefix" "$prefix"\n', 0o755);
  const file = join(out, name + ".tar.gz"); run(["tar", "-czf", file, "-C", work, name]); made.push(file);
}
if (formats.includes("arch")) {
  const packageRoot = join(work, "arch-root"); cpSync(stage, packageRoot, { recursive: true });
  const builddate = Number(process.env.SOURCE_DATE_EPOCH || Math.floor(Date.now() / 1000));
  const symbolVersions = applicationElfs.flatMap(file => [...run(["readelf", "--version-info", file]).matchAll(/\bGLIBC_(\d+(?:\.\d+)+)\b/g)].map(m => m[1]));
  const compareVersions = (a: string, b: string) => { const aa=a.split(".").map(Number), bb=b.split(".").map(Number); for (let i=0;i<Math.max(aa.length,bb.length);i++) { const delta=(aa[i]||0)-(bb[i]||0); if(delta) return delta; } return 0; };
  const glibc = symbolVersions.sort(compareVersions).at(-1);
  if (!glibc) throw new Error("Cannot determine ELF glibc symbol requirements");
  const dependencies = [`glibc>=${glibc}`, "fontconfig", "libxkbcommon", "libxkbcommon-x11", "libxcb", "libx11", "wayland", "vulkan-icd-loader", "gcc-libs", "qt6-base", "qt6-wayland", "gtk4", "libadwaita", "json-glib", "squashfs-tools", "xdg-utils", "wl-clipboard", "xclip"];
  const makeDependencies = ["rust", "cargo", "clang", "pkgconf", "cmake"];
  const size = Number(run(["du", "--bytes", "--summarize", packageRoot]).split(/\s/)[0]);
  const recipe = `# Native prebuilt ELF assembly recipe; release source: ${version}\npkgname=appimage-mannger\npkgver=${archVersion}\npkgrel=1\narch=('${arch}')\nlicense=('AGPL-3.0-only')\npackage() { cp -a prebuilt/usr "$pkgdir/"; }\n`;
  text(join(work, "PKGBUILD"), recipe);
  const recipeHash = createHash("sha256").update(recipe).digest("hex");
  text(join(packageRoot, ".PKGINFO"), `pkgname = appimage-mannger\npkgbase = appimage-mannger\nxdata = pkgtype=pkg\npkgver = ${archVersion}-1\npkgdesc = Native Linux AppImage manager built with Ely and GPUI\nurl = https://github.com/xingwangzhe/AppimageMannger\nbuilddate = ${builddate}\npackager = xingwangzhe <xingwangzhe@outlook.com>\nsize = ${size}\narch = ${arch}\nlicense = AGPL-3.0-only\n${dependencies.map(d=>`depend = ${d}\n`).join("")}${makeDependencies.map(d=>`makedepend = ${d}\n`).join("")}`);
  // This is an Ubuntu native ELF build, not an Arch makepkg build. Record that honestly.
  text(join(packageRoot, ".BUILDINFO"), `format = 2\npkgname = appimage-mannger\npkgbase = appimage-mannger\npkgver = ${archVersion}-1\npkgarch = ${arch}\npkgbuild_sha256sum = ${recipeHash}\npackager = xingwangzhe <xingwangzhe@outlook.com>\nbuilddate = ${builddate}\nbuilddir = ${work}\nstartdir = ${root}\nbuildtool = appimage-mannger-bun-native\nbuildtoolver = ${version}-${arch}\nbuildenv = !distcc\nbuildenv = !ccache\noptions = !strip\noptions = !debug\n`);
  // BSD mtree stores checksums and ownership; standard pacman can parse/verify this archive.
  const mtree = run(["bsdtar", "--format=mtree", "--options=!all,use-set,type,uid,gid,mode,time,size,md5,sha256,link", "--uid", "0", "--gid", "0", "-cf", "-", ".PKGINFO", ".BUILDINFO", "usr"], packageRoot);
  const compressed = Bun.spawnSync(["gzip", "-n", "-c"], { stdin: new TextEncoder().encode(mtree), stdout: "pipe", stderr: "pipe" });
  if (compressed.exitCode) throw new Error(compressed.stderr.toString()); writeFileSync(join(packageRoot, ".MTREE"), compressed.stdout);
  const file = join(out, `appimage-mannger-${archVersion}-1-${arch}.pkg.tar.zst`);
  run(["bsdtar", "--zstd", "--uid", "0", "--gid", "0", "-cf", file, ".PKGINFO", ".BUILDINFO", ".MTREE", "usr"], packageRoot); made.push(file);
}
if (formats.includes("appimage") || formats.includes("appdir")) {
  const appdir = join(work, "AppimageMannger.AppDir"); cpSync(stage, appdir, { recursive: true });
  install("packaging/appimage-mannger.desktop", join(appdir, "appimage-mannger.desktop"));
  install("packaging/appimage-mannger.svg", join(appdir, "appimage-mannger.svg"));
  install("packaging/appimage-mannger.svg", join(appdir, ".DirIcon"));
  // ldd returns the complete transitive NEEDED graph. Keep the host's libc and GPU drivers.
  const excluded = /^(ld-linux|ld64|lib(c|m|dl|pthread|rt|resolv|util|nss_.*|anl)\.so|lib(GLX_mesa|EGL_mesa|nvidia|vulkan_(intel|radeon|lvp))).*/;
  const libraryNames = new Set<string>();
  const systemLicenses = systemLicenseCollector(join(appdir, "usr/share/licenses/appimage-mannger"), root);
  function bundleElf(file: string) {
    for (const line of run(["ldd", file]).split("\n")) {
      if (line.includes("not found")) throw new Error(`Cannot bundle missing library for ${file}: ${line}`);
      const match = line.trim().match(/^(\S+) => (\/\S+) /); if (!match || excluded.test(match[1]) || libraryNames.has(match[1])) continue;
      install(realpathSync(match[2]), join(appdir, "usr/lib", match[1])); libraryNames.add(match[1]);
      systemLicenses.track(match[2], join("usr/lib", match[1]));
    }
  }
  function filesIn(folder: string): string[] {
    if (!existsSync(folder)) return [];
    return readdirSync(folder, { withFileTypes: true }).flatMap(item => item.isDirectory() ? filesIn(join(folder, item.name)) : [join(folder, item.name)]);
  }
  function bundlePlugin(source: string, destination: string) { install(source, destination); systemLicenses.track(source, destination.slice(appdir.length + 1)); bundleElf(source); }
  for (const file of applicationElfs) bundleElf(file);
  const qtPlugins = run(["qtpaths6", "--query", "QT_INSTALL_PLUGINS"]).trim();
  for (const category of ["platforms", "wayland-shell-integration", "wayland-graphics-integration-client", "xcbglintegrations", "imageformats", "iconengines", "platformthemes", "styles", "generic"]) {
    for (const plugin of filesIn(join(qtPlugins, category)).filter(p => p.endsWith(".so"))) {
      bundlePlugin(plugin, join(appdir, "usr/plugins", category, basename(plugin)));
    }
  }
  if (!existsSync(join(appdir, "usr/plugins/platforms/libqxcb.so")) || !filesIn(join(appdir, "usr/plugins/platforms")).some(p => basename(p).startsWith("libqwayland"))) {
    throw new Error("Qt XCB/Wayland platform plugins are missing; install qt6-qpa-plugins and qt6-wayland");
  }
  text(join(appdir, "usr/lib/appimage-mannger/qt.conf"), "[Paths]\nPrefix=../..\nLibraries=lib\nPlugins=plugins\n");
  const nativeLibdir = run(["pkg-config", "--variable=libdir", "gio-2.0"]).trim();
  const vulkanLoader = join(nativeLibdir, "libvulkan.so.1");
  if (!existsSync(vulkanLoader)) throw new Error("The Vulkan loader libvulkan.so.1 is required for the GPUI frontend");
  bundlePlugin(vulkanLoader, join(appdir, "usr/lib/libvulkan.so.1")); libraryNames.add("libvulkan.so.1");
  // Dconf/GIO modules and image loaders are dlopen dependencies, absent from the main ldd graph.
  for (const module of ["libdconfsettings.so"]) {
    const source = join(nativeLibdir, "gio/modules", module);
    if (existsSync(source)) bundlePlugin(source, join(appdir, "usr/lib/gio/modules", module));
  }
  if (existsSync(join(appdir, "usr/lib/gio/modules"))) run(["gio-querymodules", join(appdir, "usr/lib/gio/modules")]);
  const pixbufDir = run(["pkg-config", "--variable=gdk_pixbuf_moduledir", "gdk-pixbuf-2.0"]).trim();
  for (const plugin of filesIn(pixbufDir).filter(p => p.endsWith(".so"))) bundlePlugin(plugin, join(appdir, "usr/lib/gdk-pixbuf-2.0/loaders", basename(plugin)));
  const queryLoaders = run(["pkg-config", "--variable=gdk_pixbuf_query_loaders", "gdk-pixbuf-2.0"]).trim();
  install(queryLoaders, join(appdir, "usr/bin/gdk-pixbuf-query-loaders"), 0o755); systemLicenses.track(queryLoaders, "usr/bin/gdk-pixbuf-query-loaders"); bundleElf(queryLoaders);
  const schemaDir = join(appdir, "usr/share/glib-2.0/schemas"); mkdirSync(schemaDir, { recursive: true });
  for (const file of readdirSync("/usr/share/glib-2.0/schemas").filter(name => /^(org\.gtk\.|org\.gnome\.desktop\.)/.test(name) && name.endsWith(".xml"))) { const source = join("/usr/share/glib-2.0/schemas", file); install(source, join(schemaDir, file)); systemLicenses.track(source, join("usr/share/glib-2.0/schemas", file)); }
  run(["glib-compile-schemas", "--strict", schemaDir]);
  if (existsSync("/usr/share/icons/Adwaita")) { cpSync("/usr/share/icons/Adwaita", join(appdir, "usr/share/icons/Adwaita"), { recursive: true, verbatimSymlinks: true }); systemLicenses.track("/usr/share/icons/Adwaita", "usr/share/icons/Adwaita", true); }
  if (existsSync("/usr/share/icons/hicolor/index.theme")) { install("/usr/share/icons/hicolor/index.theme", join(appdir, "usr/share/icons/hicolor/index.theme")); systemLicenses.track("/usr/share/icons/hicolor/index.theme", "usr/share/icons/hicolor/index.theme"); }
  const mimeSource = "/usr/share/mime/packages/freedesktop.org.xml";
  if (existsSync(mimeSource)) { install(mimeSource, join(appdir, "usr/share/mime/packages/freedesktop.org.xml")); systemLicenses.track(mimeSource, "usr/share/mime/packages/freedesktop.org.xml"); run(["update-mime-database", join(appdir, "usr/share/mime")]); }
  for (const tool of ["wl-copy", "xclip"]) { const source = run(["which", tool]).trim(); install(source, join(appdir, "usr/bin", tool), 0o755); systemLicenses.track(source, "usr/bin/" + tool); bundleElf(source); }
  const unsquashfs = run(["which", "unsquashfs"]).trim(); install(unsquashfs, join(appdir, "usr/bin/unsquashfs"), 0o755); systemLicenses.track(unsquashfs, "usr/bin/unsquashfs"); bundleElf(unsquashfs);
  if (existsSync("/usr/share/X11/xkb")) { cpSync(realpathSync("/usr/share/X11/xkb"), join(appdir, "usr/share/X11/xkb"), { recursive: true, verbatimSymlinks: true }); systemLicenses.track("/usr/share/X11/xkb", "usr/share/X11/xkb", true); }
  const retainedSystemLicenses = systemLicenses.finish();
  text(join(appdir, "AppRun"), '#!/bin/sh\nset -eu\nAPPDIR="${APPDIR:-$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)}"\nexport LD_LIBRARY_PATH="$APPDIR/usr/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"\nexport PATH="$APPDIR/usr/bin:$PATH"\nexport QT_PLUGIN_PATH="$APPDIR/usr/plugins"\nexport QT_QPA_PLATFORM_PLUGIN_PATH="$APPDIR/usr/plugins/platforms"\nexport GSETTINGS_SCHEMA_DIR="$APPDIR/usr/share/glib-2.0/schemas"\nexport GIO_MODULE_DIR="$APPDIR/usr/lib/gio/modules"\nexport XDG_DATA_DIRS="$APPDIR/usr/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"\nif [ -d "$APPDIR/usr/share/X11/xkb" ]; then export XKB_CONFIG_ROOT="$APPDIR/usr/share/X11/xkb"; fi\nif [ -d "$APPDIR/usr/lib/gdk-pixbuf-2.0/loaders" ]; then\n  export GDK_PIXBUF_MODULEDIR="$APPDIR/usr/lib/gdk-pixbuf-2.0/loaders"\n  cache="${XDG_CACHE_HOME:-$HOME/.cache}/appimage-mannger"\n  mkdir -p "$cache"\n  GDK_PIXBUF_MODULE_FILE=$(mktemp "$cache/pixbuf-loaders.XXXXXX")\n  export GDK_PIXBUF_MODULE_FILE\n  "$APPDIR/usr/bin/gdk-pixbuf-query-loaders" "$GDK_PIXBUF_MODULEDIR"/*.so > "$GDK_PIXBUF_MODULE_FILE"\n  trap \x27rm -f "$GDK_PIXBUF_MODULE_FILE"\x27 EXIT HUP INT TERM\n  "$APPDIR/usr/bin/appimage-mannger" "$@"\nelse\n  exec "$APPDIR/usr/bin/appimage-mannger" "$@"\nfi\n', 0o755);
  const libs = [...libraryNames].sort();
  if (formats.includes("appdir") && !formats.includes("appimage")) made.push(appdir);
  if (formats.includes("appimage")) {
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
  text(join(out, name + ".build-info.json"), JSON.stringify({ version, projectLicense, architecture: arch, licenseInventory: "usr/share/licenses/appimage-mannger/bundled-system-libraries.json", retainedSystemLicenses, appimagetool: { release: release.tag_name, asset: asset.name, sha256: digest }, bundledLibraries: libs, host: run(["ldd", "--version"]).split("\n")[0], note: "Uses host libc, GPU drivers and fonts; latest-build glibc requirements apply." }, null, 2) + "\n");
  }
}
for (const file of made) console.log(file);
if (made.length === 0) throw new Error("No package formats requested (deb,rpm,appimage,tar,arch,appdir)");
