// Static checks of the application configuration (08 §10): src-tauri/tauri.conf.json and
// src-tauri/capabilities/default.json.

import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const readJson = (path) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));

const conf = readJson("../src-tauri/tauri.conf.json");
const capabilities = readJson("../src-tauri/capabilities/default.json");

// TODO(T-004): import WINDOW_MIN_WIDTH and WINDOW_MIN_HEIGHT from src/core/constants.ts.
const WINDOW_MIN_WIDTH = 1280;
const WINDOW_MIN_HEIGHT = 720;

/** The permissions of 07 §5.7, and no others. */
const ALLOWED_PERMISSIONS = [
  "core:default",
  "core:window:allow-set-title",
  "core:window:allow-destroy",
  "dialog:allow-open",
  "dialog:allow-save",
  "opener:allow-reveal-item-in-dir",
];

/** Every `http:` or `https:` source named in a content security policy. */
function webOrigins(csp) {
  return csp
    .split(";")
    .flatMap((directive) => directive.trim().split(/\s+/).slice(1))
    .filter((source) => /^https?:/i.test(source));
}

describe("application configuration", () => {
  it("RF-001 CA1: the window cannot be made smaller than 1280 × 720", () => {
    const [window] = conf.app.windows;
    expect(window.minWidth).toBe(WINDOW_MIN_WIDTH);
    expect(window.minHeight).toBe(WINDOW_MIN_HEIGHT);
    expect(window.resizable).toBe(true);
  });

  it("RF-001 CA3: there is a single window and no permission to open others", () => {
    expect(conf.app.windows).toHaveLength(1);
    expect(conf.app.windows[0].label).toBe("main");
    expect(capabilities.windows).toEqual(["main"]);
    expect(capabilities.permissions).toEqual(ALLOWED_PERMISSIONS);
  });

  it("RF-009 CA2: the configuration has no network permissions or remote origins", () => {
    expect(capabilities.permissions).toEqual(ALLOWED_PERMISSIONS);
    expect(webOrigins(conf.app.security.csp)).toEqual(["http://ipc.localhost"]);
  });

  it("finds the web origins of a content security policy", () => {
    expect(webOrigins("default-src 'self'; connect-src https://example.com ipc:")).toEqual([
      "https://example.com",
    ]);
    expect(webOrigins("img-src 'self' data: blob:")).toEqual([]);
  });
});
