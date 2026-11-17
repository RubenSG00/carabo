import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18next from "i18next";
import { newProject } from "../../core/factory";
import type { RhythmTrack } from "../../core/model";
import { useProjectStore } from "../../store/projectStore";
import { useUiStore } from "../../store/uiStore";
import { RhythmTab } from "./RhythmTab";

/** The rhythm tracks of the open project. */
function rhythmTracks(): RhythmTrack[] {
  return useProjectStore
    .getState()
    .doc.tracks.filter((track): track is RhythmTrack => track.type === "rhythm");
}

beforeEach(async () => {
  await i18next.changeLanguage("es");
  useProjectStore.getState().load(newProject(), "", true, []);
  useUiStore.getState().selectPattern(null, null);
});

describe("RhythmTab", () => {
  it("in an empty project shows «New rhythm track» and no grid", () => {
    render(<RhythmTab />);
    expect(screen.getByRole("button", { name: "Nueva pista de ritmo" })).toBeEnabled();
    expect(screen.queryByRole("grid")).not.toBeInTheDocument();
  });

  it("creates «Ritmo 1» with «Patrón 1» as one history entry, selects it and shows its grid", async () => {
    const user = userEvent.setup();
    render(<RhythmTab />);
    await user.click(screen.getByRole("button", { name: "Nueva pista de ritmo" }));

    const [track] = rhythmTracks();
    expect(track?.name).toBe("Ritmo 1");
    expect(track?.patterns.map((pattern) => pattern.name)).toEqual(["Patrón 1"]);
    expect(useProjectStore.getState().past.map((entry) => entry.label)).toEqual([
      "history.createRhythmTrack",
    ]);
    const { selectedRhythmTrackId, selectedPatternId } = useUiStore.getState();
    expect(selectedRhythmTrackId).toBe(track?.id);
    expect(selectedPatternId).toBe(track?.patterns[0]?.id);
    expect(screen.getByRole("grid")).toBeInTheDocument();
  });

  it("selects each new track: a second one is «Ritmo 2»", async () => {
    const user = userEvent.setup();
    render(<RhythmTab />);
    const button = screen.getByRole("button", { name: "Nueva pista de ritmo" });
    await user.click(button);
    await user.click(button);

    const tracks = rhythmTracks();
    expect(tracks.map((track) => track.name)).toEqual(["Ritmo 1", "Ritmo 2"]);
    expect(useUiStore.getState().selectedRhythmTrackId).toBe(tracks[1]?.id);
    expect(useUiStore.getState().selectedPatternId).toBe(tracks[1]?.patterns[0]?.id);
  });

  it("creates a rhythm track with the keyboard", async () => {
    const user = userEvent.setup();
    render(<RhythmTab />);
    screen.getByRole("button", { name: "Nueva pista de ritmo" }).focus();
    await user.keyboard("{Enter}");
    expect(rhythmTracks()).toHaveLength(1);
  });

  it("shows its texts and names the new track and pattern in English", async () => {
    await i18next.changeLanguage("en");
    const user = userEvent.setup();
    render(<RhythmTab />);
    await user.click(screen.getByRole("button", { name: "New rhythm track" }));
    const [track] = rhythmTracks();
    expect(track?.name).toBe("Rhythm 1");
    expect(track?.patterns[0]?.name).toBe("Pattern 1");
  });
});
