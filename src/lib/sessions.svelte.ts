import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type AgentKind = "claude" | "shell";

export type Status = "unknown" | "working" | "waiting" | "ended";

export interface Project {
  id: string;
  name: string;
  path: string;
}

export interface SessionInfo {
  id: string;
  id8: string;
  projectId: string;
  agent: AgentKind;
  branch: string;
  worktreePath: string;
  repoName: string;
}

// A tab: pending until Terminal.svelte's create_session call returns `info`.
export interface Tab {
  key: string;
  projectId: string;
  agent: AgentKind;
  info?: SessionInfo;
  status?: Status;
  closing?: "kept" | null;
  keptPath?: string;
  restored?: boolean;
}

export const store = $state({
  projects: [] as Project[],
  tabs: [] as Tab[],
  activeKey: null as string | null,
  view: "terminal" as "terminal" | "messages" | "scratchpad" | "settings",
  dialogOpen: false,
  preselectedProjectId: null as string | null,
  removeProjectId: null as string | null,
  error: "",
  pendingRestore: [] as SessionInfo[],
  restoreDialogOpen: false,
  // True once the launch restore decision is resolved (gates the update scheduler).
  restoreDecided: false,
});

// Events that arrive before create_session resolves (tab has no info.id yet).
const early = new Map<string, Status>();

export async function loadProjects() {
  try {
    store.projects = await invoke<Project[]>("list_projects");
  } catch (e) {
    store.error = String(e);
  }
}

export async function addProject(path: string): Promise<Project> {
  const p = await invoke<Project>("add_project", { path });
  if (!store.projects.some((x) => x.id === p.id)) store.projects.push(p);
  return p;
}

export function addPending(projectId: string, agent: AgentKind): string {
  const key = crypto.randomUUID();
  store.tabs.push({ key, projectId, agent });
  store.activeKey = key;
  return key;
}

export function resolve(key: string, info: SessionInfo) {
  const t = store.tabs.find((t) => t.key === key);
  if (t) {
    t.info = info;
    t.status = early.get(info.id) ?? "unknown";
    early.delete(info.id);
  }
}

// Create failed: drop the pending tab and surface the error in the dialog.
export function fail(key: string, message: string) {
  const i = store.tabs.findIndex((t) => t.key === key);
  if (i >= 0) store.tabs.splice(i, 1);
  if (store.activeKey === key) store.activeKey = store.tabs.at(-1)?.key ?? null;
  store.error = message;
  store.dialogOpen = true;
}

export function setActive(key: string) {
  store.activeKey = key;
  store.view = "terminal";
}

let listening = false;
export function initStatusListener() {
  if (listening) return;
  listening = true;
  listen<{ id: string; status: Status }>("session-status", (e) => {
    const t = store.tabs.find((t) => t.info?.id === e.payload.id);
    if (t) t.status = e.payload.status;
    else early.set(e.payload.id, e.payload.status);
  });
}

export function remove(key: string) {
  const i = store.tabs.findIndex((t) => t.key === key);
  if (i >= 0) store.tabs.splice(i, 1);
  if (store.activeKey === key) store.activeKey = store.tabs.at(-1)?.key ?? null;
}

type CloseOutcome = { kind: "removed" } | { kind: "kept"; path: string };

export async function closeTab(key: string) {
  const t = store.tabs.find((t) => t.key === key);
  if (!t) return;
  if (!t.info) return remove(key); // pending: nothing server-side yet
  try {
    const out = await invoke<CloseOutcome>("close_session", { id: t.info.id });
    if (out.kind === "removed") return remove(key);
    t.closing = "kept";
    t.keptPath = out.path;
  } catch (e) {
    store.error = String(e);
    return remove(key);
  }
  setTimeout(() => remove(key), 3000);
}

type ClosedSession = { id: string; outcome: CloseOutcome };

// Backend closes the sessions then removes the project; on rejection the store is untouched.
export async function removeProject(id: string): Promise<void> {
  await invoke<ClosedSession[]>("remove_project_cascade", { id });
  const keys = new Set(store.tabs.filter((t) => t.projectId === id).map((t) => t.key));
  const activeGone =
    store.activeKey !== null && (keys.has(store.activeKey) || store.activeKey === "scratchpad:" + id);
  for (let i = store.tabs.length - 1; i >= 0; i--) {
    if (keys.has(store.tabs[i].key)) store.tabs.splice(i, 1);
  }
  const pi = store.projects.findIndex((p) => p.id === id);
  if (pi >= 0) store.projects.splice(pi, 1);
  if (activeGone) {
    store.activeKey = store.tabs.at(-1)?.key ?? null;
    store.view = "terminal";
  }
}

// Called once on app start: load saved sessions into store.pendingRestore.
let listedOnce = false;
export async function listRestorable(): Promise<SessionInfo[]> {
  if (listedOnce) return store.pendingRestore;
  listedOnce = true;
  try {
    const infos = await invoke<SessionInfo[]>("list_restorable");
    await loadProjects();
    store.pendingRestore = infos;
  } catch (e) {
    store.error = String(e);
  }
  return store.pendingRestore;
}

// One tab per pending session (PTY attached by Terminal).
export function attachRestored() {
  for (const info of store.pendingRestore) {
    const key = crypto.randomUUID();
    store.tabs.push({
      key,
      projectId: info.projectId,
      agent: info.agent,
      info,
      status: early.get(info.id) ?? "unknown",
      restored: true,
    });
    early.delete(info.id);
    store.activeKey ??= key;
  }
  store.pendingRestore = [];
}

export async function forgetRestorable() {
  try {
    await invoke<number>("forget_restorable");
  } catch (e) {
    store.error = String(e);
  }
  store.pendingRestore = [];
}

export function restoreTabs() {
  return listRestorable().then(attachRestored);
}
