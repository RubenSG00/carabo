// The top bar (03 §5). For now it only has the tabs (RF-003).
// TODO(T-025): the File menu, Undo, Redo and Settings, and the explanation of each control.
// TODO(T-026): Ctrl+1, Ctrl+2 and Ctrl+3.
import { useTranslation } from "../../i18n";
import { useUiStore, type Tab } from "../../store/uiStore";
import "./TopBar.css";

/** The tabs of 1.0, in their order (RF-003). */
const TABS: readonly Tab[] = ["rhythm", "song", "mix"];

/** The id of the element that shows the active tab. */
export const TAB_PANEL_ID = "tab-panel";

/** The id of the button of a tab, which names the panel while that tab is active. */
export function tabId(tab: Tab): string {
  return `tab-${tab}`;
}

export function TopBar() {
  const { t } = useTranslation();
  const active = useUiStore((s) => s.tab);
  const setTab = useUiStore((s) => s.setTab);

  return (
    <header className="TopBar">
      <div className="TopBar-tabs" role="tablist">
        {TABS.map((tab) => (
          <button
            key={tab}
            type="button"
            role="tab"
            id={tabId(tab)}
            aria-selected={tab === active}
            aria-controls={TAB_PANEL_ID}
            className={tab === active ? "TopBar-tab is-active" : "TopBar-tab"}
            onClick={() => setTab(tab)}
          >
            {t(`tab.${tab}.label`)}
          </button>
        ))}
      </div>
    </header>
  );
}
