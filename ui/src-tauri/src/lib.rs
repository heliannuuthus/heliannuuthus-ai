use std::sync::{Mutex, Once};

#[cfg(target_os = "macos")]
use tauri::Manager;

use tauri::{ActivationPolicy, AppHandle, WindowEvent};

mod config;
mod core;
mod utils;
use core::hotkey;

use config::logging;
use config::runtime::setup_runtime;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct AppManager {
    inner: Mutex<Option<AppHandle>>,
    initializer: Once,
}

impl AppManager {
    pub fn instance() -> &'static Self {
        static INSTANCE: AppManager = AppManager {
            inner: Mutex::new(None),
            initializer: Once::new(),
        };
        &INSTANCE
    }

    pub fn init(&self, app: AppHandle) {
        self.initializer.call_once(|| {
            let mut inner = self.inner.lock().unwrap();
            *inner = Some(app);
        });
    }

    pub fn get_handler(&self) -> AppHandle {
        self.inner
            .lock()
            .unwrap()
            .clone()
            .expect("AppHandle not initialized")
    }

    pub fn enable_activation_policy_regular(&self) {
        #[cfg(target_os = "macos")]
        {
            let _ = self
                .inner
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .set_activation_policy(ActivationPolicy::Regular);
        }
    }

    pub fn enable_activation_policy_accessory(&self) {
        #[cfg(target_os = "macos")]
        {
            let _ = self
                .inner
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .set_activation_policy(ActivationPolicy::Accessory);
        }
    }

    #[allow(dead_code)]
    pub fn enabled_activation_policy_prohibited(&self) {
        #[cfg(target_os = "macos")]
        {
            let _ = self
                .inner
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .set_activation_policy(ActivationPolicy::Prohibited);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logging::init().unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_deep_link::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                suspect_error!(app.deep_link().register_all());
            }

            tauri::async_runtime::block_on(async move {
                setup_runtime(app).await;
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|handler, e| match e {
            tauri::RunEvent::ExitRequested { code, api, .. } => {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
            tauri::RunEvent::Ready | tauri::RunEvent::Resumed => {
                AppManager::instance().init(handler.clone());
                #[cfg(target_os = "macos")]
                {
                    let main_window = AppManager::instance()
                        .get_handler()
                        .get_webview_window("main")
                        .unwrap();
                    let _ = main_window.set_title("Annuus Artificial Intelligence Kit");
                }
            }
            #[cfg(target_os = "macos")]
            tauri::RunEvent::Reopen {
                has_visible_windows,
                ..
            } => {
                if !has_visible_windows {
                    AppManager::instance().enable_activation_policy_regular();
                }
                AppManager::instance().init(handler.clone());
            }
            tauri::RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    match event {
                        WindowEvent::CloseRequested { api, .. } => {
                            #[cfg(target_os = "macos")]
                            AppManager::instance().enable_activation_policy_accessory();
                            api.prevent_close();
                            let window = AppManager::instance()
                                .get_handler()
                                .get_webview_window("main")
                                .unwrap();
                            window.hide().unwrap();
                        }
                        WindowEvent::Destroyed => {
                            #[cfg(target_os = "macos")]
                            {
                                suspect_error!(hotkey::Hotkey::instance().unregister("CMD+Q"));
                            }

                            #[cfg(not(target_os = "macos"))]
                            {
                                suspect_error!(hotkey::Hotkey::instance().unregister("Control+Q"));
                            };
                        }

                        WindowEvent::Focused(true) => {
                            #[cfg(target_os = "macos")]
                            {
                                suspect_error!(hotkey::Hotkey::instance().register("CMD+Q", "quit"));
                            }

                            #[cfg(not(target_os = "macos"))]
                            {
                                suspect_error!(hotkey::Hotkey::instance().register("Control+Q", "quit"));
                            };
                        }
                        WindowEvent::Focused(false) => {
                            #[cfg(target_os = "macos")]
                            {
                                suspect_error!(hotkey::Hotkey::instance().unregister("CMD+Q"));
                            }
                            #[cfg(not(target_os = "macos"))]
                            {
                                suspect_error!(hotkey::Hotkey::instance().unregister("Control+Q"));
                            };
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });
}
