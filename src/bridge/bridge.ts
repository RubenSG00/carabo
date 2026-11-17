// The bridge between the interface and the outside world: the `Bridge` interface and the types
// of its arguments, results, errors and events, exactly as in 04 §6.1, §6.3 and §6.4.
// `Session` is the one of 05 §7.2; `EngineSnapshot` is in core/model.ts (05 §10.1).

import type { EngineSnapshot } from "../core/model";

export type Unsubscribe = () => void;
export type ExportFormat = "wav" | "mp3";

export interface Bridge {
  // ---- Project files ----
  projectCreateTemp(): Promise<{ dir: string }>;
  projectRead(file: string): Promise<{ dir: string; json: string }>;
  projectCheckFiles(dir: string, files: string[]): Promise<{ missing: string[] }>;
  projectCleanup(dir: string, keepFiles: string[]): Promise<void>;
  projectWrite(dir: string, json: string): Promise<void>;
  projectSaveAs(request: SaveAsRequest): Promise<{ dir: string }>;
  projectDiscardTemp(dir: string): Promise<void>;
  recoveryWrite(dir: string, json: string): Promise<void>;
  recoveryRead(dir: string): Promise<{ json: string; modifiedAt: string } | null>;
  recoveryDelete(dir: string): Promise<void>;
  sessionGet(): Promise<Session | null>;
  sessionSet(session: Session): Promise<void>;
  settingsRead(): Promise<string | null>;
  settingsWrite(json: string): Promise<void>;
  defaultProjectsDir(): Promise<string>;

  // ---- Native dialogs ----
  pickProjectFile(): Promise<string | null>;
  pickFolder(startDir: string): Promise<string | null>;
  pickAudioFile(): Promise<string | null>;
  pickExportPath(defaultName: string, format: ExportFormat): Promise<string | null>;
  revealInExplorer(path: string): Promise<void>;

  // ---- Audio files ----
  audioImport(srcPath: string, projectDir: string): Promise<ImportedAudio>;
  audioDiscard(projectDir: string, file: string): Promise<void>;
  peaksGet(projectDir: string, file: string): Promise<ArrayBuffer>;
  exportStart(request: ExportRequest): Promise<ExportResult>;
  exportCancel(): Promise<void>;

  // ---- Engine ----
  engineSetProject(snapshot: EngineSnapshot): Promise<void>;
  enginePlay(request: PlayRequest): Promise<void>;
  engineStop(): Promise<void>;
  engineSeek(frame: number): Promise<void>;
  engineSetLoop(loop: LoopRequest): Promise<void>;
  engineSetMetronome(enabled: boolean): Promise<void>;
  engineAudition(trackId: string, row: number): Promise<void>;
  engineRecordStart(request: RecordRequest): Promise<{ file: string }>;
  engineInputOpen(): Promise<void>;
  engineInputClose(): Promise<void>;
  devicesList(): Promise<DeviceList>;
  devicesSelect(outputName: string | null, inputName: string | null): Promise<ActiveDevices>;
  calibrationRun(): Promise<{ latencyFrames: number }>;
  engineTestTone(): Promise<void>;

  // ---- System ----
  appInfo(): Promise<{ version: string; systemLocale: string }>;
  logWrite(level: "error" | "warn" | "info", message: string): Promise<void>;
  setWindowTitle(title: string): Promise<void>;
  closeWindow(): Promise<void>;

  // ---- Events ----
  onTelemetry(callback: (t: Telemetry) => void): Unsubscribe;
  onRecordingFinished(callback: (r: RecordingFinished) => void): Unsubscribe;
  onDevicesChanged(callback: (d: DevicesChanged) => void): Unsubscribe;
  onEngineError(callback: (e: { detail: string }) => void): Unsubscribe;
  onAudioUnavailable(callback: (e: { files: string[] }) => void): Unsubscribe;
  onImportProgress(callback: (p: { percent: number }) => void): Unsubscribe;
  onExportProgress(callback: (p: { percent: number }) => void): Unsubscribe;
  onFileDrop(callback: (d: { paths: string[]; x: number; y: number }) => void): Unsubscribe;
  onCloseRequested(callback: () => void): Unsubscribe;
}

export interface SaveAsRequest {
  fromDir: string; // current folder of the project (temporary or saved)
  parentDir: string; // chosen location
  name: string; // name of the project
  json: string; // content of project.carabo
  files: string[]; // audio files that are copied, relative: "audio/take-0001.wav"
  deleteFromDir: boolean; // true if fromDir is a temporary folder and must be deleted at the end
}
export interface ImportedAudio {
  file: string;
  frames: number;
  channels: 1 | 2;
  truncated: boolean;
}
export interface ExportRequest {
  snapshot: EngineSnapshot;
  format: ExportFormat;
  path: string;
}
export interface ExportResult {
  clippedSamples: number;
  frames: number;
}

export type PlayRequest =
  { mode: "song"; startFrame: number } | { mode: "pattern"; trackId: string; patternId: string };
export interface LoopRequest {
  enabled: boolean;
  startFrame: number;
  endFrame: number;
}
export interface RecordRequest {
  trackId: string;
  startFrame: number;
  projectDir: string;
}

export interface DeviceList {
  outputs: { name: string; isDefault: boolean }[];
  inputs: { name: string; isDefault: boolean }[];
}
export interface ActiveDevices {
  outputName: string | null; // real name in use, or null if there is no output
  inputName: string | null; // real name of the device that will be used as input, or null if there is none
  outputFellBack: boolean; // true if the requested device does not exist and the default one is used
  inputFellBack: boolean;
}

// ---- Errors (04 §6.3) ----

export interface BridgeError {
  code: BridgeErrorCode;
  detail: string; // technical text for the log; it is never shown to the user
}
export type BridgeErrorCode =
  | "IO"
  | "NOT_FOUND"
  | "ALREADY_EXISTS"
  | "NAME_INVALID"
  | "UNSUPPORTED_FORMAT"
  | "BUSY"
  | "CANCELLED"
  | "NOTHING_TO_EXPORT"
  | "NO_OUTPUT_DEVICE"
  | "NO_INPUT_DEVICE"
  | "CALIBRATION_FAILED"
  | "INVALID_SNAPSHOT"
  | "ENGINE_INTERNAL";

// ---- Events (04 §6.4) ----

export interface Telemetry {
  state: "stopped" | "countIn" | "playing" | "recording";
  mode: "song" | "pattern" | null; // null when stopped
  positionFrame: number; // song mode: frame of the song; pattern mode: frame from the start of the pattern
  patternStep: number | null; // pattern mode: step that is sounding
  trackPeaks: Record<string, [number, number]>; // by track id: left and right peak since the previous telemetry, as linear amplitude
  masterPeak: [number, number];
  inputPeak: number | null; // null if the input is closed
  xruns: number; // audio dropouts since the last playback started
  recordedFrames: number | null; // frames recorded so far, or null when not recording
  outputAvailable: boolean; // false if there is no output device
}

export interface RecordingFinished {
  file: string; // relative path: "audio/take-0003.wav"
  frames: number; // frames recorded; 0 if the count-in was cancelled
  trackId: string;
  startFrame: number; // frame of the song where the recording started
  reason: "stopped" | "cancelled" | "songLimit" | "inputLost" | "outputLost" | "diskError";
}

export interface DevicesChanged {
  lostOutputName: string | null; // output device that has disappeared, if any
  lostInputName: string | null;
  active: ActiveDevices; // what remains in use
}

// ---- Session (05 §7.2) ----

export interface Session {
  projectDir: string | null; // folder of the open project, or null if there is none
  isTemp: boolean; // true if it is a project that has never been saved
  cleanExit: boolean; // false while the program is open; true when it closes properly
}
