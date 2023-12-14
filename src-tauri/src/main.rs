#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod menu;
use crate::menu::AddDefaultSubmenus;
use cocoa::{
    appkit::{NSWindow, NSWindowTitleVisibility},
    base::{id, YES},
};
use std::env;
use tauri::{Manager, Menu};
use tauri_plugin_fs_watch::Watcher;

#[tauri::command]
fn get_current_dir() -> String {
    format!("{}", env::current_dir().unwrap().display())
}

fn main() {
    let ctx = tauri::generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            handle.windows().iter().for_each(|window| {
                let ns_window: id = window.1.ns_window().unwrap() as id;
                unsafe {
                    ns_window.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);
                    ns_window.setTitlebarAppearsTransparent_(YES);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_current_dir])
        .plugin(Watcher::default())
        .menu(
            Menu::new()
                .add_default_app_submenu_if_macos(&ctx.package_info().name)
                .add_default_file_submenu()
                .add_default_edit_submenu()
                .add_default_view_submenu()
                .add_default_window_submenu(),
        )
        .on_menu_event(menu::handle_menu_event)
        .run(ctx)
        .expect("error while running tauri application");
}
