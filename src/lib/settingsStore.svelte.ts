import { invoke } from "@tauri-apps/api/core";
import type { AgentKind } from "./sessions.svelte";
import type { ThemeSetting } from "./theme.svelte";

export interface TerminalSettings {
  fontFamily: string;
  fontSize: number;
  lineHeight: number;
  cursorBlink: boolean;
}

export interface Settings {
  worktreesRoot: string;
  defaultAgent: AgentKind;
  autoPermission: boolean;
  bypassPermissions: boolean;
  autoDelivery: boolean;
  confirmQuit: boolean;
  updateChecks: boolean;
  restoreSessions: "ask" | "always" | "never";
  theme: ThemeSetting;
  terminal: TerminalSettings;
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
    bypassPermissions: false,
    autoDelivery: true,
    confirmQuit: true,
    updateChecks: true,
    restoreSessions: "ask",
    theme: "system",
    terminal: { fontFamily: "", fontSize: 13, lineHeight: 1.0, cursorBlink: true },
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

type DeepPartial<T> = { [K in keyof T]?: T[K] extends object ? Partial<T[K]> : T[K] };

let chain: Promise<unknown> = Promise.resolve();

/** Deep-merge `patch` into a snapshot of the current settings and save. Resolves "" on success, the backend message on failure; settings.value is untouched on failure. Never throws. */
export function updateSettings(patch: DeepPartial<Settings>): Promise<string> {
  const run = async (): Promise<string> => {
    const current = $state.snapshot(settings.value) as Settings;
    const { terminal, ...rest } = patch;
    const next: Settings = {
      ...current,
      ...(rest as Partial<Settings>),
      terminal: { ...current.terminal, ...terminal },
    };
    try {
      await saveSettings(next);
      return "";
    } catch (e) {
      return String(e);
    }
  };
  const result = chain.then(run, run);
  chain = result;
  return result;
}

export function appInfo() {
  return invoke<AppInfo>("app_info");
}
