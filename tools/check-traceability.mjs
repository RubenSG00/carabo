// Reports which acceptance criteria of the [1.0] requirements have a test (08 §13.3).
// Usage: node tools/check-traceability.mjs [--strict]   (npm run traceability)
// Without --strict it only reports and exits with code 0. With --strict it exits with code 1
// while any criterion is not covered.

import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const REQUIREMENTS_FILE = "docs/spec/02-requisitos.md";
const MANUAL_COVERAGE_FILE = "fixtures/manual-coverage.json";

/** Folders whose tests are searched (08 §13.3, step 2). */
const TEST_FOLDERS = ["src", "e2e", "tools", "engine", "src-tauri"];

/** Generated or installed folders that contain no tests of the project. */
const SKIPPED_FOLDERS = new Set(["node_modules", "target", "gen"]);

/** TypeScript, Vitest and Playwright test files (08 §2). */
const TYPESCRIPT_TEST_FILE = /\.(test|spec)\.(ts|tsx|mjs)$/;

/** A call `it("…")` or `test("…")` and the literal string of its title. */
const TEST_CALL = /(?<![\w$.])(?:it|test)\s*\(\s*(["'`])((?:\\.|(?!\1)[^\\])*)\1/g;

/** One or more criterion identifiers at the start of a title: `RF-401 CA1, RF-401 CA2: …`. */
const TITLE_CRITERIA = /^RF-\d{3} CA\d+(?:,\s*RF-\d{3} CA\d+)*(?!\d)/;

/** A Rust function whose name starts with a criterion identifier: `fn rf_211_ca1_…`. */
const RUST_TEST_FUNCTION = /\bfn\s+rf_(\d{3})_ca(\d+)/g;

/** The criteria (`RF-nnn CAn`) of the [1.0] requirements of 02, in document order. */
export function parseCriteria(markdown) {
  const criteria = [];
  let requirement;
  for (const line of markdown.split(/\r?\n/)) {
    const header = line.match(/^\*\*(RF-\d{3}) \[([^\]]+)\]/);
    if (header) {
      requirement = header[2] === "1.0" ? header[1] : undefined;
      continue;
    }
    if (/^#{1,6}\s/.test(line)) {
      requirement = undefined;
      continue;
    }
    const criterion = line.match(/^- (CA\d+):/);
    if (criterion && requirement) criteria.push(`${requirement} ${criterion[1]}`);
  }
  return criteria;
}

/** The criteria named at the start of the test titles of a TypeScript or Playwright file. */
export function criteriaInTypeScript(source) {
  const criteria = [];
  for (const [, , title] of source.matchAll(TEST_CALL)) {
    const lead = title.match(TITLE_CRITERIA);
    if (lead) criteria.push(...lead[0].split(/,\s*/));
  }
  return criteria;
}

/** The criteria named at the start of the function names of a Rust file. */
export function criteriaInRust(source) {
  return [...source.matchAll(RUST_TEST_FUNCTION)].map(
    ([, requirement, criterion]) => `RF-${requirement} CA${criterion}`,
  );
}

function visitFiles(folder, visit) {
  if (!existsSync(folder)) return;
  for (const entry of readdirSync(folder, { withFileTypes: true })) {
    const path = join(folder, entry.name);
    if (entry.isDirectory()) {
      if (!SKIPPED_FOLDERS.has(entry.name)) visitFiles(path, visit);
    } else if (entry.isFile()) {
      visit(path, entry.name);
    }
  }
}

/** Every criterion named by an automatic test under the test folders of `root`. */
export function findAutomaticCriteria(root) {
  const found = new Set();
  for (const folder of TEST_FOLDERS) {
    visitFiles(join(root, folder), (path, name) => {
      let criteria = [];
      if (TYPESCRIPT_TEST_FILE.test(name))
        criteria = criteriaInTypeScript(readFileSync(path, "utf8"));
      else if (name.endsWith(".rs")) criteria = criteriaInRust(readFileSync(path, "utf8"));
      for (const criterion of criteria) found.add(criterion);
    });
  }
  return found;
}

/** Splits the criteria into those with an automatic test, those only manual and the rest. */
export function classifyCriteria(criteria, automatic, manual) {
  const withTest = criteria.filter((criterion) => automatic.has(criterion));
  const manualOnly = criteria.filter(
    (criterion) => !automatic.has(criterion) && Object.hasOwn(manual, criterion),
  );
  const uncovered = criteria.filter(
    (criterion) => !automatic.has(criterion) && !Object.hasOwn(manual, criterion),
  );
  return { withTest, manualOnly, uncovered };
}

/** The report of 08 §13.3, step 4. */
export function formatReport(criteria, { withTest, manualOnly, uncovered }, manual) {
  const section = (title, lines) => [`${title}: ${lines.length}`, ...lines.map((l) => `  ${l}`)];
  return [
    `Acceptance criteria of the [1.0] requirements in ${REQUIREMENTS_FILE}: ${criteria.length}`,
    ...section("With an automatic test", withTest),
    ...section(
      "Only with a manual test",
      manualOnly.map((criterion) => `${criterion} (${manual[criterion]})`),
    ),
    ...section("Not covered", uncovered),
  ].join("\n");
}

/** Builds the report for the repository at `root` and the exit code of the script. */
export function runTraceability(root, { strict = false } = {}) {
  const criteria = parseCriteria(readFileSync(join(root, REQUIREMENTS_FILE), "utf8"));
  const manual = JSON.parse(readFileSync(join(root, MANUAL_COVERAGE_FILE), "utf8"));
  const classes = classifyCriteria(criteria, findAutomaticCriteria(root), manual);
  const report = formatReport(criteria, classes, manual);
  const exitCode = strict && classes.uncovered.length > 0 ? 1 : 0;
  return { report, exitCode, ...classes };
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const root = join(dirname(fileURLToPath(import.meta.url)), "..");
  const { report, exitCode } = runTraceability(root, { strict: process.argv.includes("--strict") });
  console.log(report);
  process.exitCode = exitCode;
}
