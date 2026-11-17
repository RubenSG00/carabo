// The Rhythm tab (03 §8). For now it has «New rhythm track» and the step grid of the selected
// pattern.
// TODO(T-030): the rest of the toolbar, the pattern list and the disabled states of RF-200 and
// RF-201 (with MAX_TRACKS tracks, «New rhythm track» is disabled).
import { createRhythmTrack } from "../../core/commands/tracks";
import { useTranslation } from "../../i18n";
import { useProjectStore } from "../../store/projectStore";
import { useUiStore } from "../../store/uiStore";
import { StepGrid } from "./StepGrid";
import "./RhythmTab.css";

export function RhythmTab() {
  const { t } = useTranslation();
  const commit = useProjectStore((s) => s.commit);
  const trackId = useUiStore((s) => s.selectedRhythmTrackId);
  const patternId = useUiStore((s) => s.selectedPatternId);
  const selectPattern = useUiStore((s) => s.selectPattern);

  const onNewTrack = () => {
    const created = commit("history.createRhythmTrack", (draft) =>
      createRhythmTrack(
        draft,
        (n) => t("names.rhythmTrack", { n }),
        (n) => t("names.pattern", { n }),
      ),
    );
    if (typeof created !== "string") return;
    // RF-201: the new track is selected, with its only pattern.
    const track = useProjectStore.getState().doc.tracks.find((tr) => tr.id === created);
    if (track?.type === "rhythm") selectPattern(track.id, track.patterns[0]?.id ?? null);
  };

  return (
    <div className="RhythmTab">
      <div className="RhythmTab-toolbar">
        <button type="button" onClick={onNewTrack}>
          {t("rhythm.newTrack.label")}
        </button>
      </div>
      {trackId !== null && patternId !== null && (
        <StepGrid trackId={trackId} patternId={patternId} />
      )}
    </div>
  );
}
