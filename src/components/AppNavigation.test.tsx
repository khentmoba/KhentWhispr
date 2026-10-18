import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import AppNavigation from "./AppNavigation";

vi.mock("@tauri-apps/api/window", () => ({ getCurrentWindow: vi.fn() }));

describe("workspace navigation", () => {
  it("identifies the active view and opens each existing workspace", () => {
    const onNavigate = vi.fn();
    const { rerender } = render(<AppNavigation view="main" onNavigate={onNavigate} />);
    expect(screen.getByRole("navigation", { name: "Main navigation" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Dictation" })).toHaveAttribute("aria-current", "page");
    for (const [label, view] of [["Dictation", "main"], ["History", "history"], ["Settings", "settings"]]) {
      fireEvent.click(screen.getByRole("button", { name: label }));
      expect(onNavigate).toHaveBeenLastCalledWith(view);
    }
    rerender(<AppNavigation view="settings" onNavigate={onNavigate} />);
    expect(screen.getByRole("button", { name: "Settings" })).toHaveAttribute("aria-current", "page");
    expect(screen.getByRole("button", { name: "Dictation" })).not.toHaveAttribute("aria-current");
  });
});
