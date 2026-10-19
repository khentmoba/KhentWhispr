import { useEffect, useState } from "react";
import { getVersion } from "@tauri-apps/api/app";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, X } from "lucide-react";
import { hideMainWindow } from "@/api/tauri";
import TitleBar from "@/components/TitleBar";
import AppUpdatePanel from "@/components/settings/AppUpdatePanel";

export default function UpdatesPage({ animClass = "" }: { animClass?: string }) {
  const [version, setVersion] = useState("");
  useEffect(() => { void getVersion().then(setVersion).catch(() => {}); }, []);
  return (
    <div className="page-root">
      <TitleBar title="Updates" rightActions={<>
        <button aria-label="Minimize" className="icon-btn" onClick={() => void getCurrentWindow().minimize()}><Minus size={12} /></button>
        <button aria-label="Close" className="icon-btn" onClick={() => void hideMainWindow()}><X size={12} /></button>
      </>} />
      <div className={`page-content updates-content ${animClass}`}>
        <header className="workspace-heading"><div>
          <h1>Keep your flow up to date.</h1>
          <p>New releases, checked quietly in the background.</p>
        </div></header>
        <AppUpdatePanel appVersion={version} />
      </div>
    </div>
  );
}
