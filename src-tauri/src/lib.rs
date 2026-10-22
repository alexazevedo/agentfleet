mod agent;
mod broker;
pub mod cli;
mod collab;
mod deliver;
mod platform;
mod project;
mod pty;
mod runtime;
mod scratchpad;
mod session;
mod state;
mod status;
mod worktree;

pub fn run() {
    use tauri::Manager;
    let rt = runtime::Runtime::create().expect("failed to create agentfleet runtime dir");
    let rt_dir = rt.dir.clone();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(session::Sessions::default())
        .manage(project::Projects::default())
        .manage(collab::CollabNotify::default())
        .manage(rt)
        .setup(|app| {
            app.manage(std::sync::Mutex::new(collab::load(&collab::collab_path())));
            tauri::async_runtime::spawn(broker::serve(app.handle().clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            session::create_session,
            session::write_session,
            session::resize_session,
            session::close_session,
            session::list_restorable,
            session::attach_session,
            collab::list_runs,
            collab::send_request,
            deliver::execute_run,
            deliver::cancel_run,
            project::list_projects,
            project::add_project,
            project::remove_project,
            scratchpad::read_scratchpad,
            scratchpad::write_scratchpad
        ])
        .build(tauri::generate_context!())
        .expect("error while running agentfleet")
        .run(move |_app, event| {
            if let tauri::RunEvent::Exit = event {
                let _ = std::fs::remove_dir_all(&rt_dir);
            }
        });
}
