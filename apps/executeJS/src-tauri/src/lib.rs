// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod js_executor;

use commands::*;
use tauri::menu::{MenuBuilder, SubmenuBuilder};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
                        // Settings 창이 이미 열려있는지 확인
                        if let Some(existing_window) = app_handle.get_webview_window("settings") {
                            existing_window.set_focus().unwrap_or_default();
                        } else {
                            // 새 Settings 창 생성
                            // 개발 모드에서는 devUrl 사용, 프로덕션에서는 App 경로 사용
                            let url = if cfg!(debug_assertions) {
                                WebviewUrl::External(
                                    "http://localhost:1420/settings"
                                        .parse()
                                        .expect("failed to parse dev settings URL"),
                                )
                            } else {
                                // TODO: 해시 라우팅을 사용하여 Settings 페이지로 이동 처리. 다른 방법으로 수정 필요
                                WebviewUrl::App("index.html#/settings".into())
                            };

                            match WebviewWindowBuilder::new(app_handle, "settings", url)
                                .title("General")
                                .inner_size(800.0, 600.0)
                                .min_inner_size(600.0, 400.0)
                                .resizable(true)
                                .build()
                            {
                                Ok(_settings_window) => {
                                    // Settings 창 생성 완료
                                    // DevTools는 플러그인을 통해 자동으로 관리됨
                                }
                                Err(e) => {
                                    eprintln!("Settings 창 생성 실패: {:?}", e);
                                }
                            }
                        }
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
                        // Tauri 2.0에서는 DevTools 플러그인을 통해 제어
                        #[cfg(debug_assertions)]
                        {
                            if app_handle.get_webview_window("main").is_some() {
                                // DevTools는 플러그인을 통해 자동으로 관리됨
                                eprintln!("DevTools 토글 (플러그인 사용)");
                            }
                        }
                    }
                    _ => {
                        eprintln!("메뉴 이벤트: {:?}", event.id());
                    }
                }
            });

            // JavaScript 실행기 상태 관리
            // DevTools는 플러그인을 통해 자동으로 관리됨

            // 앱 시작 시 초기화 작업
            tauri::async_runtime::spawn(async {
                tracing::info!("ExecuteJS 애플리케이션이 시작되었습니다.");

                // Node.js 바이너리 확인 및 다운로드
                eprintln!("[ExecuteJS] Node.js 바이너리 확인 시작...");
                match node_runtime::NodeExecutor::ensure_node_binary().await {
                    Ok(path) => {
                        eprintln!(
                            "[ExecuteJS] ✅ Node.js 바이너리 준비 완료: {}",
                            path.display()
                        );
                        tracing::info!("Node.js 바이너리 준비 완료: {}", path.display());
                    }
                    Err(e) => {
                        eprintln!("[ExecuteJS] ❌ Node.js 바이너리 초기화 실패: {}", e);
                        tracing::error!("Node.js 바이너리 초기화 실패: {}", e);
                        // 사용자에게 알림은 나중에 UI로 표시할 수 있음
                    }
                }
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
