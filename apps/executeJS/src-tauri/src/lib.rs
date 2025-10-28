// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod deno_runtime;
mod js_executor;

use commands::*;
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
            .invoke_handler(tauri::generate_handler![execute_js, get_app_info])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
