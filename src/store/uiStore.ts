// State of the interface that does not belong to the project (04 §3.2): it is neither saved nor
// part of the history.
// TODO(T-024): the rest of the fields of 04 §3.2 (selection in Song, cursor, scroll, armed
// track, open dialog and status bar message).
import { create } from "zustand";

/** The tabs of 1.0, in their order (RF-002). */
export type Tab = "rhythm" | "song" | "mix";

export interface UiState {
  /** The active tab: Rhythm when a project is created or opened (RF-002). */
  tab: Tab;
  /** The rhythm track selected in the Rhythm tab, if any (RF-202). */
  selectedRhythmTrackId: string | null;
  /** The pattern of that track selected in the Rhythm tab, if any. */
  selectedPatternId: string | null;

  setTab: (tab: Tab) => void;
  /** Selects a rhythm track and one of its patterns in the Rhythm tab, or nothing. */
  selectPattern: (trackId: string | null, patternId: string | null) => void;
}

export const useUiStore = create<UiState>()((set) => ({
  tab: "rhythm",
  selectedRhythmTrackId: null,
  selectedPatternId: null,

  setTab(tab) {
    set({ tab });
  },

  selectPattern(trackId, patternId) {
    set({ selectedRhythmTrackId: trackId, selectedPatternId: patternId });
  },
}));
