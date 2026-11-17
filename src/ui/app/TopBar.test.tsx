import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18next from "i18next";
import { useUiStore } from "../../store/uiStore";
import { TopBar } from "./TopBar";

beforeEach(async () => {
  await i18next.changeLanguage("es");
  useUiStore.getState().setTab("rhythm");
});

describe("TopBar", () => {
  it("shows the three tabs in their order, with Rhythm active", () => {
    render(<TopBar />);
    const tabs = screen.getAllByRole("tab");
    expect(tabs.map((tab) => tab.textContent)).toEqual(["Ritmo", "Canción", "Mezcla"]);
    expect(tabs.map((tab) => tab.getAttribute("aria-selected"))).toEqual([
      "true",
      "false",
      "false",
    ]);
    expect(screen.getByRole("tablist")).toContainElement(tabs[0]!);
  });

  it("activates the tab that is clicked, and only that one", async () => {
    const user = userEvent.setup();
    render(<TopBar />);
    await user.click(screen.getByRole("tab", { name: "Canción" }));
    expect(useUiStore.getState().tab).toBe("song");
    expect(screen.getByRole("tab", { name: "Canción" })).toHaveAttribute("aria-selected", "true");
    expect(screen.getByRole("tab", { name: "Ritmo" })).toHaveAttribute("aria-selected", "false");
    await user.click(screen.getByRole("tab", { name: "Mezcla" }));
    expect(useUiStore.getState().tab).toBe("mix");
  });

  it("activates a tab with the keyboard", async () => {
    const user = userEvent.setup();
    render(<TopBar />);
    screen.getByRole("tab", { name: "Mezcla" }).focus();
    await user.keyboard("{Enter}");
    expect(useUiStore.getState().tab).toBe("mix");
    screen.getByRole("tab", { name: "Ritmo" }).focus();
    await user.keyboard(" ");
    expect(useUiStore.getState().tab).toBe("rhythm");
  });

  it("shows its texts in English", async () => {
    await i18next.changeLanguage("en");
    render(<TopBar />);
    expect(screen.getAllByRole("tab").map((tab) => tab.textContent)).toEqual([
      "Rhythm",
      "Song",
      "Mix",
    ]);
  });
});
