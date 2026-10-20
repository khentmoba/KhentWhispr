import { useEffect, useState } from "react";
import { getVersion } from "@tauri-apps/api/app";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, X } from "lucide-react";
import { useTranslation } from "react-i18next";
import { hideMainWindow } from "@/api/tauri";
import TitleBar from "@/components/TitleBar";
import AppUpdatePanel from "@/components/settings/AppUpdatePanel";

export default function UpdatesPage({ animClass = "" }: { animClass?: string }) {
  const { t } = useTranslation();
  const [version, setVersion] = useState("");
  useEffect(() => { void getVersion().then(setVersion).catch(() => {}); }, []);
  return (
    <div className="page-root">
      <TitleBar title={t("updates.title")} rightActions={<>
        <button aria-label={t("common.minimize")} className="icon-btn" onClick={() => void getCurrentWindow().minimize()}><Minus size={12} /></button>
        <button aria-label={t("common.close")} className="icon-btn" onClick={() => void hideMainWindow()}><X size={12} /></button>
      </>} />
      <div className={`page-content updates-content ${animClass}`}>
        <header className="workspace-heading"><div>
          <h1>{t("updates.heading")}</h1>
          <p>{t("updates.subheading")}</p>
        </div></header>
        <AppUpdatePanel appVersion={version} />
      </div>
    </div>
  );
}
