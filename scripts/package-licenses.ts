#!/usr/bin/env bun
/** Retain project/Rust notices and exact Ubuntu copyrights for redistributed files. */
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, realpathSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";

function run(command: string[], cwd: string): string {
  const p = Bun.spawnSync(command, { cwd, stdout: "pipe", stderr: "pipe" });
  if (p.exitCode !== 0) throw new Error(`${command.join(" ")}\n${p.stderr.toString()}`);
  return p.stdout.toString();
}
function put(source: string, destination: string) { mkdirSync(dirname(destination), { recursive: true }); copyFileSync(source, destination); }
const safeName = (value: string) => value.replace(/[^a-zA-Z0-9_.+-]/g, "_");
export function collectProjectLicenses(destination: string, root = resolve(import.meta.dir, "..")) {
  mkdirSync(destination, { recursive: true });
  put(join(root, "LICENSE"), join(destination, "LICENSE"));
  put(join(root, "Cargo.lock"), join(destination, "Cargo.lock"));
  put(join(root, "packaging/licenses/ELY-MIT.txt"), join(destination, "ELY-MIT.txt"));
  put(join(root, "docs/THIRD-PARTY-NOTICES.md"), join(destination, "THIRD-PARTY-NOTICES.md"));
  const target = run(["rustc", "-vV"], root).match(/^host: (.+)$/m)?.[1];
  if (!target) throw new Error("Cannot determine Rust host target for license inventory");
  const metadata = JSON.parse(run(["cargo", "metadata", "--locked", "--offline", "--filter-platform", target, "--format-version", "1"], root));
  const inventory = [];
  for (const pkg of metadata.packages.filter((p: any) => p.source)) {
    const crateDir = dirname(pkg.manifest_path);
    const roots = [crateDir];
    if (pkg.source.startsWith("git+")) {
      for (let parent = dirname(crateDir), count = 0; count < 5 && parent !== dirname(parent); parent = dirname(parent), count++) {
        roots.push(parent); if (existsSync(join(parent, ".git"))) break;
      }
    }
    const candidates = new Set<string>();
    const declaredLicense = pkg.license_file && resolve(crateDir, pkg.license_file);
    if (declaredLicense && existsSync(declaredLicense)) candidates.add(declaredLicense);
    for (const folder of roots) {
      for (const item of readdirSync(folder, { withFileTypes: true })) {
        if (/^(licen[sc]e|copying|copyright|notice)([.-]|$)/i.test(item.name) && item.isFile()) candidates.add(join(folder, item.name));
        if (/^licenses$/i.test(item.name) && item.isDirectory()) {
          for (const child of readdirSync(join(folder, item.name), { withFileTypes: true })) if (child.isFile()) candidates.add(join(folder, item.name, child.name));
        }
      }
      if (candidates.size) break;
    }
    const files: string[] = [];
    for (const file of candidates) {
      if (statSync(file).size > 2 * 1024 * 1024) throw new Error(`Unexpectedly large license document: ${file}`);
      const label = safeName(relative(crateDir, file));
      const dest = join("rust", safeName(`${pkg.name}-${pkg.version}`), label);
      put(file, join(destination, dest)); files.push(dest);
    }
    inventory.push({ name: pkg.name, version: pkg.version, license: pkg.license, source: pkg.source, repository: pkg.repository, retainedDocuments: files, authors: pkg.authors });
  }
  writeFileSync(join(destination, "rust-dependencies.json"), JSON.stringify({ target, note: "Conservative target-resolved Cargo inventory, including build/procedural dependencies. Original licenses are retained verbatim; SPDX terms without a bundled document remain visible for review.", packages: inventory }, null, 2) + "\n");
  return inventory.length;
}

export function systemLicenseCollector(destination: string, cwd: string) {
  const packageRecords = new Map<string, any>();
  const sourceRecords = new Map<string, any>();
  const osRelease = readFileSync("/etc/os-release", "utf8");
  mkdirSync(destination, { recursive: true });
  const commonDir = "/usr/share/common-licenses";
  for (const name of readdirSync(commonDir)) if (statSync(join(commonDir, name)).isFile()) put(join(commonDir, name), join(destination, "common-licenses", name));
  function recordPackage(packageName: string) {
    if (packageRecords.has(packageName)) return;
    const format = "${binary:Package}\t${Version}\t${Architecture}\t${source:Package}\t${source:Version}\t${Homepage}\n";
    const [binaryPackage, version, architecture, sourcePackage, sourceVersion, homepage] = run(["dpkg-query", "-W", "-f=" + format, packageName], cwd).trimEnd().split("\t");
    const doc = join("/usr/share/doc", binaryPackage.split(":")[0], "copyright");
    if (!existsSync(doc)) throw new Error(`Cannot redistribute ${binaryPackage}: its distribution copyright file is absent (${doc})`);
    const rel = join("ubuntu", safeName(binaryPackage), "copyright");
    put(doc, join(destination, rel));
    const copyright = readFileSync(doc, "utf8");
    packageRecords.set(packageName, { binaryPackage, version, architecture, sourcePackage: sourcePackage || binaryPackage.split(":")[0], sourceVersion: sourceVersion || version, homepage: homepage || null, upstreamSource: copyright.match(/^Source:\s*(.+)$/m)?.[1] || null, copyrightFile: rel, copyrightSource: realpathSync(doc), sourceRetrieval: `apt-get source ${sourcePackage || binaryPackage.split(":")[0]}=${sourceVersion || version}` });
  }
  function ownerPackages(path: string, tree: boolean): string[] {
    const candidates = tree ? [...new Set([path.replace(/\/$/, "") + "/*", realpathSync(path).replace(/\/$/, "") + "/*", path])] : [...new Set([path, realpathSync(path), path.replace(/^\/lib\//, "/usr/lib/")])];
    for (const candidate of candidates) {
      const p = Bun.spawnSync(["dpkg-query", "-S", candidate], { cwd, stdout: "pipe", stderr: "pipe" });
      if (p.exitCode !== 0) continue;
      const owners = p.stdout.toString().split("\n").flatMap(line => { const split=line.indexOf(": "); return split < 0 ? [] : line.slice(0, split).split(", "); }).filter(name => !name.includes("diversion") && /^[a-z0-9][a-z0-9+.:_-]*$/.test(name));
      if (owners.length) return [...new Set(owners)];
    }
    throw new Error(`Cannot identify Ubuntu source package for redistributed file/tree: ${path}`);
  }
  function track(path: string, installedAs: string, tree = false) {
    if (sourceRecords.has(path)) return;
    const owners = ownerPackages(path, tree);
    for (const name of owners) recordPackage(name);
    sourceRecords.set(path, { sourcePath: path, installedAs, kind: tree ? "asset-tree" : "file", binaryPackages: owners });
  }
  function finish() {
    const result = { distribution: osRelease, note: "Exact Ubuntu binary/source versions and retained distribution copyrights for files redistributed inside this AppDir. License texts referenced by Debian copyright files are copied under common-licenses; unmodified upstream terms continue to apply. Source retrieval requires the matching Ubuntu archive/deb-src configuration and is recorded for traceability, not as a blanket compliance guarantee.", packages: [...packageRecords.values()].sort((a,b)=>a.binaryPackage.localeCompare(b.binaryPackage)), files: [...sourceRecords.values()].sort((a,b)=>a.sourcePath.localeCompare(b.sourcePath)) };
    writeFileSync(join(destination, "bundled-system-libraries.json"), JSON.stringify(result, null, 2) + "\n");
    writeFileSync(join(destination, "SYSTEM-LICENSES.txt"), "The ubuntu/ directory retains the original distribution copyright file for every recorded package. Full GPL, LGPL, Apache and other shared texts referenced through /usr/share/common-licenses are included in common-licenses/. Replacing a library's license with the project's AGPL license is not intended. See bundled-system-libraries.json for exact binary and source package versions.\n");
    return { packages: packageRecords.size, files: sourceRecords.size };
  }
  return { track, finish };
}
if (import.meta.main) {
  const index = process.argv.indexOf("--destination");
  if (index < 0 || !process.argv[index + 1]) throw new Error("Usage: bun scripts/package-licenses.ts --destination DIR");
  console.log(`Retained project and ${collectProjectLicenses(resolve(process.argv[index + 1]))} Rust dependency license records`);
}
