import type { Bridge, LoopRequest, PlayRequest, Telemetry } from "./bridge";
import type { EngineSnapshot } from "../core/model";

type Listener = (event: { payload: unknown }) => void;

const tauri = vi.hoisted(() => ({
  invoke: vi.fn<(command: string, args?: Record<string, unknown>) => Promise<unknown>>(),
  listen: vi.fn<(event: string, listener: Listener) => Promise<() => void>>(),
}));
vi.mock("@tauri-apps/api/core", () => ({ invoke: tauri.invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: tauri.listen }));

import { tauriBridge } from "./tauriBridge";

const snapshot: EngineSnapshot = {
  snapshotVersion: 1,
  tempoBpm: 120,
  timeSignature: "4/4",
  tracks: [],
  master: { gainDb: 0 },
};
const playRequest: PlayRequest = { mode: "pattern", trackId: "track-1", patternId: "pattern-1" };
const loop: LoopRequest = { enabled: true, startFrame: 0, endFrame: 384000 };
const telemetry: Telemetry = {
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

/** Each method of T-008: how to call it, and the command and arguments it must invoke (07 §6.2). */
const commands: [keyof Bridge, (bridge: Bridge) => Promise<unknown>, string, unknown][] = [
  ["engineSetProject", (b) => b.engineSetProject(snapshot), "engine_set_project", { snapshot }],
  ["enginePlay", (b) => b.enginePlay(playRequest), "engine_play", { request: playRequest }],
  ["engineStop", (b) => b.engineStop(), "engine_stop", undefined],
  ["engineSeek", (b) => b.engineSeek(96000), "engine_seek", { frame: 96000 }],
  ["engineSetLoop", (b) => b.engineSetLoop(loop), "engine_set_loop", { loop }],
  [
    "engineSetMetronome",
    (b) => b.engineSetMetronome(true),
    "engine_set_metronome",
    { enabled: true },
  ],
  [
    "engineAudition",
    (b) => b.engineAudition("track-1", 3),
    "engine_audition",
    { trackId: "track-1", row: 3 },
  ],
  ["engineInputOpen", (b) => b.engineInputOpen(), "engine_input_open", undefined],
  ["engineInputClose", (b) => b.engineInputClose(), "engine_input_close", undefined],
  ["engineTestTone", (b) => b.engineTestTone(), "engine_test_tone", undefined],
  ["appInfo", (b) => b.appInfo(), "app_info", undefined],
  [
    "logWrite",
    (b) => b.logWrite("warn", "late telemetry"),
    "log_write",
    { level: "warn", message: "late telemetry" },
  ],
];
const implemented = new Set<string>([...commands.map(([method]) => method), "onTelemetry"]);
const pending = Object.keys(tauriBridge).filter((method) => !implemented.has(method));
const pendingMethods = pending.filter((method) => !method.startsWith("on"));
const pendingEvents = pending.filter((method) => method.startsWith("on"));

/** Calls a method of the bridge by name. */
function callByName(method: string, ...args: unknown[]): unknown {
  const fn: unknown = Reflect.get(tauriBridge, method);
  return typeof fn === "function" ? Reflect.apply(fn, tauriBridge, args) : undefined;
}

beforeEach(() => {
  tauri.invoke.mockReset().mockResolvedValue(undefined);
  tauri.listen.mockReset();
});

describe("tauriBridge", () => {
  it.each(commands)("%s invokes its command with its arguments", async (_, call, command, args) => {
    await call(tauriBridge);
    expect(tauri.invoke.mock.calls).toEqual([[command, args]]);
  });

  it("appInfo returns what the command returns", async () => {
    tauri.invoke.mockResolvedValue({ version: "0.1.0", systemLocale: "es-ES" });
    await expect(tauriBridge.appInfo()).resolves.toEqual({
      version: "0.1.0",
      systemLocale: "es-ES",
    });
  });

  it("rejects with the BridgeError of the command", async () => {
    tauri.invoke.mockRejectedValue({ code: "NO_OUTPUT_DEVICE", detail: "no output stream" });
    await expect(tauriBridge.enginePlay(playRequest)).rejects.toEqual({
      code: "NO_OUTPUT_DEVICE",
      detail: "no output stream",
    });
  });

  it("turns any other rejection into ENGINE_INTERNAL with its text", async () => {
    const tauriError = "invalid args `frame` for command `engine_seek`";
    tauri.invoke.mockRejectedValueOnce(tauriError);
    await expect(tauriBridge.engineSeek(-1)).rejects.toEqual({
      code: "ENGINE_INTERNAL",
      detail: tauriError,
    });
    tauri.invoke.mockRejectedValueOnce({ code: "UNKNOWN", detail: "x" });
    await expect(tauriBridge.engineStop()).rejects.toEqual({
      code: "ENGINE_INTERNAL",
      detail: '{"code":"UNKNOWN","detail":"x"}',
    });
  });

  it("onTelemetry listens to engine://telemetry and delivers each payload", async () => {
    let listener: Listener = () => undefined;
    tauri.listen.mockImplementation((_, l) => {
      listener = l;
      return Promise.resolve(() => undefined);
    });
    const received: Telemetry[] = [];
    tauriBridge.onTelemetry((t) => received.push(t));
    await Promise.resolve();
    listener({ payload: telemetry });
    expect(tauri.listen).toHaveBeenCalledWith("engine://telemetry", expect.any(Function));
    expect(received).toEqual([telemetry]);
  });

  it("unsubscribing stops listening once listen has resolved, even if it had not yet", async () => {
    let resolveListen: (unlisten: () => void) => void = () => undefined;
    tauri.listen.mockReturnValue(new Promise((resolve) => (resolveListen = resolve)));
    const unlisten = vi.fn();
    const unsubscribe = tauriBridge.onTelemetry(() => undefined);
    unsubscribe();
    expect(unlisten).not.toHaveBeenCalled();
    resolveListen(unlisten);
    await vi.waitFor(() => expect(unlisten).toHaveBeenCalledTimes(1));
  });

  it("has every method of Bridge, and those of later tasks are pending (09 §1.3, rule 6)", () => {
    expect(Object.keys(tauriBridge)).toHaveLength(52);
    expect(pendingMethods).toHaveLength(31);
    expect(pendingEvents).toHaveLength(8);
  });

  it.each(pendingMethods)("%s rejects with ENGINE_INTERNAL without invoking", async (method) => {
    await expect(callByName(method)).rejects.toEqual({
      code: "ENGINE_INTERNAL",
      detail: `not implemented: ${method}`,
    });
    expect(tauri.invoke).not.toHaveBeenCalled();
  });

  it.each(pendingEvents)("%s returns an unsubscribe function without listening", (method) => {
    const unsubscribe = callByName(method, () => undefined);
    expect(unsubscribe).toBeTypeOf("function");
    expect(tauri.listen).not.toHaveBeenCalled();
  });
});
