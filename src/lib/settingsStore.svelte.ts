import { invoke } from "@tauri-apps/api/core";
import type { AgentKind } from "./sessions.svelte";

export interface Settings {
  worktreesRoot: string;
  defaultAgent: AgentKind;
  autoPermission: boolean;
  autoDelivery: boolean;
  confirmQuit: boolean;
  updateChecks: boolean;
}

export interface AppInfo {
  version: string;
  commit: string;
}

export const settings = $state({
  loaded: false,
  value: {
    worktreesRoot: "",
    defaultAgent: "claude",
    autoPermission: true,
    autoDelivery: true,
    confirmQuit: true,
    updateChecks: true,
  } as Settings,
});

export async function loadSettings() {
  Object.assign(settings.value, await invoke<Settings>("get_settings"));
  settings.loaded = true;
}

/** Throws the backend validation message on failure. */
export async function saveSettings(next: Settings) {
  Object.assign(settings.value, await invoke<Settings>("set_settings", { settings: next }));
  settings.loaded = true;
}

export function appInfo() {
  return invoke<AppInfo>("app_info");
}
