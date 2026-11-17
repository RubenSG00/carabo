import { act, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18next from "i18next";
// eslint-disable-next-line no-restricted-imports -- 08 §5.1: the test reads what mockBridge received (DI-39).
import { mockControl } from "../../bridge/mockBridge";
import { newProject } from "../../core/factory";
import { listenToBridge, useEngineStore, type EngineState } from "../../store/engineStore";
import { useProjectStore } from "../../store/projectStore";
import { useUiStore } from "../../store/uiStore";
import { App } from "./App";

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

/**
 * Runs the animation frames that engineSync has requested, until it has sent what is pending.
 * The clock is simulated, so the test does not wait in real time (08 §1).
 */
function runFrames() {
  act(() => {
    vi.runAllTimers();
  });
}

beforeEach(async () => {
  // Before any change of the project, so that engineSync asks the simulated clock for its frames.
  // Only the animation frames are simulated: Testing Library still needs the real setTimeout.
  vi.useFakeTimers({ toFake: ["requestAnimationFrame", "cancelAnimationFrame"] });
  await i18next.changeLanguage("es");
  useProjectStore.getState().load(newProject(), "", true, []);
  useUiStore.getState().setTab("rhythm");
  useUiStore.getState().selectPattern(null, null);
  useEngineStore.setState({ telemetry: null });
  stopListening = listenToBridge({
    onTelemetry: (callback) => {
      emit = callback;
      return () => {
        emit = () => undefined;
      };
    },
  });
  // The snapshot of the project just loaded.
  runFrames();
  mockControl.clearCalls();
});

afterEach(() => {
  // No animation frame of engineSync may stay pending on a clock that is thrown away.
  runFrames();
  vi.useRealTimers();
  stopListening();
});

describe("App", () => {
  it("shows the top bar, the transport bar and the Rhythm tab, from top to bottom", () => {
    render(<App />);
    const tablist = screen.getByRole("tablist");
    const play = screen.getByRole("button", { name: "Reproducir" });
    const panel = screen.getByRole("tabpanel", { name: "Ritmo" });
    expect(tablist.compareDocumentPosition(play)).toBe(Node.DOCUMENT_POSITION_FOLLOWING);
    expect(play.compareDocumentPosition(panel)).toBe(Node.DOCUMENT_POSITION_FOLLOWING);
    expect(within(panel).getByRole("button", { name: "Nueva pista de ritmo" })).toBeEnabled();
  });

  it("shows the tab that is selected, and only it, with the transport bar in all of them", async () => {
    const user = userEvent.setup();
    render(<App />);

    await user.click(screen.getByRole("tab", { name: "Canción" }));
    expect(screen.getAllByRole("tabpanel")).toHaveLength(1);
    expect(screen.getByRole("tabpanel", { name: "Canción" })).toBeEmptyDOMElement();
    expect(screen.queryByRole("button", { name: "Nueva pista de ritmo" })).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Reproducir" })).toBeInTheDocument();

    await user.click(screen.getByRole("tab", { name: "Mezcla" }));
    expect(screen.getByRole("tabpanel", { name: "Mezcla" })).toBeEmptyDOMElement();
    expect(screen.getByRole("button", { name: "Reproducir" })).toBeInTheDocument();

    await user.click(screen.getByRole("tab", { name: "Ritmo" }));
    expect(screen.getByRole("button", { name: "Nueva pista de ritmo" })).toBeInTheDocument();
  });

  it("after «New rhythm track», three clicks on the grid and Play, the bridge has a snapshot with those three hits and a pattern-mode enginePlay; after Stop, an engineStop", async () => {
    const user = userEvent.setup();
    render(<App />);

    await user.click(screen.getByRole("button", { name: "Nueva pista de ritmo" }));
    await user.click(screen.getByRole("button", { name: "Bombo, paso 1" }));
    await user.click(screen.getByRole("button", { name: "Caja, paso 5" }));
    await user.click(screen.getByRole("button", { name: "Charles cerrado, paso 3" }));
    runFrames();
    await user.click(screen.getByRole("button", { name: "Reproducir" }));

    const { selectedRhythmTrackId: trackId, selectedPatternId: patternId } = useUiStore.getState();
    // 1 bar of 4/4: 8 rows × 16 steps; each click makes a level 2 hit (RF-205).
    const steps = Array.from({ length: 8 }, () => Array<number>(16).fill(0));
    steps[0]![0] = 2;
    steps[1]![4] = 2;
    steps[3]![2] = 2;
    expect(mockControl.calls.map((call) => call.method)).toEqual([
      "engineSetProject",
      "enginePlay",
    ]);
    expect(mockControl.calls[0]?.args[0]).toMatchObject({
      tracks: [{ id: trackId, type: "rhythm", patterns: [{ id: patternId, steps }] }],
    });
    expect(mockControl.calls[1]?.args).toEqual([{ mode: "pattern", trackId, patternId }]);

    mockControl.clearCalls();
    act(() => emit(telemetry({ state: "playing", mode: "pattern", patternStep: 2 })));
    // The step that sounds is highlighted from the telemetry.
    expect(screen.getByRole("button", { name: "Charles cerrado, paso 3" })).toHaveClass(
      "is-playing",
    );
    await user.click(screen.getByRole("button", { name: "Parar" }));
    expect(mockControl.calls).toEqual([{ method: "engineStop", args: [] }]);
  });

  it("shows its texts in English", async () => {
    await i18next.changeLanguage("en");
    const user = userEvent.setup();
    render(<App />);
    expect(screen.getAllByRole("tab").map((tab) => tab.textContent)).toEqual([
      "Rhythm",
      "Song",
      "Mix",
    ]);
    expect(screen.getByRole("tabpanel", { name: "Rhythm" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "New rhythm track" }));
    expect(screen.getByRole("button", { name: "Play" })).toBeEnabled();
    expect(screen.getByRole("button", { name: "Kick, step 1" })).toBeInTheDocument();
  });
});
