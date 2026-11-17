// The transport bar (03 §6). For now it only has Play/Stop (RF-101).
// TODO(T-025): Record, what is playing, the position, Tempo, Time signature, Metronome and Loop,
// and the explanation of each control, also when it is disabled (RF-008).
// TODO(T-026): the Space shortcut.
import { useTranslation } from "../../i18n";
import { play, stop } from "../../services/playback";
import { useEngineStore } from "../../store/engineStore";
import { useUiStore } from "../../store/uiStore";
import "./TransportBar.css";

export function TransportBar() {
  const { t } = useTranslation();
  // RF-101: the button reads «Stop» while the engine plays, counts in or records.
  const running = useEngineStore((s) => s.telemetry !== null && s.telemetry.state !== "stopped");
  // RF-102: in the Rhythm tab, Play needs a pattern to play.
  // TODO(T-064): play the song in the Song and Mix tabs. Until then, Play is disabled there
  // (09 §1.3, rule 5).
  const canPlay = useUiStore((s) => s.tab === "rhythm" && s.selectedPatternId !== null);

  const onClick = () => {
    (running ? stop() : play()).catch(() => {
      // TODO(T-024): show the message of the bridge error (04 §6.5).
    });
  };

  return (
    <div className="TransportBar">
      <button
        type="button"
        className="TransportBar-play"
        disabled={!running && !canPlay}
        onClick={onClick}
      >
        {t(running ? "transport.stop.label" : "transport.play.label")}
      </button>
    </div>
  );
}
