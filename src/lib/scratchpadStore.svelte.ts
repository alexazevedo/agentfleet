import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface Scratchpad {
  revision: number;
  text: string;
}

type Listener = (projectId: string, revision: number) => void;
const listeners = new Set<Listener>();

export function loadScratchpad(projectId: string) {
  return invoke<Scratchpad>("read_scratchpad", { projectId });
}

export function saveScratchpad(projectId: string, text: string, ifRevision: number | null) {
  return invoke<number>("write_scratchpad", { projectId, text, ifRevision });
}

/** Subscribe to scratchpad-changed; returns an unsubscribe function. */
export function onScratchpadChanged(fn: Listener) {
  listeners.add(fn);
  return () => listeners.delete(fn);
}

let listening = false;
export function initScratchpadListener() {
  if (listening) return;
  listening = true;
  listen<{ projectId: string; revision: number }>("scratchpad-changed", (e) => {
    for (const l of listeners) l(e.payload.projectId, e.payload.revision);
  });
}
