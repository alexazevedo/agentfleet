// Usage: node scripts/release.mjs <semver> [--dry-run]
import { readFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";

const args = process.argv.slice(2);
const dry = args.includes("--dry-run");
const ver = args.find((a) => !a.startsWith("--"));
if (!ver || !/^\d+\.\d+\.\d+$/.test(ver)) {
  console.error("usage: release.mjs <major.minor.patch> [--dry-run]");
  process.exit(1);
}
const sh = (cmd, a, cwd) =>
  execFileSync(cmd, a, { cwd, encoding: "utf8", stdio: ["ignore", "pipe", "inherit"] });

if (!dry && sh("git", ["status", "--porcelain"]).trim() !== "") {
  console.error("working tree is dirty; commit or stash first");
  process.exit(1);
}

const jsonEdit = (file, fn) => {
  const j = JSON.parse(readFileSync(file, "utf8"));
  fn(j);
  writeFileSync(file, JSON.stringify(j, null, 2) + "\n");
};
const edits = [
  ["package.json", (j) => (j.version = ver)],
  [
    "package-lock.json",
    (j) => {
      j.version = ver;
      if (j.packages?.[""]) j.packages[""].version = ver;
    },
  ],
];

if (dry) {
  for (const [f] of edits) console.log(`would set version ${ver} in ${f}`);
  console.log(`would set version ${ver} in src-tauri/tauri.conf.json`);
  console.log(`would set [package] version ${ver} in src-tauri/Cargo.toml`);
  console.log("would run: cargo update -p agentfleet --offline (src-tauri)");
  console.log(`would commit "chore: release v${ver}" and tag v${ver}`);
  process.exit(0);
}

for (const [f, fn] of edits) jsonEdit(f, fn);
// Text edit (not JSON.stringify) to keep tauri.conf.json formatting intact.
const tauriConf = "src-tauri/tauri.conf.json";
writeFileSync(
  tauriConf,
  readFileSync(tauriConf, "utf8").replace(/^(\s*"version": ")[^"]*"/m, `$1${ver}"`),
);
const cargo = "src-tauri/Cargo.toml";
const t = readFileSync(cargo, "utf8");
const t2 = t.replace(/^version = "[^"]*"/m, `version = "${ver}"`);
if (t2 === t && !t.includes(`version = "${ver}"`)) {
  console.error("could not find version line in Cargo.toml");
  process.exit(1);
}
writeFileSync(cargo, t2);
sh("cargo", ["update", "-p", "agentfleet", "--offline"], "src-tauri");
sh("git", ["add", "package.json", "package-lock.json", "src-tauri/tauri.conf.json", cargo, "src-tauri/Cargo.lock"]);
sh("git", ["commit", "-m", `chore: release v${ver}`]);
sh("git", ["tag", `v${ver}`]);
console.log(`released v${ver}. Push with:\n  git push --follow-tags origin main`);
