// Checks that every dependency shipped inside the executable has an admitted license (07 §11).
// Usage: node tools/check-licenses.mjs   (npm run licenses)
// Follows the steps of 07 §13.3: lists the Rust packages with `cargo tree` and the npm packages
// with `npm ls`, evaluates each license expression, prints the rejected packages and exits with
// code 1 if there is any.

import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

/** Licenses admitted in distributed dependencies (07 §11.2). */
export const ADMITTED_LICENSES = new Set([
  "MIT",
  "MIT-0",
  "Apache-2.0",
  "BSD-2-Clause",
  "BSD-3-Clause",
  "ISC",
  "Zlib",
  "0BSD",
  "BSL-1.0",
  "Unicode-3.0",
  "Unicode-DFS-2016",
  "CC0-1.0",
  "Unlicense",
  // Only while the dependency is used without changing its code (07 §11.2).
  "MPL-2.0",
]);

/** Packages admitted by name whatever their license: the MP3 encoder (07 §11.3). */
export const LICENSE_EXCEPTIONS = new Set(["mp3lame-encoder", "mp3lame-sys"]);

/** The project's own packages, which are not checked (07 §13.3). */
export const OWN_PACKAGES = new Set(["carabo", "carabo-engine"]);

const RUST_COMMAND = 'cargo tree -p carabo -e normal,build --prefix none --format "{p}|{l}"';
const NPM_COMMAND = "npm ls --omit=dev --all --parseable";

/** Splits a license expression into names, parentheses and the `/` operator. */
function tokenize(expression) {
  return expression.match(/[()/]|[^\s()/]+/g) ?? [];
}

/**
 * Whether a license expression is admitted (07 §13.3, step 3): `A OR B` (or `A/B`) if either is
 * admitted, `A AND B` if both are, parentheses are respected and `WITH <exception>` is ignored.
 * `AND` binds tighter than `OR`, as in SPDX expressions. An empty or malformed expression is
 * rejected.
 */
export function isLicenseAdmitted(expression) {
  const tokens = tokenize(expression);
  let position = 0;
  const peek = () => tokens[position];
  const take = () => tokens[position++];

  const name = () => {
    const token = take();
    if (token === undefined || ["(", ")", "/", "OR", "AND", "WITH"].includes(token)) {
      throw new SyntaxError(`unexpected ${token ?? "end"} in "${expression}"`);
    }
    return token;
  };

  const parseTerm = () => {
    let admitted;
    if (peek() === "(") {
      take();
      admitted = parseChoice();
      if (take() !== ")") throw new SyntaxError(`missing ")" in "${expression}"`);
    } else {
      admitted = ADMITTED_LICENSES.has(name());
    }
    if (peek() === "WITH") {
      take();
      name();
    }
    return admitted;
  };

  const parseCombination = () => {
    let admitted = parseTerm();
    while (peek() === "AND") {
      take();
      const next = parseTerm();
      admitted = admitted && next;
    }
    return admitted;
  };

  const parseChoice = () => {
    let admitted = parseCombination();
    while (peek() === "OR" || peek() === "/") {
      take();
      const next = parseCombination();
      admitted = admitted || next;
    }
    return admitted;
  };

  try {
    const admitted = parseChoice();
    return admitted && position === tokens.length;
  } catch {
    return false;
  }
}

/**
 * The unique packages listed by `cargo tree --prefix none --format "{p}|{l}"`. Cargo marks the
 * packages it has already listed with a trailing `(*)`, which is removed before deduplicating.
 */
export function parseCargoTree(output) {
  const packages = new Map();
  for (const rawLine of output.split(/\r?\n/)) {
    const line = rawLine.replace(/\s*\(\*\)\s*$/, "").trim();
    if (line === "") continue;
    const separator = line.indexOf("|");
    const id = separator === -1 ? line : line.slice(0, separator);
    const license = separator === -1 ? "" : line.slice(separator + 1).trim();
    const [name = "", version = ""] = id.trim().split(/\s+/);
    packages.set(`${name} ${version}`, { name, version, license });
  }
  return [...packages.values()];
}

/** The packages of the folders listed by `npm ls --parseable`, with the `license` of each. */
export function readNpmPackages(output) {
  const folders = new Set(output.split(/\r?\n/).map((line) => line.trim()));
  folders.delete("");
  return [...folders].map((folder) => {
    const manifest = JSON.parse(readFileSync(join(folder, "package.json"), "utf8"));
    const license = typeof manifest.license === "string" ? manifest.license.trim() : "";
    return { name: manifest.name ?? folder, version: manifest.version ?? "", license };
  });
}

/** The packages that are distributed: every package except the project's own. */
export function distributedPackages(packages) {
  return packages.filter((pkg) => !OWN_PACKAGES.has(pkg.name));
}

/** The distributed packages whose license is not admitted (07 §13.3, steps 4 and 5). */
export function rejectedPackages(packages) {
  return distributedPackages(packages).filter(
    (pkg) => !LICENSE_EXCEPTIONS.has(pkg.name) && !isLicenseAdmitted(pkg.license),
  );
}

function main() {
  const root = join(dirname(fileURLToPath(import.meta.url)), "..");
  const run = (command) =>
    execSync(command, {
      cwd: root,
      encoding: "utf8",
      maxBuffer: 64 * 1024 * 1024,
      stdio: ["ignore", "pipe", "inherit"],
    });

  const rust = distributedPackages(parseCargoTree(run(RUST_COMMAND)));
  const npm = distributedPackages(readNpmPackages(run(NPM_COMMAND)));
  console.log(
    `Checked the licenses of ${rust.length} Rust packages and ${npm.length} npm packages.`,
  );

  const rejected = rejectedPackages([...rust, ...npm]);
  if (rejected.length === 0) {
    console.log("All licenses are admitted (07 §11.2).");
    return 0;
  }
  console.log(`${rejected.length} packages have a license that is not admitted (07 §11.2):`);
  for (const pkg of rejected) {
    console.log(`  ${pkg.name} ${pkg.version}: ${pkg.license || "no license declared"}`);
  }
  return 1;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.exitCode = main();
}
