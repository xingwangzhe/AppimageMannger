#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
const dir = resolve(process.argv[2] || "dist");
const files = readdirSync(dir,{withFileTypes:true}).filter(e=>e.isFile()&&e.name!=="SHA256SUMS").map(e=>e.name).sort();
writeFileSync(join(dir,"SHA256SUMS"),files.map(name=>`${createHash("sha256").update(readFileSync(join(dir,name))).digest("hex")}  ${name}`).join("\n")+"\n");
console.log(`SHA256SUMS: ${files.length} artifacts`);
