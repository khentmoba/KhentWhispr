import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import AppearanceSettingsSection from "./AppearanceSettingsSection";

vi.mock("@/hooks/useTheme", () => ({
  useTheme: () => ({ isDark: false, theme: "system", setTheme: vi.fn() }),
}));
vi.mock("@/hooks/useLiveTranscript", () => ({
  useLiveTranscript: () => [true, vi.fn()],
}));
vi.mock("react-i18next", () => ({ useTranslation: () => ({ t: (key: string) => key }) }));

describe("Appearance settings", () => {
  it("keeps themes and live transcription without a UI locale selector", () => {
    render(<AppearanceSettingsSection />);
    expect(screen.getAllByRole("button")).toHaveLength(3);
    expect(screen.getByRole("switch")).toBeInTheDocument();
    expect(screen.queryByRole("listbox")).not.toBeInTheDocument();
    expect(screen.queryByLabelText("settings.language")).not.toBeInTheDocument();
    expect(document.body.textContent).not.toMatch(/\p{Script=Han}/u);
  });
});
