// Tests of tools/check-licenses.mjs (07 §13.3).

import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import {
  ADMITTED_LICENSES,
  isLicenseAdmitted,
  parseCargoTree,
  readNpmPackages,
  rejectedPackages,
} from "./check-licenses.mjs";

describe("license expressions", () => {
  it("admits a choice when one of its licenses is admitted", () => {
    expect(isLicenseAdmitted("MIT OR Apache-2.0")).toBe(true);
    expect(isLicenseAdmitted("GPL-3.0 OR MIT")).toBe(true);
    expect(isLicenseAdmitted("MIT/Apache-2.0")).toBe(true);
    expect(isLicenseAdmitted("Apache-2.0 / MIT")).toBe(true);
    expect(isLicenseAdmitted("GPL-3.0 OR LGPL-2.1")).toBe(false);
  });

  it("admits a combination only when all its licenses are admitted", () => {
    expect(isLicenseAdmitted("(MIT OR Apache-2.0) AND Unicode-3.0")).toBe(true);
    expect(isLicenseAdmitted("Apache-2.0 AND MIT")).toBe(true);
    expect(isLicenseAdmitted("MIT AND GPL-3.0")).toBe(false);
  });

  it("respects parentheses, and AND binds tighter than OR", () => {
    expect(isLicenseAdmitted("(GPL-3.0 OR MIT) AND Apache-2.0")).toBe(true);
    expect(isLicenseAdmitted("GPL-3.0 OR (MIT AND LGPL-2.1)")).toBe(false);
    expect(isLicenseAdmitted("MIT OR GPL-3.0 AND LGPL-2.1")).toBe(true);
    expect(isLicenseAdmitted("(MIT OR GPL-3.0) AND LGPL-2.1")).toBe(false);
  });

  it("ignores the exception after WITH", () => {
    expect(isLicenseAdmitted("Apache-2.0 WITH LLVM-exception")).toBe(true);
    expect(isLicenseAdmitted("Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT")).toBe(true);
    expect(isLicenseAdmitted("GPL-2.0 WITH Classpath-exception-2.0")).toBe(false);
  });

  it("admits MPL-2.0", () => {
    expect(isLicenseAdmitted("MPL-2.0")).toBe(true);
  });

  it("admits every license of 07 §11.2", () => {
    for (const license of ADMITTED_LICENSES) expect(isLicenseAdmitted(license)).toBe(true);
    expect(ADMITTED_LICENSES.size).toBe(14);
  });

  it("rejects GPL-3.0, an empty license and a malformed expression", () => {
    expect(isLicenseAdmitted("GPL-3.0")).toBe(false);
    expect(isLicenseAdmitted("")).toBe(false);
    expect(isLicenseAdmitted("   ")).toBe(false);
    expect(isLicenseAdmitted("MIT OR")).toBe(false);
    expect(isLicenseAdmitted("(MIT")).toBe(false);
    expect(isLicenseAdmitted("MIT Apache-2.0")).toBe(false);
  });
});

describe("dependency lists", () => {
  let folder;
  afterEach(() => {
    if (folder) rmSync(folder, { recursive: true, force: true });
    folder = undefined;
  });

  it("reads each package of the cargo tree output once", () => {
    const output = [
      "carabo v0.0.0 (C:\\code\\carabo\\src-tauri)|MIT",
      "serde v1.0.228|MIT OR Apache-2.0",
      "serde v1.0.228|MIT OR Apache-2.0 (*)",
      "unicode-ident v1.0.24|(MIT OR Apache-2.0) AND Unicode-3.0\r",
      "unlicensed v0.1.0|",
      "",
    ].join("\n");
    expect(parseCargoTree(output)).toEqual([
      { name: "carabo", version: "v0.0.0", license: "MIT" },
      { name: "serde", version: "v1.0.228", license: "MIT OR Apache-2.0" },
      { name: "unicode-ident", version: "v1.0.24", license: "(MIT OR Apache-2.0) AND Unicode-3.0" },
      { name: "unlicensed", version: "v0.1.0", license: "" },
    ]);
  });

  it("reads the license field of each npm package folder", () => {
    folder = mkdtempSync(join(tmpdir(), "carabo-licenses-"));
    const write = (name, manifest) => {
      mkdirSync(join(folder, name), { recursive: true });
      writeFileSync(join(folder, name, "package.json"), JSON.stringify(manifest));
    };
    write("a", { name: "a", version: "1.0.0", license: "MIT" });
    write("b", { name: "b", version: "2.0.0" });
    const output = `${join(folder, "a")}\n${join(folder, "b")}\n`;
    expect(readNpmPackages(output)).toEqual([
      { name: "a", version: "1.0.0", license: "MIT" },
      { name: "b", version: "2.0.0", license: "" },
    ]);
  });

  it("rejects packages without an admitted license, but not own packages or the MP3 encoder", () => {
    const packages = [
      { name: "carabo", version: "v0.0.0", license: "" },
      { name: "carabo-engine", version: "v0.0.0", license: "" },
      { name: "mp3lame-sys", version: "v0.1.0", license: "LGPL-2.1" },
      { name: "mp3lame-encoder", version: "v0.2.0", license: "LGPL-3.0" },
      { name: "serde", version: "v1.0.228", license: "MIT OR Apache-2.0" },
      { name: "copyleft", version: "v1.0.0", license: "GPL-3.0" },
      { name: "undeclared", version: "1.0.0", license: "" },
    ];
    expect(rejectedPackages(packages).map((pkg) => pkg.name)).toEqual(["copyleft", "undeclared"]);
  });
});
