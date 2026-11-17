// The bridge in use (07 §6.5): the Tauri one when the application runs inside Tauri, and the
// simulated one otherwise (`npm run dev` in a browser, and the tests). The Tauri one is loaded
// with a dynamic import, so that a browser never loads the Tauri packages.
import type { Bridge } from "./bridge";
import { mockBridge } from "./mockBridge";

export const bridge: Bridge =
  "__TAURI_INTERNALS__" in window ? (await import("./tauriBridge")).tauriBridge : mockBridge;
