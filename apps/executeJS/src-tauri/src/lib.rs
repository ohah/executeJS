// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod js_executor;

use commands::*;
use tauri::menu::{MenuBuilder, SubmenuBuilder};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)]
    {
        let mut builder = tauri::Builder::default()
            .plugin(tauri_plugin_http::init())
            .plugin(tauri_plugin_opener::init())
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_clipboard_manager::init())
            .plugin(tauri_plugin_store::Builder::default().build());

        // DevTools 플러그인 추가 (개발 빌드에서만)
        #[cfg(debug_assertions)]
        {
            builder = builder.plugin(tauri_plugin_devtools::init());
        }

        builder
            .setup(|app_handle| {
                // Window Menu
                let about_menu = SubmenuBuilder::new(app_handle, "About")
                    .text("about", "About ExecuteJS")
                    .separator()
                    .text("settings", "Settings...")
                    .separator()
                    .text("quit", "Quit ExecuteJS")
                    .build()?;

                let file_menu = SubmenuBuilder::new(app_handle, "File")
                    .text("new_tab", "New Tab")
                    .separator()
                    .text("close_tab", "Close Tab")
                    .build()?;

                let edit_menu = SubmenuBuilder::new(app_handle, "Edit")
                    .undo()
                    .redo()
                    .separator()
                    .cut()
                    .copy()
                    .paste()
                    .separator()
                    .select_all()
                    .build()?;

                let view_menu = SubmenuBuilder::new(app_handle, "View")
                    .text("reload", "Reload")
                    .text("toggle_devtools", "Toggle Developer Tools")
                    .build()?;

                // Main Menu
                let menu = MenuBuilder::new(app_handle)
                    .items(&[&about_menu, &file_menu, &edit_menu, &view_menu])
                    .build()?;

                app_handle.set_menu(menu)?;

                // Menu Event
                app_handle.on_menu_event(move |app_handle, event| {
                    match event.id().0.as_str() {
                        // About Menu
                        "about" => {
                            // TODO: About ExecuteJS 다이얼로그 표시
                            eprintln!("About ExecuteJS 메뉴 클릭됨");
                        }
                        "settings" => {
                            // TODO: Settings 다이얼로그 표시
                            eprintln!("Settings 메뉴 클릭됨");
                        }
                        "quit" => {
                            app_handle.exit(0);
                        }

                        // File Menu
                        "new_tab" => {
                            // TODO: 새 탭 추가
                            eprintln!("New Tab 메뉴 클릭됨");
                        }
                        "close_tab" => {
                            // TODO: 현재 탭 닫기
                            eprintln!("Close Tab 메뉴 클릭됨");
                        }

                        // View Menu
                        "reload" => {
                            if let Some(window) = app_handle.get_webview_window("main") {
                                window.eval("window.location.reload()").unwrap();
                            }
                        }
                        "toggle_devtools" => {
                            if let Some(window) = app_handle.get_webview_window("main") {
                                window.open_devtools();
                                window.close_devtools();
                            }
                        }
                        _ => {
                            eprintln!("메뉴 이벤트: {:?}", event.id());
                        }
                    }
                });

                // JavaScript 실행기 상태 관리
                #[cfg(debug_assertions)]
                {
                    let window = app_handle.get_webview_window("main").unwrap();
                    window.open_devtools();
                    window.close_devtools();
                }

                // 앱 시작 시 초기화 작업
                tauri::async_runtime::spawn(async {
                    tracing::info!("ExecuteJS 애플리케이션이 시작되었습니다.");
                });

                Ok(())
            })
            .on_window_event(|_window, event| {
                // 앱 종료 시 정리 작업
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    tracing::info!("ExecuteJS 애플리케이션이 종료됩니다.");
                }
            })
            .invoke_handler(tauri::generate_handler![
                execute_js,
                get_app_info,
                lint_code
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
