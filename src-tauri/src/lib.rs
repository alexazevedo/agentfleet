mod agent;
mod platform;
mod project;
mod pty;
mod session;
mod state;
mod status;
mod worktree;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(session::Sessions::default())
        .manage(project::Projects::default())
        .invoke_handler(tauri::generate_handler![
            session::create_session,
            session::write_session,
            session::resize_session,
            session::close_session,
            session::list_restorable,
            session::attach_session,
            project::list_projects,
            project::add_project,
            project::remove_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running agentfleet");
}
