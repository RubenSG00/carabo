import type { Bridge, LoopRequest, PlayRequest, Telemetry } from "./bridge";
import type { EngineSnapshot } from "../core/model";
import { mockBridge, mockControl } from "./mockBridge";

const snapshot: EngineSnapshot = {
  snapshotVersion: 1,
  tempoBpm: 120,
  timeSignature: "4/4",
  tracks: [],
  master: { gainDb: 0 },
};
const playRequest: PlayRequest = { mode: "song", startFrame: 0 };
const loop: LoopRequest = { enabled: false, startFrame: 0, endFrame: 192000 };

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

/** Each method of T-008, how to call it and the arguments it receives. */
const methods: [keyof Bridge, (bridge: Bridge) => Promise<unknown>, unknown[]][] = [
  ["engineSetProject", (b) => b.engineSetProject(snapshot), [snapshot]],
  ["enginePlay", (b) => b.enginePlay(playRequest), [playRequest]],
  ["engineStop", (b) => b.engineStop(), []],
  ["engineSeek", (b) => b.engineSeek(4800), [4800]],
  ["engineSetLoop", (b) => b.engineSetLoop(loop), [loop]],
  ["engineSetMetronome", (b) => b.engineSetMetronome(false), [false]],
  ["engineAudition", (b) => b.engineAudition("track-1", 0), ["track-1", 0]],
  ["engineInputOpen", (b) => b.engineInputOpen(), []],
  ["engineInputClose", (b) => b.engineInputClose(), []],
  ["engineTestTone", (b) => b.engineTestTone(), []],
  ["appInfo", (b) => b.appInfo(), []],
  ["logWrite", (b) => b.logWrite("error", "boom"), ["error", "boom"]],
];
const implemented = new Set<string>([...methods.map(([method]) => method), "onTelemetry"]);
const pending = Object.keys(mockBridge).filter((method) => !implemented.has(method));
const pendingMethods = pending.filter((method) => !method.startsWith("on"));
const pendingEvents = pending.filter((method) => method.startsWith("on"));

/** Calls a method of the bridge by name. */
function callByName(method: string, ...args: unknown[]): unknown {
  const fn: unknown = Reflect.get(mockBridge, method);
  return typeof fn === "function" ? Reflect.apply(fn, mockBridge, args) : undefined;
}

/** Lets the pending microtasks run. */
const settle = () => new Promise<void>((resolve) => setTimeout(resolve, 0));

beforeEach(() => {
  mockControl.clearCalls();
});

describe("mockBridge", () => {
  it.each(methods)("%s keeps what it receives and resolves", async (method, call, args) => {
    await call(mockBridge);
    expect(mockControl.calls).toEqual([{ method, args }]);
  });

  it("appInfo gives a version and a language of the system", async () => {
    await expect(mockBridge.appInfo()).resolves.toEqual({
      version: "0.0.0",
      systemLocale: "es-ES",
    });
  });

  it("keeps the calls in order until clearCalls empties the list", async () => {
    await mockBridge.engineSetProject(snapshot);
    await mockBridge.enginePlay(playRequest);
    await mockBridge.engineStop();
    expect(mockControl.calls.map((call) => call.method)).toEqual([
      "engineSetProject",
      "enginePlay",
      "engineStop",
    ]);
    mockControl.clearCalls();
    expect(mockControl.calls).toEqual([]);
  });

  it("onTelemetry emits one telemetry in the stopped state", async () => {
    const received: Telemetry[] = [];
    mockBridge.onTelemetry((t) => received.push(t));
    await settle();
    expect(received).toEqual([stopped]);
  });

  it("onTelemetry emits nothing after unsubscribing", async () => {
    const received: Telemetry[] = [];
    const unsubscribe = mockBridge.onTelemetry((t) => received.push(t));
    unsubscribe();
    await settle();
    expect(received).toEqual([]);
  });

  it("has every method of Bridge, and those of later tasks are pending (09 §1.3, rule 6)", () => {
    expect(Object.keys(mockBridge)).toHaveLength(52);
    expect(pendingMethods).toHaveLength(31);
    expect(pendingEvents).toHaveLength(8);
  });

  it.each(pendingMethods)("%s rejects with ENGINE_INTERNAL", async (method) => {
    await expect(callByName(method)).rejects.toEqual({
      code: "ENGINE_INTERNAL",
      detail: `not implemented: ${method}`,
    });
  });

  it.each(pendingEvents)("%s returns an unsubscribe function and emits nothing", async (method) => {
    const callback = vi.fn();
    const unsubscribe = callByName(method, callback);
    await settle();
    expect(unsubscribe).toBeTypeOf("function");
    expect(callback).not.toHaveBeenCalled();
  });
});
