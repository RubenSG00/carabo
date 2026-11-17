// The open project and its undo history (04 §3.2). It is the only place where the project
// lives (AR-01), and it only changes the document through the functions of core/commands/.
// TODO(T-013): beginGesture, updateGesture, endGesture, cancelGesture, undo, redo, setView,
// addMissingFiles, markSaved, isDirty, canUndo and canRedo.
import { createDraft, enablePatches, finishDraft, type Patch } from "immer";
import { create } from "zustand";
import { HISTORY_LIMIT } from "../core/constants";
import { newProject } from "../core/factory";
import type { ProjectDoc, ProjectFile, ViewState } from "../core/model";

enablePatches();

/** Translation key that names a history entry (03 §14.3): "history.toggleStep"… */
export type HistoryLabel = `history.${string}`;

export interface HistoryEntry {
  label: HistoryLabel;
  patches: Patch[];
  inversePatches: Patch[];
}

export interface Gesture {
  label: HistoryLabel;
  baseDoc: ProjectDoc;
  pending: HistoryEntry | null;
}

export interface ProjectState {
  doc: ProjectDoc;
  view: ViewState;
  /** Project folder. */
  dir: string;
  /** True while the project has never been saved. */
  isTemp: boolean;
  missingFiles: string[];
  past: HistoryEntry[];
  future: HistoryEntry[];
  savedIndex: number | null;
  gesture: Gesture | null;

  /**
   * Applies `recipe`, an operation of core/commands/, as one history entry, and returns what
   * the operation returns: the id it created or the reason it did not apply.
   */
  commit: <T>(label: HistoryLabel, recipe: (draft: ProjectDoc) => T) => T;
  /** Replaces the whole project and empties the history. */
  load: (file: ProjectFile, dir: string, isTemp: boolean, missingFiles: string[]) => void;
}

/** The state of a project just loaded: `file` in `dir`, with no history. */
function loaded(file: ProjectFile, dir: string, isTemp: boolean, missingFiles: string[]) {
  const { view, ...doc } = file;
  return {
    doc,
    view,
    dir,
    isTemp,
    missingFiles,
    past: [],
    future: [],
    savedIndex: 0,
    gesture: null,
  };
}

// Until a project is created or opened (T-053), the store holds an empty one with no folder.
export const useProjectStore = create<ProjectState>()((set, get) => ({
  ...loaded(newProject(), "", true, []),

  commit(label, recipe) {
    const { doc, gesture, past, savedIndex } = get();
    if (gesture !== null) throw new Error(`commit("${label}") during a gesture`);

    const draft = createDraft(doc);
    const result = recipe(draft);
    let patches: Patch[] = [];
    let inversePatches: Patch[] = [];
    const next = finishDraft(draft, (p, inverse) => {
      patches = p;
      inversePatches = inverse;
    });
    if (patches.length === 0) return result;

    const entries = [...past, { label, patches, inversePatches }];
    let saved = savedIndex;
    if (entries.length > HISTORY_LIMIT) {
      entries.shift();
      if (saved !== null) saved = saved > 0 ? saved - 1 : null;
    }
    set({ doc: next, past: entries, future: [], savedIndex: saved });
    return result;
  },

  load(file, dir, isTemp, missingFiles) {
    set(loaded(file, dir, isTemp, missingFiles));
  },
}));
