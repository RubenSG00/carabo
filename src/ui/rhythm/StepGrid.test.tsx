import { act, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18next from "i18next";
import { createRhythmTrack } from "../../core/commands/tracks";
import { newProject } from "../../core/factory";
import { soundNames } from "../../core/kits";
import type { RhythmTrack } from "../../core/model";
import { listenToBridge, useEngineStore, type EngineState } from "../../store/engineStore";
import { useProjectStore } from "../../store/projectStore";
import { StepGrid } from "./StepGrid";

type Telemetry = NonNullable<EngineState["telemetry"]>;

/** A telemetry of the engine: stopped, with the changes given. */
function telemetry(changes: Partial<Telemetry>): Telemetry {
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
    ...changes,
  };
}

// What the engine reports reaches engineStore through its bridge event handler.
let emit: (t: Telemetry) => void = () => undefined;
let stopListening: () => void = () => undefined;

/** Creates a rhythm track in an empty project and returns it. */
function projectWithRhythmTrack(): RhythmTrack {
  useProjectStore.getState().load(newProject(), "", true, []);
  const trackId = useProjectStore.getState().commit("history.createRhythmTrack", (draft) =>
    createRhythmTrack(
      draft,
      (n) => `Ritmo ${n}`,
      (n) => `Patrón ${n}`,
    ),
  );
  const track = useProjectStore.getState().doc.tracks.find((t) => t.id === trackId);
  if (track?.type !== "rhythm") throw new Error("the rhythm track was not created");
  return track;
}

/** The level of a step of the first pattern of the first track. */
function level(row: number, step: number): number | undefined {
  const track = useProjectStore.getState().doc.tracks[0];
  return track?.type === "rhythm" ? track.patterns[0]?.steps[row]?.[step] : undefined;
}

/** Renders the grid of the first pattern of a new rhythm track. */
function renderGrid() {
  const track = projectWithRhythmTrack();
  return render(<StepGrid trackId={track.id} patternId={track.patterns[0]!.id} />);
}

beforeEach(async () => {
  await i18next.changeLanguage("es");
  useEngineStore.setState({ telemetry: null });
  stopListening = listenToBridge({
    onTelemetry: (callback) => {
      emit = callback;
      return () => {
        emit = () => undefined;
      };
    },
  });
});

afterEach(() => {
  stopListening();
});

describe("StepGrid", () => {
  it("shows 8 rows named after the sounds of the kit, with 16 steps each in a 1-bar 4/4 pattern", () => {
    renderGrid();
    const rows = within(screen.getByRole("grid")).getAllByRole("row");
    expect(rows.map((row) => within(row).getByRole("rowheader").textContent)).toEqual(
      soundNames("acoustic", "es"),
    );
    for (const row of rows) {
      expect(within(row).getAllByRole("gridcell")).toHaveLength(16);
      expect(within(row).getAllByRole("button")).toHaveLength(16);
    }
  });

  it("names each step after its sound and its number, from 1", () => {
    renderGrid();
    const first = within(screen.getAllByRole("row")[0]!).getAllByRole("button");
    expect(first[0]).toHaveAccessibleName("Bombo, paso 1");
    expect(first[15]).toHaveAccessibleName("Bombo, paso 16");
  });

  it("a click marks a level 2 hit and a second click removes it, as two history entries", async () => {
    const user = userEvent.setup();
    renderGrid();
    const step = screen.getByRole("button", { name: "Caja, paso 5" });
    expect(step).toHaveAttribute("aria-pressed", "false");

    await user.click(step);
    expect(level(1, 4)).toBe(2);
    expect(step).toHaveAttribute("aria-pressed", "true");

    await user.click(step);
    expect(level(1, 4)).toBe(0);
    expect(step).toHaveAttribute("aria-pressed", "false");
    expect(useProjectStore.getState().past.map((entry) => entry.label)).toEqual([
      "history.createRhythmTrack",
      "history.toggleStep",
      "history.toggleStep",
    ]);
  });

  it("toggles a step with the keyboard", async () => {
    const user = userEvent.setup();
    renderGrid();
    screen.getByRole("button", { name: "Bombo, paso 3" }).focus();
    await user.keyboard("{Enter}");
    expect(level(0, 2)).toBe(2);
    await user.keyboard(" ");
    expect(level(0, 2)).toBe(0);
  });

  it("highlights the column that sounds in pattern mode", () => {
    renderGrid();
    act(() => emit(telemetry({ state: "playing", mode: "pattern", patternStep: 4 })));
    for (const row of screen.getAllByRole("row")) {
      within(row)
        .getAllByRole("button")
        .forEach((button, step) => {
          if (step === 4) expect(button).toHaveClass("is-playing");
          else expect(button).not.toHaveClass("is-playing");
        });
    }
    act(() => emit(telemetry({ state: "playing", mode: "pattern", patternStep: 5 })));
    expect(screen.getByRole("button", { name: "Bombo, paso 6" })).toHaveClass("is-playing");
    expect(screen.getByRole("button", { name: "Bombo, paso 5" })).not.toHaveClass("is-playing");
  });

  it("highlights no column while stopped or in song mode", () => {
    renderGrid();
    act(() => emit(telemetry({})));
    expect(document.querySelectorAll(".is-playing")).toHaveLength(0);
    act(() => emit(telemetry({ state: "playing", mode: "song", positionFrame: 4800 })));
    expect(document.querySelectorAll(".is-playing")).toHaveLength(0);
  });

  it("names its rows and steps in English", async () => {
    await i18next.changeLanguage("en");
    renderGrid();
    expect(screen.getAllByRole("rowheader").map((header) => header.textContent)).toEqual(
      soundNames("acoustic", "en"),
    );
    expect(screen.getByRole("button", { name: "Kick, step 1" })).toBeInTheDocument();
  });
});
