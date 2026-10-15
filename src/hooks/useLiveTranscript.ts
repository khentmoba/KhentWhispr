import { useSyncExternalStore } from "react";
import { readLocalStorage, writeLocalStorage } from "@/lib/storage";

export const LIVE_TRANSCRIPT_KEY = "show-live-transcript";
const CHANGE_EVENT = "live-transcript-changed";

function subscribe(onChange: () => void) {
  const onStorage = (event: StorageEvent) => {
    if (event.key === LIVE_TRANSCRIPT_KEY || event.key === null) onChange();
  };
  window.addEventListener("storage", onStorage);
  window.addEventListener(CHANGE_EVENT, onChange);
  return () => {
    window.removeEventListener("storage", onStorage);
    window.removeEventListener(CHANGE_EVENT, onChange);
  };
}

export function useLiveTranscript() {
  const enabled = useSyncExternalStore(subscribe, () => readLocalStorage(LIVE_TRANSCRIPT_KEY) !== "false");
  const setEnabled = (value: boolean) => {
    writeLocalStorage(LIVE_TRANSCRIPT_KEY, String(value));
    window.dispatchEvent(new Event(CHANGE_EVENT));
  };
  return [enabled, setEnabled] as const;
}
