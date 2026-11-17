// The step grid (03 §8.4, RF-204, RF-205): the selected pattern, with one row per sound of the
// kit and one button per step. A click on a step toggles its hit.
// TODO(T-031): one Tab stop with the arrows, Home and End; the change on pressing the mouse
// button; the sound of a new hit while stopped; the row headers of 03 §8.1; the groups by beat
// and bar; and the column that sounds only when the pattern that sounds is the one shown.
import { toggleStep } from "../../core/commands/patterns";
import { soundNames } from "../../core/kits";
import { useLanguage, useTranslation } from "../../i18n";
import { useEngineStore } from "../../store/engineStore";
import { useProjectStore } from "../../store/projectStore";
import "./StepGrid.css";

interface StepGridProps {
  trackId: string;
  patternId: string;
}

export function StepGrid({ trackId, patternId }: StepGridProps) {
  const { t } = useTranslation();
  const language = useLanguage();
  const track = useProjectStore((s) => s.doc.tracks.find((tr) => tr.id === trackId));
  const commit = useProjectStore((s) => s.commit);
  // 03 §8.4: in pattern mode, the column that sounds is highlighted.
  const playingStep = useEngineStore((s) =>
    s.telemetry?.mode === "pattern" ? s.telemetry.patternStep : null,
  );

  const pattern =
    track?.type === "rhythm" ? track.patterns.find((p) => p.id === patternId) : undefined;
  if (track?.type !== "rhythm" || pattern === undefined) return null;
  const names = soundNames(track.kitId, language);

  return (
    <div className="StepGrid" role="grid">
      {pattern.steps.map((levels, row) => (
        <div key={row} className="StepGrid-row" role="row">
          <div className="StepGrid-name" role="rowheader">
            {names[row]}
          </div>
          {levels.map((level, step) => (
            <div key={step} className="StepGrid-cell" role="gridcell">
              <button
                type="button"
                className={
                  "StepGrid-step" +
                  (level > 0 ? " is-hit" : "") +
                  (step === playingStep ? " is-playing" : "")
                }
                aria-pressed={level > 0}
                aria-label={t("a11y.step", { sound: names[row] ?? "", n: step + 1 })}
                onClick={() => {
                  commit("history.toggleStep", (draft) =>
                    toggleStep(draft, trackId, patternId, row, step),
                  );
                }}
              />
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}
