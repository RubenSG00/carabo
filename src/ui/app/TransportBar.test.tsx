import { act, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18next from "i18next";
// eslint-disable-next-line no-restricted-imports -- 08 §5.1: the test reads what mockBridge received (DI-39).
import { mockControl } from "../../bridge/mockBridge";
import { listenToBridge, useEngineStore, type EngineState } from "../../store/engineStore";
import { useUiStore } from "../../store/uiStore";
import { TransportBar } from "./TransportBar";

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

const playingPattern = telemetry({ state: "playing", mode: "pattern", patternStep: 0 });

// What the engine reports reaches engineStore through its bridge event handler.
let emit: (t: Telemetry) => void = () => undefined;
let stopListening: () => void = () => undefined;

beforeEach(async () => {
  await i18next.changeLanguage("es");
  mockControl.clearCalls();
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
});

afterEach(() => {
  stopListening();
});

describe("TransportBar", () => {
  it("disables Play in the Rhythm tab while there is no pattern to play", () => {
    render(<TransportBar />);
    expect(screen.getByRole("button", { name: "Reproducir" })).toBeDisabled();
  });

  it("plays the selected pattern in pattern mode", async () => {
    const user = userEvent.setup();
    useUiStore.getState().selectPattern("trk-1", "pat-2");
    render(<TransportBar />);
    await user.click(screen.getByRole("button", { name: "Reproducir" }));
    expect(mockControl.calls).toEqual([
      { method: "enginePlay", args: [{ mode: "pattern", trackId: "trk-1", patternId: "pat-2" }] },
    ]);
  });

  it("reads Stop while the engine plays, and stops it", async () => {
    const user = userEvent.setup();
    useUiStore.getState().selectPattern("trk-1", "pat-2");
    render(<TransportBar />);
    act(() => emit(playingPattern));
    expect(screen.queryByRole("button", { name: "Reproducir" })).not.toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Parar" }));
    expect(mockControl.calls).toEqual([{ method: "engineStop", args: [] }]);
    act(() => emit(telemetry({})));
    expect(screen.getByRole("button", { name: "Reproducir" })).toBeEnabled();
  });

  it("plays and stops with the keyboard", async () => {
    const user = userEvent.setup();
    useUiStore.getState().selectPattern("trk-1", "pat-2");
    render(<TransportBar />);
    screen.getByRole("button", { name: "Reproducir" }).focus();
    await user.keyboard("{Enter}");
    act(() => emit(playingPattern));
    screen.getByRole("button", { name: "Parar" }).focus();
    await user.keyboard(" ");
    expect(mockControl.calls.map((call) => call.method)).toEqual(["enginePlay", "engineStop"]);
  });

  it("disables Play in the Song and Mix tabs, where the song cannot play yet", () => {
    useUiStore.getState().selectPattern("trk-1", "pat-2");
    useUiStore.getState().setTab("song");
    const { rerender } = render(<TransportBar />);
    expect(screen.getByRole("button", { name: "Reproducir" })).toBeDisabled();
    act(() => useUiStore.getState().setTab("mix"));
    rerender(<TransportBar />);
    expect(screen.getByRole("button", { name: "Reproducir" })).toBeDisabled();
  });

  it("keeps Stop available after moving to the Song tab while the pattern plays", async () => {
    const user = userEvent.setup();
    useUiStore.getState().selectPattern("trk-1", "pat-2");
    render(<TransportBar />);
    act(() => emit(playingPattern));
    act(() => useUiStore.getState().setTab("song"));
    await user.click(screen.getByRole("button", { name: "Parar" }));
    expect(mockControl.calls).toEqual([{ method: "engineStop", args: [] }]);
  });

  it("shows its texts in English", async () => {
    await i18next.changeLanguage("en");
    useUiStore.getState().selectPattern("trk-1", "pat-2");
    render(<TransportBar />);
    expect(screen.getByRole("button", { name: "Play" })).toBeEnabled();
    act(() => emit(playingPattern));
    expect(screen.getByRole("button", { name: "Stop" })).toBeEnabled();
  });
});
