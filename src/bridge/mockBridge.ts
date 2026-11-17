// The simulated bridge (04 §3.3, 08 §5.2): it works in memory, without Tauri or the engine, for
// the tests and for running the interface in a browser. For now, the methods of T-008 keep what
// they receive and `onTelemetry` emits one telemetry in the stopped state.
// TODO(T-023): the fake file system, the fake engine and the rest of `mockControl`.
import type { Bridge, BridgeError, Telemetry, Unsubscribe } from "./bridge";

/** A call received by the simulated bridge (08 §5.2). */
export interface MockCall {
  method: keyof Bridge;
  args: unknown[];
}

const calls: MockCall[] = [];

/** Control object of the simulated bridge for the tests (08 §5.2). */
export const mockControl = {
  /** Every call received, in order. */
  calls,
  /** Empties `calls`. */
  clearCalls(): void {
    calls.length = 0;
  },
};

/** What the engine reports when it is stopped, with an output device. */
function stoppedTelemetry(): Telemetry {
  return {
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
}

/** A promise rejected with `error`. */
function rejection(error: BridgeError): Promise<never> {
  // eslint-disable-next-line @typescript-eslint/prefer-promise-reject-errors -- 04 §6.1: the bridge rejects with a BridgeError object.
  return Promise.reject(error);
}

/** Keeps a call and resolves with `result`. */
function received<T>(method: keyof Bridge, args: unknown[], result: T): Promise<T> {
  calls.push({ method, args });
  return Promise.resolve(result);
}

/** A method whose task has not been done yet (09 §1.3, rule 6). */
function notImplemented(method: keyof Bridge, args: unknown[]): Promise<never> {
  calls.push({ method, args });
  return rejection({ code: "ENGINE_INTERNAL", detail: `not implemented: ${method}` });
}

/** An event whose task has not been done yet: nothing to unsubscribe from (09 §1.3, rule 6). */
function noEvent(): Unsubscribe {
  return () => undefined;
}

export const mockBridge: Bridge = {
  // ---- Project files ----
  projectCreateTemp: (...args) => notImplemented("projectCreateTemp", args),
  projectRead: (...args) => notImplemented("projectRead", args),
  projectCheckFiles: (...args) => notImplemented("projectCheckFiles", args),
  projectCleanup: (...args) => notImplemented("projectCleanup", args),
  projectWrite: (...args) => notImplemented("projectWrite", args),
  projectSaveAs: (...args) => notImplemented("projectSaveAs", args),
  projectDiscardTemp: (...args) => notImplemented("projectDiscardTemp", args),
  recoveryWrite: (...args) => notImplemented("recoveryWrite", args),
  recoveryRead: (...args) => notImplemented("recoveryRead", args),
  recoveryDelete: (...args) => notImplemented("recoveryDelete", args),
  sessionGet: (...args) => notImplemented("sessionGet", args),
  sessionSet: (...args) => notImplemented("sessionSet", args),
  settingsRead: (...args) => notImplemented("settingsRead", args),
  settingsWrite: (...args) => notImplemented("settingsWrite", args),
  defaultProjectsDir: (...args) => notImplemented("defaultProjectsDir", args),

  // ---- Native dialogs ----
  pickProjectFile: (...args) => notImplemented("pickProjectFile", args),
  pickFolder: (...args) => notImplemented("pickFolder", args),
  pickAudioFile: (...args) => notImplemented("pickAudioFile", args),
  pickExportPath: (...args) => notImplemented("pickExportPath", args),
  revealInExplorer: (...args) => notImplemented("revealInExplorer", args),

  // ---- Audio files ----
  audioImport: (...args) => notImplemented("audioImport", args),
  audioDiscard: (...args) => notImplemented("audioDiscard", args),
  peaksGet: (...args) => notImplemented("peaksGet", args),
  exportStart: (...args) => notImplemented("exportStart", args),
  exportCancel: (...args) => notImplemented("exportCancel", args),

  // ---- Engine ----
  engineSetProject: (...args) => received("engineSetProject", args, undefined),
  enginePlay: (...args) => received("enginePlay", args, undefined),
  engineStop: (...args) => received("engineStop", args, undefined),
  engineSeek: (...args) => received("engineSeek", args, undefined),
  engineSetLoop: (...args) => received("engineSetLoop", args, undefined),
  engineSetMetronome: (...args) => received("engineSetMetronome", args, undefined),
  engineAudition: (...args) => received("engineAudition", args, undefined),
  engineRecordStart: (...args) => notImplemented("engineRecordStart", args),
  engineInputOpen: (...args) => received("engineInputOpen", args, undefined),
  engineInputClose: (...args) => received("engineInputClose", args, undefined),
  devicesList: (...args) => notImplemented("devicesList", args),
  devicesSelect: (...args) => notImplemented("devicesSelect", args),
  calibrationRun: (...args) => notImplemented("calibrationRun", args),
  engineTestTone: (...args) => received("engineTestTone", args, undefined),

  // ---- System ----
  appInfo: (...args) => received("appInfo", args, { version: "0.0.0", systemLocale: "es-ES" }),
  logWrite: (...args) => received("logWrite", args, undefined),
  setWindowTitle: (...args) => notImplemented("setWindowTitle", args),
  closeWindow: (...args) => notImplemented("closeWindow", args),

  // ---- Events ----
  onTelemetry: (callback) => {
    // Like a real event, the telemetry arrives after subscribing, never during it.
    let subscribed = true;
    queueMicrotask(() => {
      if (subscribed) {
        callback(stoppedTelemetry());
      }
    });
    return () => {
      subscribed = false;
    };
  },
  onRecordingFinished: noEvent,
  onDevicesChanged: noEvent,
  onEngineError: noEvent,
  onAudioUnavailable: noEvent,
  onImportProgress: noEvent,
  onExportProgress: noEvent,
  onFileDrop: noEvent,
  onCloseRequested: noEvent,
};
