// Which bridge is used (07 §6.5): the Tauri one inside Tauri, the simulated one elsewhere, and
// the Tauri one is loaded only inside Tauri.
import type { Bridge } from "./bridge";

const tauriBridgeLoaded = vi.fn();
const fakeTauriBridge: Partial<Bridge> = {};

function mockTauriBridgeModule() {
  vi.doMock("./tauriBridge", () => {
    tauriBridgeLoaded();
    return { tauriBridge: fakeTauriBridge };
  });
}

beforeEach(() => {
  vi.resetModules();
  tauriBridgeLoaded.mockClear();
  mockTauriBridgeModule();
});

afterEach(() => {
  Reflect.deleteProperty(window, "__TAURI_INTERNALS__");
  vi.doUnmock("./tauriBridge");
});

describe("bridge", () => {
  it("is the simulated bridge outside Tauri, without loading the Tauri bridge", async () => {
    const { bridge } = await import("./index");
    const { mockBridge } = await import("./mockBridge");
    expect(bridge).toBe(mockBridge);
    expect(tauriBridgeLoaded).not.toHaveBeenCalled();
  });

  it("is the Tauri bridge when __TAURI_INTERNALS__ is in window", async () => {
    Object.assign(window, { __TAURI_INTERNALS__: {} });
    const { bridge } = await import("./index");
    expect(bridge).toBe(fakeTauriBridge);
    expect(tauriBridgeLoaded).toHaveBeenCalledTimes(1);
  });
});
