import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { store } from "./sessions.svelte";

export type RunStatus =
  | "queued"
  | "running"
  | "completed"
  | "cancelled"
  | "interrupted"
  | "needsAttention";

export interface Run {
  id: string;
  projectId: string;
  fromSessionId: string;
  toSessionId: string;
  prompt: string;
  status: RunStatus;
  response: string | null;
  detail: string | null;
  createdAt: number;
  updatedAt: number;
}

export const collab = $state({ runsBySession: {} as Record<string, Run[]> });

export async function loadRuns(sessionId: string) {
  try {
    collab.runsBySession[sessionId] = await invoke<Run[]>("list_runs", { sessionId });
  } catch (e) {
    store.error = String(e);
  }
}

let listening = false;
export function initCollabListener() {
  if (listening) return;
  listening = true;
  listen<{ projectId: string }>("collab-changed", (e) => {
    for (const t of store.tabs) {
      if (t.projectId === e.payload.projectId && t.info) loadRuns(t.info.id);
    }
  });
}

export function queuedCount(sessionId: string): number {
  return (collab.runsBySession[sessionId] ?? []).filter(
    (r) => r.toSessionId === sessionId && r.status === "queued",
  ).length;
}

export function executeRun(id: string) {
  return invoke<boolean>("execute_run", { id });
}

export function cancelRun(id: string) {
  return invoke<void>("cancel_run", { id });
}

export function sendRequest(from: string, to: string, text: string) {
  return invoke<Run>("send_request", { from, to, text });
}
