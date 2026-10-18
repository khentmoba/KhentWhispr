import { useCallback, useEffect, useRef, useState } from "react";
import { getVersion } from "@tauri-apps/api/app";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";
import {
  checkAppUpdate,
  copyToClipboard,
  disableAutostart,
  enableAutostart,
  exportUserProfile,
  importUserProfile,
  isAutostartEnabled,
  installAppUpdate,
  pasteText,
  setAutostartMinimized,
} from "@/api/tauri";
import type { UserProfile } from "@/types";

interface UseSystemSettingsOptions {
  inputMethod: "sendInput" | "clipboard";
  profile: UserProfile | null;
  refreshProfile: () => Promise<UserProfile | undefined>;
}

export function useSystemSettings({
  inputMethod,
  profile,
  refreshProfile,
}: UseSystemSettingsOptions) {
  const { t } = useTranslation();
  const [autostart, setAutostart] = useState(false);
  const [autostartLoading, setAutostartLoading] = useState(true);
  const [autostartMinimizedSaving, setAutostartMinimizedSaving] = useState(false);
  const autostartMinimized = profile?.autostart_minimized ?? false;
  const [appVersion, setAppVersion] = useState("");
  const [updateChecking, setUpdateChecking] = useState(false);
  const [updateStatusText, setUpdateStatusText] = useState("");
  const [latestAvailableVersion, setLatestAvailableVersion] = useState<string | null>(null);
  const [updateInstalling, setUpdateInstalling] = useState(false);
  const updatePending = useRef(false);
  const [lastExportPath, setLastExportPath] = useState("");
  const [exporting, setExporting] = useState(false);
  const exportPending = useRef(false);

  useEffect(() => {
    void getVersion().then(setAppVersion).catch(() => {});
  }, []);

  useEffect(() => {
    void isAutostartEnabled().then((enabled) => {
      setAutostart(enabled);
      setAutostartLoading(false);
    }).catch(() => setAutostartLoading(false));
  }, []);

  const handleCheckForUpdates = useCallback(async () => {
    if (updatePending.current) return;
    updatePending.current = true;

    setUpdateChecking(true);
    setLatestAvailableVersion(null);
    setUpdateStatusText(t("toast.checkingGitHub"));

    try {
      const updateInfo = await checkAppUpdate();
      if (!updateInfo.latestVersion) {
        setUpdateStatusText(t("toast.noPublishedReleases"));
        toast.info(t("toast.noPublishedReleases"));
        return;
      }
      if (!updateInfo.available) {
        setUpdateStatusText(t("toast.alreadyLatest"));
        toast.success(t("toast.alreadyLatest"));
        return;
      }

      setLatestAvailableVersion(updateInfo.latestVersion);
      setUpdateStatusText(t("toast.newVersionFound", { version: updateInfo.latestVersion }));
      toast.info(t("toast.newVersionToast", { version: updateInfo.latestVersion }));
    } catch (error) {
      const message = error instanceof Error ? error.message : t("toast.checkUpdateFailed");
      setUpdateStatusText(message);
      toast.error(message);
    } finally {
      updatePending.current = false;
      setUpdateChecking(false);
    }
  }, [t]);

  const handleInstallUpdate = useCallback(async () => {
    if (updatePending.current || !latestAvailableVersion) return;
    updatePending.current = true;
    setUpdateChecking(true);
    setUpdateInstalling(true);
    setUpdateStatusText(t("settings.downloadingUpdate"));
    try {
      await installAppUpdate(latestAvailableVersion, (progress) => {
        setUpdateStatusText(progress.phase === "installing"
          ? t("settings.installingUpdate")
          : progress.total
            ? t("settings.updateDownloadProgress", { percent: Math.min(100, Math.floor(progress.downloaded * 100 / progress.total)) })
            : t("settings.downloadingUpdate"));
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : t("toast.installUpdateFailed");
      setUpdateStatusText(message);
      toast.error(message);
    } finally {
      updatePending.current = false;
      setUpdateChecking(false);
      setUpdateInstalling(false);
    }
  }, [latestAvailableVersion, t]);

  const handleAutostartToggle = useCallback(async () => {
    if (autostartLoading) return;
    const prev = autostart;
    const next = !prev;
    // Reflect the requested state immediately, then reconcile it with the
    // plugin's authoritative value before reporting success.
    setAutostart(next);
    setAutostartLoading(true);
    try {
      if (prev) {
        await disableAutostart();
      } else {
        await enableAutostart();
      }
      const confirmed = await isAutostartEnabled();
      setAutostart(confirmed);
      if (confirmed !== next) {
        throw new Error("Autostart state was not persisted");
      }
      toast.success(t(next ? "toast.autostartEnabled" : "toast.autostartDisabled"), {
        duration: 1100,
      });
    } catch {
      // A failed operation or unreadable confirmation leaves no authoritative
      // new value, so return to the last confirmed UI state.
      setAutostart(prev);
      toast.error(t("toast.autostartFailed"));
    } finally {
      setAutostartLoading(false);
    }
  }, [autostart, autostartLoading, t]);

  const handleAutostartMinimizedToggle = useCallback(async () => {
    if (!profile || !autostart || autostartLoading || autostartMinimizedSaving) return;
    const next = !autostartMinimized;
    setAutostartMinimizedSaving(true);
    try {
      await setAutostartMinimized(next);
      const confirmed = await refreshProfile();
      if (confirmed?.autostart_minimized !== next) {
        throw new Error("Startup preference could not be confirmed");
      }
    } catch {
      toast.error(t("toast.autostartMinimizedFailed"));
    } finally {
      setAutostartMinimizedSaving(false);
    }
  }, [profile, autostart, autostartLoading, autostartMinimized, autostartMinimizedSaving, refreshProfile, t]);

  const handleExportConfig = useCallback(async () => {
    if (exportPending.current) return;
    exportPending.current = true;
    setExporting(true);
    try {
      const path = await exportUserProfile();
      if (!path) return;
      setLastExportPath(path);
      toast.success(t("toast.configExported"));
    } catch {
      toast.error(t("toast.configExportFailed"));
    } finally {
      exportPending.current = false;
      setExporting(false);
    }
  }, [t]);

  const handleCopyExportPath = useCallback(async () => {
    if (!lastExportPath) return;
    try {
      await copyToClipboard(lastExportPath);
      toast.success(t("common.copiedToClipboard"));
    } catch {
      toast.error(t("common.copyFailed"));
    }
  }, [lastExportPath, t]);

  const handleImportConfig = useCallback(async (json: string) => {
    try {
      await importUserProfile(json);
      await refreshProfile();
      toast.success(t("toast.configImported"));
    } catch {
      toast.error(t("toast.configImportFailed"));
    }
  }, [refreshProfile, t]);

  const handleTestPaste = useCallback(async () => {
    try {
      await pasteText(t("settings.testPasteContent"), inputMethod);
      toast.success(t("toast.pasteOk"));
    } catch {
      toast.error(t("toast.pasteFailed"));
    }
  }, [inputMethod, t]);

  return {
    appVersion,
    exporting,
    autostart,
    autostartLoading,
    autostartMinimized,
    autostartMinimizedLoading: autostartLoading || autostartMinimizedSaving || !profile,
    handleAutostartToggle,
    handleAutostartMinimizedToggle,
    handleCheckForUpdates,
    handleCopyExportPath,
    handleExportConfig,
    handleImportConfig,
    handleInstallUpdate,
    handleTestPaste,
    lastExportPath,
    latestAvailableVersion,
    updateChecking,
    updateInstalling,
    updateStatusText,
  };
}
