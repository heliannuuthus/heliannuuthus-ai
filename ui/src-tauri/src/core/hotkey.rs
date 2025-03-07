use crate::config::window::quit;
use crate::{config::window, core::handler::Handler};
use anyhow::{bail, Result};
use parking_lot::Mutex;
use std::sync::{Arc, OnceLock};
use tauri::{async_runtime, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, ShortcutState};
use tracing::{error, info};
pub struct Hotkey {
    current: Arc<Mutex<Vec<String>>>, // 保存当前的热键设置
}

impl Hotkey {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<Hotkey> = OnceLock::new();
        INSTANCE.get_or_init(|| Hotkey {
            current: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn init(&self) -> Result<()> {
        info!("Initializing hotkeys, global hotkey enabled");
        Ok(())
    }

    pub fn register(&self, hotkey: &str, func: &str) -> Result<()> {
        let app_handle = Handler::instance().get_handler().unwrap();
        let shortcut = app_handle.global_shortcut();
        info!("Attempting to register hotkey: {}, func: {}", hotkey, func);
        if shortcut.is_registered(hotkey) {
            info!("Hotkey already registered: {}, start unregister", hotkey);
            shortcut.unregister(hotkey)?;
        }

        let f = match func.trim() {
            "open_or_close_dashboard" => {
                info!("Registering open or close dashboard");
                || {
                    async_runtime::spawn_blocking(|| {
                        if let Some(window) = Handler::instance()
                            .get_handler()
                            .unwrap()
                            .get_webview_window("main")
                        {
                            // 如果窗口可见，则隐藏它
                            if window.is_visible().unwrap_or(false) {
                                info!("Window is visible, hiding it");
                                let _ = window.hide();
                            } else {
                                // 如果窗口不可见，则显示它
                                info!("Window is hidden, showing it");
                                if window.is_minimized().unwrap_or(false) {
                                    let _ = window.unminimize();
                                }
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        } else {
                            // 如果窗口不存在，创建一个新窗口
                            info!("Window does not exist, creating a new one");
                            window::create();
                        }
                    });
                }
            }
            "quit" => || quit(0),
            _ => {
                error!("Invalid function: {}", func);
                bail!("Invalid function: {}", func)
            }
        };

        let quit = func.trim() == "quit";

        let _ = shortcut.on_shortcut(hotkey, move |app_handle, hotkey, event| {
            if event.state == ShortcutState::Pressed {
                if hotkey.key == Code::KeyQ && quit {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        if window.is_focused().unwrap_or(false) {
                            info!("Executing quit");
                            f();
                        }
                    }
                } else {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        if window.is_focused().unwrap_or(false)
                            && window.is_visible().unwrap_or(false)
                        {
                            f();
                        }
                    }
                }
            }
        });

        Ok(())
    }

    pub fn unregister(&self, hotkey: &str) -> Result<()> {
        let app_handle = Handler::instance().get_handler().unwrap();
        let manager = app_handle.global_shortcut();
        manager.unregister(hotkey)?;
        info!(target: "app", "unregister hotkey {hotkey}");
        Ok(())
    }

    #[allow(dead_code)]
    pub fn reset(&self) -> Result<()> {
        let handler = Handler::instance();
        let app_handle = handler.get_handler().unwrap();
        let shortcut = app_handle.global_shortcut();
        shortcut.unregister_all()?;
        Ok(())
    }
}
