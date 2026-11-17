import type { Bridge, Telemetry } from "../bridge/bridge";
import { listenToBridge, useEngineStore } from "./engineStore";

const playing: Telemetry = {
  state: "playing",
  mode: "pattern",
  positionFrame: 12000,
  patternStep: 1,
  trackPeaks: { "track-1": [0.5, 0.25] },
  masterPeak: [0.5, 0.25],
  inputPeak: null,
  xruns: 0,
  recordedFrames: null,
  outputAvailable: true,
};
const stopped: Telemetry = {
  state: "stopped",
  mode: null,
  positionFrame: 0,
  patternStep: null,
  trackPeaks: {},
  masterPeak: [0, 0],
  inputPeak: null,
  xruns: 0,
  recordedFrames: null,
  outputAvailable: true,
};

/** A source of telemetry that the test drives by hand. */
function fakeSource() {
  const listeners = new Set<(t: Telemetry) => void>();
  const source: Pick<Bridge, "onTelemetry"> = {
    onTelemetry: (callback) => {
      listeners.add(callback);
      return () => listeners.delete(callback);
    },
  };
  const emit = (t: Telemetry) => listeners.forEach((listener) => listener(t));
  return { source, emit };
}

beforeEach(() => {
  useEngineStore.setState({ telemetry: null });
});

describe("engineStore", () => {
  it("keeps the last telemetry received", () => {
    const { source, emit } = fakeSource();
    const unsubscribe = listenToBridge(source);
    emit(stopped);
    expect(useEngineStore.getState().telemetry).toEqual(stopped);
    emit(playing);
    expect(useEngineStore.getState().telemetry).toEqual(playing);
    unsubscribe();
  });

  it("stops changing once unsubscribed", () => {
    const { source, emit } = fakeSource();
    listenToBridge(source)();
    emit(playing);
    expect(useEngineStore.getState().telemetry).toBeNull();
  });

  it("listens to the bridge in use as soon as it is loaded", async () => {
    vi.resetModules();
    const store = await import("./engineStore");
    await vi.waitFor(() => expect(store.useEngineStore.getState().telemetry).toEqual(stopped));
  });
});
