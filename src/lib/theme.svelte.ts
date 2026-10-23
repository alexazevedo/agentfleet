export type ThemeSetting = "system" | "light" | "dark";
export type ResolvedTheme = "light" | "dark";

const QUERY = "(prefers-color-scheme: dark)";

export function resolveTheme(setting: ThemeSetting, prefersDark: boolean): ResolvedTheme {
  if (setting === "system") return prefersDark ? "dark" : "light";
  return setting;
}

export const theme = $state({ resolved: "dark" as ResolvedTheme });

let current: ThemeSetting = "system";

function prefersDark(): boolean {
  return window.matchMedia(QUERY).matches;
}

export function applyTheme(setting: ThemeSetting): void {
  current = setting;
  const root = document.documentElement;
  if (setting === "system") delete root.dataset.theme;
  else root.dataset.theme = setting;
  theme.resolved = resolveTheme(setting, prefersDark());
}

/** Tracks OS appearance changes while the setting is "system". Returns a disposer. */
export function initTheme(): () => void {
  const mq = window.matchMedia(QUERY);
  const onChange = () => applyTheme(current);
  mq.addEventListener("change", onChange);
  onChange();
  return () => mq.removeEventListener("change", onChange);
}
