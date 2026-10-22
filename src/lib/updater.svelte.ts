import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

export const updater = $state({
  checking: false,
  available: null as null | { version: string; body: string },
  progress: "",
  error: "",
  lastResult: "",
});

let pending: Update | null = null;
let timeout: ReturnType<typeof setTimeout> | undefined;
let interval: ReturnType<typeof setInterval> | undefined;

export async function checkForUpdate(manual: boolean) {
  if (updater.checking) return;
  updater.checking = true;
  if (manual) {
    updater.error = "";
    updater.lastResult = "";
  }
  try {
    const u = await check();
    if (u) {
      pending = u;
      updater.available = { version: u.version, body: u.body ?? "" };
    } else if (manual) {
      updater.lastResult = "You're up to date";
    }
  } catch (e) {
    if (manual) updater.error = String(e);
    else console.warn("update check failed", e);
  } finally {
    updater.checking = false;
  }
}

export async function installUpdate() {
  if (!pending) return;
  updater.error = "";
  let total = 0;
  let done = 0;
  try {
    await pending.downloadAndInstall((ev) => {
      if (ev.event === "Started") total = ev.data.contentLength ?? 0;
      else if (ev.event === "Progress") done += ev.data.chunkLength;
      else if (ev.event === "Finished") updater.progress = "Installing…";
      if (ev.event !== "Finished") {
        updater.progress = `Downloaded ${done} / ${total || "?"} bytes`;
      }
    });
    await relaunch();
  } catch (e) {
    updater.error = String(e);
    updater.progress = "";
  }
}

export function dismissUpdate() {
  updater.available = null;
  updater.progress = "";
  pending = null;
}

export function startScheduler() {
  stopScheduler();
  timeout = setTimeout(() => {
    checkForUpdate(false);
    interval = setInterval(() => checkForUpdate(false), 24 * 60 * 60 * 1000);
  }, 10_000);
}

export function stopScheduler() {
  clearTimeout(timeout);
  clearInterval(interval);
  timeout = interval = undefined;
}
