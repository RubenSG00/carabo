// The window (03 §3, RF-002): from top to bottom, the top bar, the transport bar and the area of
// the active tab, which takes the rest of the space.
// TODO(T-025): the status bar.
import "../../store/engineSync";
import { useUiStore } from "../../store/uiStore";
import { RhythmTab } from "../rhythm/RhythmTab";
import { TAB_PANEL_ID, TopBar, tabId } from "./TopBar";
import { TransportBar } from "./TransportBar";
import "./App.css";

export function App() {
  const tab = useUiStore((s) => s.tab);

  return (
    <div className="App">
      <TopBar />
      <TransportBar />
      <div className="App-tab" role="tabpanel" id={TAB_PANEL_ID} aria-labelledby={tabId(tab)}>
        {/* TODO(T-060): the Song tab. TODO(T-090): the Mix tab. */}
        {tab === "rhythm" && <RhythmTab />}
      </div>
    </div>
  );
}
