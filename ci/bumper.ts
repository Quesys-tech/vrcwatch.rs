#!/usr/bin/env -S deno run --allow-read --allow-write
// Usage: deno --allow-read --allow-write ci/bumper.ts vX.Y.Z -o Cargo.toml

const verArg = Deno.args[0] ?? "";
const outIdx = Deno.args.indexOf("-o");
const path = outIdx >= 0 ? Deno.args[outIdx + 1] : "";

if (
  !path ||
  !/^v?\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/.test(verArg)
) {
  console.error("Usage: deno --allow-read --allow-write ci/bumper.ts vX.Y.Z -o Cargo.toml");
  Deno.exit(2);
}

const version = verArg.startsWith("v") ? verArg.slice(1) : verArg;

const s = await Deno.readTextFile(path);
const lines = s.split(/\r?\n/);

let inPackage = false;
let changed = false;

for (let i = 0; i < lines.length; i++) {
  const sec = lines[i].match(/^\s*\[([^\]]+)\]\s*$/);
  if (sec) {
    if (inPackage) break; // [package] を抜けた
    inPackage = sec[1].trim() === "package";
    continue;
  }
  if (!inPackage) continue;

  const m = lines[i].match(/^(\s*version\s*=\s*)(["'])([^"']*)(\2)(.*)$/);
  if (!m) continue;

  lines[i] = `${m[1]}${m[2]}${version}${m[2]}${m[5] ?? ""}`;
  changed = true;
  break;
}

if (!changed) {
  console.error(`No [package].version found in ${path}`);
  Deno.exit(1);
}

await Deno.writeTextFile(path, lines.join("\n"));
console.log(`Bumped ${path} [package].version -> ${version}`);
