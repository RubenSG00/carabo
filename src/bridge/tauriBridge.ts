// The real bridge: each method is a Tauri command or event (04 §6.2, §6.4; 07 §6). This is the
// only file of src/ that imports from @tauri-apps/* (04 §9).
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Bridge, BridgeError, BridgeErrorCode, Unsubscribe } from "./bridge";

const BRIDGE_ERROR_CODES: readonly BridgeErrorCode[] = [
  "IO",
  "NOT_FOUND",
  "ALREADY_EXISTS",
  "NAME_INVALID",
  "UNSUPPORTED_FORMAT",
  "BUSY",
  "CANCELLED",
  "NOTHING_TO_EXPORT",
  "NO_OUTPUT_DEVICE",
  "NO_INPUT_DEVICE",
  "CALIBRATION_FAILED",
  "INVALID_SNAPSHOT",
  "ENGINE_INTERNAL",
];

function isBridgeError(value: unknown): value is BridgeError {
  return (
    typeof value === "object" &&
    value !== null &&
    "code" in value &&
    "detail" in value &&
    BRIDGE_ERROR_CODES.some((code) => code === value.code) &&
    typeof value.detail === "string"
  );
}

/**
 * The bridge rejects only with a `BridgeError` (04 §6.1). Commands already fail with one; what
 * Tauri rejects with by itself (for example, arguments it cannot read) becomes `ENGINE_INTERNAL`.
 */
function toBridgeError(error: unknown): BridgeError {
  if (isBridgeError(error)) {
    return error;
  }
  const detail = typeof error === "string" ? error : String(JSON.stringify(error));
  return { code: "ENGINE_INTERNAL", detail };
}

/** A promise rejected with `error`. */
function rejection(error: BridgeError): Promise<never> {
  // eslint-disable-next-line @typescript-eslint/prefer-promise-reject-errors -- 04 §6.1: the bridge rejects with a BridgeError object.
  return Promise.reject(error);
}

/** Invokes a command; its arguments go in camelCase and Tauri hands them to Rust in snake_case (07 §6.2). */
function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return invoke<T>(command, args).catch((error: unknown) => rejection(toBridgeError(error)));
}

/**
 * Listens to a Tauri event. `listen` gives its unsubscribe function in a promise, so it is kept
 * and, on unsubscribing, called once it resolves (07 §6.3).
 */
function subscribe<T>(event: string, callback: (payload: T) => void): Unsubscribe {
  const unlisten = listen<T>(event, (e) => {
    callback(e.payload);
  });
  return () => {
    void unlisten.then((stop) => {
      stop();
    });
  };
}

/** A method whose task has not been done yet (09 §1.3, rule 6). */
function notImplemented(method: keyof Bridge): Promise<never> {
  return rejection({ code: "ENGINE_INTERNAL", detail: `not implemented: ${method}` });
}

/** An event whose task has not been done yet: nothing to unsubscribe from (09 §1.3, rule 6). */
function noEvent(): Unsubscribe {
  return () => undefined;
}

export const tauriBridge: Bridge = {
  // ---- Project files ----
  // TODO(T-050): project_* commands. TODO(T-051): recovery_*, session_* and settings_*.
  projectCreateTemp: () => notImplemented("projectCreateTemp"),
  projectRead: () => notImplemented("projectRead"),
  projectCheckFiles: () => notImplemented("projectCheckFiles"),
  projectCleanup: () => notImplemented("projectCleanup"),
  projectWrite: () => notImplemented("projectWrite"),
  projectSaveAs: () => notImplemented("projectSaveAs"),
  projectDiscardTemp: () => notImplemented("projectDiscardTemp"),
  recoveryWrite: () => notImplemented("recoveryWrite"),
  recoveryRead: () => notImplemented("recoveryRead"),
  recoveryDelete: () => notImplemented("recoveryDelete"),
  sessionGet: () => notImplemented("sessionGet"),
  sessionSet: () => notImplemented("sessionSet"),
  settingsRead: () => notImplemented("settingsRead"),
  settingsWrite: () => notImplemented("settingsWrite"),
  defaultProjectsDir: () => notImplemented("defaultProjectsDir"),

  // ---- Native dialogs ----
  // TODO(T-051): dialogs of @tauri-apps/plugin-dialog and @tauri-apps/plugin-opener (07 §6.4).
  pickProjectFile: () => notImplemented("pickProjectFile"),
  pickFolder: () => notImplemented("pickFolder"),
  pickAudioFile: () => notImplemented("pickAudioFile"),
  pickExportPath: () => notImplemented("pickExportPath"),
  revealInExplorer: () => notImplemented("revealInExplorer"),

  // ---- Audio files ----
  // TODO(T-071): audio_import, audio_discard and peaks_get. TODO(T-101): export_*.
  audioImport: () => notImplemented("audioImport"),
  audioDiscard: () => notImplemented("audioDiscard"),
  peaksGet: () => notImplemented("peaksGet"),
  exportStart: () => notImplemented("exportStart"),
  exportCancel: () => notImplemented("exportCancel"),

  // ---- Engine ----
  engineSetProject: (snapshot) => call("engine_set_project", { snapshot }),
  enginePlay: (request) => call("engine_play", { request }),
  engineStop: () => call("engine_stop"),
  engineSeek: (frame) => call("engine_seek", { frame }),
  engineSetLoop: (loop) => call("engine_set_loop", { loop }),
  engineSetMetronome: (enabled) => call("engine_set_metronome", { enabled }),
  engineAudition: (trackId, row) => call("engine_audition", { trackId, row }),
  // TODO(T-086): engine_record_start, devices_list, devices_select and calibration_run.
  engineRecordStart: () => notImplemented("engineRecordStart"),
  engineInputOpen: () => call("engine_input_open"),
  engineInputClose: () => call("engine_input_close"),
  devicesList: () => notImplemented("devicesList"),
  devicesSelect: () => notImplemented("devicesSelect"),
  calibrationRun: () => notImplemented("calibrationRun"),
  engineTestTone: () => call("engine_test_tone"),

  // ---- System ----
  appInfo: () => call("app_info"),
  logWrite: (level, message) => call("log_write", { level, message }),
  // TODO(T-051): window functions (07 §6.4).
  setWindowTitle: () => notImplemented("setWindowTitle"),
  closeWindow: () => notImplemented("closeWindow"),

  // ---- Events ----
  onTelemetry: (callback) => subscribe("engine://telemetry", callback),
  // TODO(T-086): engine://recording-finished and engine://devices-changed.
  onRecordingFinished: noEvent,
  onDevicesChanged: noEvent,
  // TODO(T-111): engine://error.
  onEngineError: noEvent,
  // TODO(T-076): engine://audio-unavailable.
  onAudioUnavailable: noEvent,
  // TODO(T-071): import://progress and the file drop. TODO(T-101): export://progress.
  onImportProgress: noEvent,
  onExportProgress: noEvent,
  onFileDrop: noEvent,
  // TODO(T-051): close request of the window.
  onCloseRequested: noEvent,
};
