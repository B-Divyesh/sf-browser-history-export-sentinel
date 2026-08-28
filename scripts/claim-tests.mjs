import { execFileSync, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { chmodSync, mkdtempSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dirname, "..");
const binary = join(root, "target/debug/sentinel");

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function run(args, options = {}) {
  return spawnSync(binary, args, { cwd: root, encoding: "utf8", ...options });
}

function demoFixture() {
  const temp = mkdtempSync(join(tmpdir(), "sentinel-claim-"));
  const output = join(temp, "export");
  const home = join(temp, "real-home");
  const taskTemp = join(temp, "temporary");
  mkdirSync(home);
  mkdirSync(taskTemp);
  const marker = join(home, "must-not-change.txt");
  writeFileSync(marker, "real profile marker\n");
  const result = run(["--json", "demo", "--output", output], { env: { ...process.env, HOME: home, TMPDIR: taskTemp } });
  assert(result.status === 0, result.stderr || "demo failed");
  return { temp, output, home, taskTemp, marker, result: JSON.parse(result.stdout) };
}

function exportedFiles(output) {
  return readdirSync(output).flatMap((profile) => readdirSync(join(output, profile)).map((file) => join(output, profile, file)));
}

const tests = {
  "@claim:demo-isolation": () => {
    const fixture = demoFixture();
    assert(readFileSync(fixture.marker, "utf8") === "real profile marker\n", "real namespace changed");
    assert(readdirSync(fixture.taskTemp).length === 0, "sample workspace was not removed");
    assert(fixture.result.profile_count === 2 && fixture.result.record_count === 6, "sample result differs");
    rmSync(fixture.temp, { recursive: true });
  },
  "@claim:end-to-end-export": () => {
    const fixture = demoFixture();
    assert(fixture.result.status === "verified", "demo did not verify");
    assert(fixture.result.reports_checked === 2 && fixture.result.artifacts_checked === 4, "report totals differ");
    const files = exportedFiles(fixture.output);
    assert(files.filter((file) => file.endsWith("history.json")).length === 2, "missing JSON export");
    assert(files.filter((file) => file.endsWith("history.csv")).length === 2, "missing CSV export");
    for (const reportPath of files.filter((file) => file.endsWith("report.json"))) {
      const report = JSON.parse(readFileSync(reportPath, "utf8"));
      assert(report.record_count === 3 && report.earliest_visit && report.latest_visit, "report count or date range missing");
      assert(!JSON.stringify(report).includes(fixture.home) && !JSON.stringify(report).includes(fixture.temp), "report contains an absolute source path");
      for (const item of report.artifacts) {
        const bytes = readFileSync(join(resolve(reportPath, ".."), item.path));
        assert(createHash("sha256").update(bytes).digest("hex") === item.sha256, `bad hash for ${item.path}`);
      }
    }
    rmSync(fixture.temp, { recursive: true });
  },
  "@claim:sample-domains": () => {
    const fixture = demoFixture();
    for (const file of exportedFiles(fixture.output).filter((path) => path.endsWith("history.json"))) {
      for (const row of JSON.parse(readFileSync(file, "utf8"))) assert(new URL(row.url).hostname.endsWith(".example"), "sample contains a real domain");
    }
    rmSync(fixture.temp, { recursive: true });
  },
  "@claim:history-fields-only": () => {
    const fixture = demoFixture();
    for (const file of exportedFiles(fixture.output).filter((path) => path.endsWith("history.json"))) {
      for (const row of JSON.parse(readFileSync(file, "utf8"))) {
        assert(JSON.stringify(Object.keys(row).sort()) === JSON.stringify(["browser", "profile", "title", "url", "visit_count", "visited_at"]), "unexpected history field");
      }
    }
    rmSync(fixture.temp, { recursive: true });
  },
  "@claim:verification": () => {
    const fixture = demoFixture();
    const json = exportedFiles(fixture.output).find((file) => file.endsWith("history.json"));
    writeFileSync(json, "[]\n");
    const changed = run(["verify", fixture.output]);
    assert(changed.status === 13 && changed.stderr.includes(json), "changed file did not return code 13 with its path");
    rmSync(fixture.temp, { recursive: true });
  },
  "@claim:copy-safety": () => {
    execFileSync("cargo", ["test", "--manifest-path", "cli/Cargo.toml", "tests::claim_copy_safety", "--", "--exact"], { cwd: root, stdio: "pipe" });
  },
  "@claim:profile-discovery": () => {
    execFileSync("cargo", ["test", "--manifest-path", "cli/Cargo.toml", "tests::discovers_layouts_on_all_supported_operating_systems", "--", "--exact"], { cwd: root, stdio: "pipe" });
  },
  "@claim:actionable-errors": () => {
    execFileSync("cargo", ["test", "--manifest-path", "cli/Cargo.toml", "--test", "cli", "refuses_empty_and_malformed_databases_without_leaving_an_archive", "--", "--exact"], { cwd: root, stdio: "pipe" });
    if (process.getuid?.() === 0) {
      const temp = mkdtempSync(join(tmpdir(), "sentinel-permission-"));
      const home = join(temp, "home");
      mkdirSync(join(home, ".mozilla/firefox"), { recursive: true });
      chmodSync(temp, 0o755); chmodSync(home, 0o755); chmodSync(join(home, ".mozilla"), 0o755); chmodSync(join(home, ".mozilla/firefox"), 0o000);
      const denied = spawnSync("setpriv", ["--reuid=65534", "--regid=65534", "--clear-groups", binary, "scan", "--home", home], { encoding: "utf8" });
      chmodSync(join(home, ".mozilla/firefox"), 0o700);
      assert(denied.status === 11 && denied.stderr.includes("permissions"), "discovery permission error was hidden");
      rmSync(temp, { recursive: true });
    }
  },
  "@claim:cli-privacy": () => {
    const source = readFileSync(join(root, "cli/src/main.rs"), "utf8");
    const tree = execFileSync("cargo", ["tree", "--manifest-path", "cli/Cargo.toml", "--edges", "normal"], { cwd: root, encoding: "utf8" });
    assert(!/(reqwest|hyper|ureq|telemetry|analytics)/i.test(`${source}\n${tree}`), "network or telemetry code found");
    const fixture = demoFixture();
    assert(fixture.result.status === "verified", "offline demo failed");
    rmSync(fixture.temp, { recursive: true });
  },
  "@claim:no-overwrite": () => {
    const fixture = demoFixture();
    const marker = join(fixture.output, "keep.txt");
    writeFileSync(marker, "keep");
    const second = run(["demo", "--output", fixture.output]);
    assert(second.status === 1 && readFileSync(marker, "utf8") === "keep", "existing output was overwritten");
    rmSync(fixture.temp, { recursive: true });
  },
  "@claim:json-mode": () => {
    const fixture = demoFixture();
    assert(fixture.result.status === "verified" && typeof fixture.result.output === "string", "demo JSON is invalid");
    const verified = run(["--json", "verify", fixture.output]);
    assert(verified.status === 0 && JSON.parse(verified.stdout).status === "verified", "verify JSON is invalid");
    rmSync(fixture.temp, { recursive: true });
  },
  "@claim:license-version-package": () => {
    assert(readFileSync(join(root, "Cargo.toml"), "utf8").includes('license = "MIT"'), "MIT metadata missing");
    assert(readFileSync(join(root, "cli/Cargo.toml"), "utf8").includes('version = "0.1.0"'), "version differs");
    const files = execFileSync("cargo", ["package", "--manifest-path", "cli/Cargo.toml", "--locked", "--allow-dirty", "--list"], { cwd: root, encoding: "utf8" });
    assert(files.includes("LICENSE") && files.includes("CHANGELOG.md"), "package omits license or changelog");
  },
  "@claim:site-build": () => {
    for (const path of ["index.html", "demo/index.html", "privacy/index.html", "terms/index.html", "404.html", "sw.js"]) assert(statSync(join(root, "dist/site", path)).isFile(), `${path} missing from build`);
    assert(!readFileSync(join(root, "dist/site/sw.js"), "utf8").includes("staticwebapp.config.json"), "service worker precaches deployment config");
  }
};

const grepIndex = process.argv.indexOf("--grep");
const selected = grepIndex >= 0 ? process.argv[grepIndex + 1] : undefined;
const entries = Object.entries(tests).filter(([name]) => !selected || name.includes(selected));
assert(entries.length > 0, `no claim test matched ${selected}`);
for (const [name, test] of entries) {
  process.stdout.write(`${name} ... `);
  test();
  console.log("PASS");
}
