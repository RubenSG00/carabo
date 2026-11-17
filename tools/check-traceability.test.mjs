// Tests of tools/check-traceability.mjs (08 §13.3).
// The examples use requirement numbers that do not exist (RF-99x), so the test titles they contain
// never count as coverage when the script searches this file.

import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it } from "vitest";
import {
  criteriaInRust,
  criteriaInTypeScript,
  findAutomaticCriteria,
  parseCriteria,
  runTraceability,
} from "./check-traceability.mjs";

const REPO_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const readRepoFile = (path) => readFileSync(join(REPO_ROOT, path), "utf8");

const REQUIREMENTS = [
  "## 2. Example",
  "",
  "**RF-990 [1.0] · First**",
  "Statement.",
  "**Criterios de aceptación:**",
  "- CA1: one.",
  "- CA2: two.",
  "",
  "**RF-991 [1.0] · Second**",
  "**Criterios de aceptación:**",
  "- CA1: one.",
  "",
  "**RF-992 [1.1] · Later**",
  "**Criterios de aceptación:**",
  "- CA1: not in 1.0.",
  "",
  "## 3. Non-functional",
  "",
  "- CA9: not a criterion of any requirement.",
].join("\n");

const TYPESCRIPT_EXAMPLE = `
import { describe, expect, it } from "vitest";
describe("example", () => {
  it("RF-990 CA1: the first criterion", () => expect(1).toBe(1));
  it(
    "RF-990 CA2, RF-991 CA1: two criteria in one wrapped title",
    () => expect(1).toBe(1),
  );
  it("mentions RF-992 CA1 later in the title", () => expect(1).toBe(1));
});
`;

const PLAYWRIGHT_EXAMPLE = `
import { test } from "@playwright/test";
test('RF-993 CA1: a Playwright test', async ({ page }) => { await page.goto("/"); });
`;

const RUST_EXAMPLE = `
#[cfg(test)]
mod tests {
    #[test]
    fn rf_994_ca1_hits_start_on_their_exact_frame() {}

    #[test]
    fn rf_994_ca12_two_digit_criterion() {}

    fn helper_for_rf_994_ca3() {}
}
`;

/** The PA entries of the table of 08 §13.1, as `{ "RF-nnn CAn": "PA-nn" }`. */
function manualEntriesOfTestTable() {
  const text = readRepoFile("docs/spec/08-pruebas.md");
  const table = text.slice(text.indexOf("### 13.1"), text.indexOf("### 13.2"));
  const entries = {};
  for (const line of table.split("\n").filter((l) => l.startsWith("| RF-"))) {
    const cells = line
      .split("|")
      .slice(1, -1)
      .map((cell) => cell.trim());
    for (let i = 0; i + 1 < cells.length; i += 2) {
      for (const part of cells[i + 1].split(" · ")) {
        const [criterion, ...levels] = part.split(/[\s,]+/);
        for (const level of levels.filter((l) => /^PA-\d{2}$/.test(l))) {
          entries[`${cells[i]} CA${criterion}`] = level;
        }
      }
    }
  }
  return entries;
}

describe("requirements", () => {
  it("reads only the criteria of the [1.0] requirements", () => {
    expect(parseCriteria(REQUIREMENTS)).toEqual(["RF-990 CA1", "RF-990 CA2", "RF-991 CA1"]);
    expect(parseCriteria(REQUIREMENTS.replaceAll("\n", "\r\n"))).toHaveLength(3);
  });

  it("reads the 172 criteria of the [1.0] requirements of 02", () => {
    const criteria = parseCriteria(readRepoFile("docs/spec/02-requisitos.md"));
    expect(criteria).toHaveLength(172);
    expect(new Set(criteria).size).toBe(criteria.length);
    expect(criteria[0]).toBe("RF-001 CA1");
    expect(criteria).toContain("RF-703 CA2");
    expect(criteria).not.toContain("RF-033 CA1");
    expect(criteria).not.toContain("RF-800 CA1");
  });
});

describe("test names", () => {
  it("finds the criteria at the start of TypeScript and Playwright test titles", () => {
    expect(criteriaInTypeScript(TYPESCRIPT_EXAMPLE)).toEqual([
      "RF-990 CA1",
      "RF-990 CA2",
      "RF-991 CA1",
    ]);
    expect(criteriaInTypeScript(PLAYWRIGHT_EXAMPLE)).toEqual(["RF-993 CA1"]);
  });

  it("finds the criteria at the start of Rust function names", () => {
    expect(criteriaInRust(RUST_EXAMPLE)).toEqual(["RF-994 CA1", "RF-994 CA12"]);
  });
});

describe("report", () => {
  let root;
  afterEach(() => {
    if (root) rmSync(root, { recursive: true, force: true });
    root = undefined;
  });

  /** A repository in a temporary folder with an example file of each language. */
  function exampleRepository(manualCoverage) {
    root = mkdtempSync(join(tmpdir(), "carabo-traceability-"));
    const files = {
      "docs/spec/02-requisitos.md": REQUIREMENTS,
      "fixtures/manual-coverage.json": JSON.stringify(manualCoverage),
      "src/ui/example.test.tsx": TYPESCRIPT_EXAMPLE.replace("RF-990 CA2, ", ""),
      "src/core/example.ts": 'it("RF-991 CA1: not in a test file", () => {});',
      "e2e/example.spec.ts": PLAYWRIGHT_EXAMPLE,
      "engine/tests/example.rs": RUST_EXAMPLE.replace("rf_994_ca1_", "rf_990_ca2_"),
      "src-tauri/target/debug/example.rs": "fn rf_991_ca1_generated() {}",
    };
    for (const [path, content] of Object.entries(files)) {
      mkdirSync(dirname(join(root, path)), { recursive: true });
      writeFileSync(join(root, path), content);
    }
    return root;
  }

  it("finds the tests of an example file of each language in the test folders", () => {
    const found = findAutomaticCriteria(exampleRepository({}));
    expect([...found].sort()).toEqual([
      "RF-990 CA1",
      "RF-990 CA2",
      "RF-991 CA1",
      "RF-993 CA1",
      "RF-994 CA12",
    ]);
  });

  it("reports the uncovered criteria and fails only with --strict", () => {
    const repository = exampleRepository({});
    const result = runTraceability(repository);
    expect(result.exitCode).toBe(0);
    expect(result.withTest).toEqual(["RF-990 CA1", "RF-990 CA2", "RF-991 CA1"]);
    expect(result.uncovered).toEqual([]);

    rmSync(join(repository, "src/ui/example.test.tsx"));
    const partial = runTraceability(repository, { strict: true });
    expect(partial.withTest).toEqual(["RF-990 CA2"]);
    expect(partial.uncovered).toEqual(["RF-990 CA1", "RF-991 CA1"]);
    expect(partial.report).toContain("Not covered: 2\n  RF-990 CA1\n  RF-991 CA1");
    expect(partial.exitCode).toBe(1);
    expect(runTraceability(repository, { strict: false }).exitCode).toBe(0);
  });

  it("counts a criterion covered by a manual test as covered", () => {
    const repository = exampleRepository({ "RF-990 CA1": "PA-06", "RF-991 CA1": "PA-05" });
    rmSync(join(repository, "src/ui/example.test.tsx"));
    const result = runTraceability(repository, { strict: true });
    expect(result.manualOnly).toEqual(["RF-990 CA1", "RF-991 CA1"]);
    expect(result.report).toContain("Only with a manual test: 2\n  RF-990 CA1 (PA-06)");
    expect(result.uncovered).toEqual([]);
    expect(result.exitCode).toBe(0);
  });

  it("the manual coverage holds exactly the PA entries of 08 §13.1", () => {
    const manual = JSON.parse(readRepoFile("fixtures/manual-coverage.json"));
    expect(Object.keys(manual)).toHaveLength(20);
    expect(manual).toEqual(manualEntriesOfTestTable());
  });
});
