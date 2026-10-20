import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { AppUpdateProvider, useAppUpdates } from "./AppUpdateContext";
import type { AppUpdateInfo } from "@/types";
import type { AppUpdateProgress } from "@/api/tauri";

const api = vi.hoisted(() => ({ checkAppUpdate: vi.fn(), installAppUpdate: vi.fn() }));
const notifications = vi.hoisted(() => ({ info: vi.fn(), success: vi.fn(), error: vi.fn() }));
vi.mock("@/api/tauri", () => api);
vi.mock("sonner", () => ({ toast: notifications }));
vi.mock("react-i18next", () => ({ useTranslation: () => ({ t: (key: string) => key }) }));

function Controls() {
  const updates = useAppUpdates();
  return <>
    <button onClick={() => void updates.handleCheckForUpdates()}>Check</button>
    <button onClick={() => void updates.handleInstallUpdate()}>Install</button>
    <output>{updates.latestAvailableVersion ?? "none"}</output>
    <p>{updates.updateProgress?.phase}</p>
  </>;
}
const release: AppUpdateInfo = { available: true, currentVersion: "1.0.0", latestVersion: "1.1.0" };

beforeEach(() => { vi.clearAllMocks(); vi.useFakeTimers(); api.checkAppUpdate.mockResolvedValue(release); });
afterEach(() => { vi.useRealTimers(); });

describe("background updates", () => {
  it("checks at startup, throttles focus, checks hourly, and removes its timer", async () => {
    const ui = render(<AppUpdateProvider><Controls /></AppUpdateProvider>);
    await act(async () => {});
    expect(api.checkAppUpdate).toHaveBeenCalledTimes(1);
    expect(screen.getByText("1.1.0")).toBeInTheDocument();
    expect(notifications.info).not.toHaveBeenCalled();
    await act(async () => { window.dispatchEvent(new Event("focus")); window.dispatchEvent(new Event("online")); });
    expect(api.checkAppUpdate).toHaveBeenCalledTimes(1);
    await act(async () => { vi.advanceTimersByTime(60 * 60 * 1000); });
    expect(api.checkAppUpdate).toHaveBeenCalledTimes(2);
    ui.unmount();
    await act(async () => { vi.advanceTimersByTime(60 * 60 * 1000); window.dispatchEvent(new Event("focus")); });
    expect(api.checkAppUpdate).toHaveBeenCalledTimes(2);
  });

  it("deduplicates checks and silently preserves a known release on network failure", async () => {
    let resolve!: (info: AppUpdateInfo) => void;
    api.checkAppUpdate.mockReturnValueOnce(new Promise<AppUpdateInfo>(done => { resolve = done; }));
    render(<AppUpdateProvider><Controls /></AppUpdateProvider>);
    fireEvent.click(screen.getByText("Check"));
    expect(api.checkAppUpdate).toHaveBeenCalledTimes(1);
    await act(async () => resolve(release));
    api.checkAppUpdate.mockRejectedValueOnce(new Error("offline"));
    await act(async () => { vi.advanceTimersByTime(60 * 60 * 1000); });
    expect(screen.getByText("1.1.0")).toBeInTheDocument();
    expect(notifications.error).not.toHaveBeenCalled();
    expect(api.installAppUpdate).not.toHaveBeenCalled();
  });

  it("keeps installation and progress alive when the visible page unmounts", async () => {
    let progress!: (value: AppUpdateProgress) => void;
    let finish!: () => void;
    api.installAppUpdate.mockImplementation((_version, callback) => {
      progress = callback;
      return new Promise<void>(resolve => { finish = resolve; });
    });
    const ui = render(<AppUpdateProvider><Controls /></AppUpdateProvider>);
    await act(async () => {});
    fireEvent.click(screen.getByText("Install"));
    fireEvent.click(screen.getByText("Install"));
    expect(api.installAppUpdate).toHaveBeenCalledTimes(1);
    ui.rerender(<AppUpdateProvider><div>Another page</div></AppUpdateProvider>);
    act(() => progress({ phase: "installing", downloaded: 100, total: 100 }));
    ui.rerender(<AppUpdateProvider><Controls /></AppUpdateProvider>);
    expect(screen.getByText("installing")).toBeInTheDocument();
    await act(async () => finish());
    expect(screen.getByText("none")).toBeInTheDocument();
  });
});
