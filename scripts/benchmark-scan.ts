#!/usr/bin/env bun
/** Compare two release binaries using temporary libraries and desktop entries. */
import { mkdtempSync, mkdirSync, writeFileSync, rmSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const [baselineArg, optimizedArg, outputArg = "work/scan-benchmark.json"] = process.argv.slice(2);
if (!baselineArg || !optimizedArg) throw new Error("Usage: bun run scripts/benchmark-scan.ts BASELINE OPTIMIZED [OUTPUT]");
const binaries = [baselineArg, optimizedArg].map(path => resolve(path));
const hashes = binaries.map(path => new Bun.CryptoHasher("sha256").update(readFileSync(path)).digest("hex"));
const root = mkdtempSync(join(tmpdir(), "appimage-scan-bench-"));
const environment = {
  ...process.env,
  XDG_CONFIG_HOME: join(root, "config"),
  XDG_DATA_HOME: join(root, "data"),
  XDG_CACHE_HOME: join(root, "cache"),
  XDG_DATA_DIRS: join(root, "system-data"),
};
const desktops = join(environment.XDG_DATA_HOME, "applications");
mkdirSync(desktops, { recursive: true });
const fixture = Buffer.alloc(128);
fixture.set([0x7f, 0x45, 0x4c, 0x46, 2, 1]);
fixture.set([0x41, 0x49, 2], 8);
fixture.writeUInt16LE(62, 18);
const median = (values: number[]) => [...values].sort((a, b) => a - b)[Math.floor(values.length / 2)];
const results: unknown[] = [];
try {
  for (const count of [20, 100, 250]) {
    const library = join(root, `library-${count}`);
    mkdirSync(library);
    for (let i = 0; i < count; i++) {
      const name = `Tool-${i.toString().padStart(4, "0")}`;
      const path = join(library, `${name}-1.2.3.AppImage`);
      writeFileSync(path, fixture);
      writeFileSync(join(desktops, `${name}.desktop`), `[Desktop Entry]\nName=${name}\nExec="${path}" %F\nComment=Benchmark fixture\n`);
    }
    for (let i = 0; i < 500; i++) writeFileSync(join(desktops, `unrelated-${i}.desktop`), `[Desktop Entry]\nName=Unrelated ${i}\nExec=/usr/bin/unrelated-${i}\n`);
    let expected: string | undefined;
    const invoke = (binary: string) => {
      const start = performance.now();
      const run = Bun.spawnSync([binary, "--list", "--directory", library], { env: environment, stdout: "pipe", stderr: "pipe" });
      const milliseconds = performance.now() - start;
      if (run.exitCode !== 0) throw new Error(run.stderr.toString());
      const value = JSON.parse(run.stdout.toString());
      // Version intentionally differs after a release bump; compare app data.
      const apps = JSON.stringify(value.applications);
      if (value.applications.length !== count) throw new Error("Incorrect fixture count");
      if (expected !== undefined && apps !== expected) throw new Error("Scan output changed");
      expected = apps;
      return milliseconds;
    };
    for (const binary of binaries) invoke(binary);
    const samples: number[][] = [[], []];
    for (let sample = 0; sample < 7; sample++) {
      const order = sample % 2 === 0 ? [0, 1] : [1, 0];
      for (const index of order) samples[index].push(invoke(binaries[index]));
    }
    const oldMs = median(samples[0]), newMs = median(samples[1]);
    results.push({ applications: count, desktop_entries: count + 500, baseline_ms: samples[0], optimized_ms: samples[1], baseline_median_ms: oldMs, optimized_median_ms: newMs, speedup: oldMs / newMs });
    console.log(JSON.stringify(results.at(-1)));
    for (let i = 0; i < count; i++) rmSync(join(desktops, `Tool-${i.toString().padStart(4, "0")}.desktop`));
  }
  for (const [index, path] of binaries.entries()) if (new Bun.CryptoHasher("sha256").update(readFileSync(path)).digest("hex") !== hashes[index]) throw new Error("A binary changed during measurement; rebuild first, then rerun");
  const report = { measured_at: new Date().toISOString(), platform: `${process.platform}/${process.arch}`, binaries: binaries.map((path, index) => ({ path, sha256: hashes[index] })), samples: 7, fixtures: "128-byte inert ELF AppImages; 500 unrelated launchers; warm filesystem cache; whole CLI process elapsed time", results };
  writeFileSync(resolve(outputArg), JSON.stringify(report, null, 2) + "\n");
} finally {
  rmSync(root, { recursive: true, force: true });
}
