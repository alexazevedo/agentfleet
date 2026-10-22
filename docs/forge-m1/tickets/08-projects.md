---
id: 08
status: done
model: sonnet
depends_on: [07]
max_attempts: 3
files:
  - src-tauri/src/state.rs
  - src-tauri/src/session.rs
  - src-tauri/src/project.rs
  - src-tauri/src/lib.rs
  - src/lib/sessions.svelte.ts
  - src/lib/NewTabDialog.svelte
  - src/lib/TabBar.svelte
  - src/lib/Terminal.svelte
---
## Goal
Introduce the Project entity (a git repo folder): sessions belong to a project, state.json persists projects + sessions, and the new-session dialog picks an existing project or adds one.

## Context
State is unshipped, so keep `version: 1` and change the schema in place: `PersistedState { version, projects: Vec<Project>, sessions }`, `PersistedSession` gains `project_id: Uuid`. `Project { id: Uuid, name: String, path: PathBuf }` (`Serialize`/`Deserialize`, camelCase over IPC). A state file without `projects` fails to parse → logged, treated as empty (documented shortcut).

`project.rs`: `pub type Projects = Mutex<Vec<Project>>`; `pub fn add(projects, path) -> Result<Project, String>`: canonicalize, refuse if `git rev-parse --show-toplevel` fails ("not a git repository"), return the existing project if the path is already listed, else push `{ name = folder basename }`. `pub fn remove(projects, id, has_sessions: bool) -> Result<(), String>` refuses when sessions exist. Commands: `list_projects`, `add_project(path)`, `remove_project(id)`. Tests: add on a tempdir git repo returns basename, second add returns same id, non-repo errors, remove-with-sessions errors.

`session.rs`: `Session` gets `project_id`; `create_session(project_id: String, agent, cols, rows, on_data)` looks up the project path and uses it as `repo` (unknown id → Err). `SessionInfo` gains `projectId`. `persist` writes both lists; `list_restorable` loads projects into `Projects` and drops sessions whose project is missing. Register `.manage(Projects::default())`.

Frontend: store gains `projects: Project[]`, `loadProjects()`, `addProject(path)`; `Tab.projectId` replaces `Tab.repo`. `NewTabDialog`: `<select>` of projects + "Add project…" button (folder picker → `add_project` → select it) + agent radio. `Terminal.svelte` passes `projectId` to `create_session`. `TabBar` label uses project name. No layout change yet (ticket 09).

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `npm run build`

## Done when
- New session dialog lists projects; "Add project…" on a repo folder adds it once (second add is a no-op) and selects it; a session created under it appears with the project name; `state.json` has `projects` and each session has `projectId`.
- Relaunch restores projects and sessions.

## Not included
- Sidebar, status bar, project rename/emoji (ticket 09 and later).
