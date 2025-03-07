use crate::AppManager;
use tauri::{AppHandle, Manager};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use tracing::{error, info};

pub fn create() {
    info!("Starting to create window");
    let app_handle = AppManager::instance().get_handler();

    AppManager::instance().enable_activation_policy_regular();

    if let Some(window) = app_handle.get_webview_window("main") {
        {
            info!("Found existing window, trying to show it");

            if window.is_minimized().unwrap_or(false) {
                info!("Window is minimized, unminimizing");
                let _ = window.unminimize();
            }
            let _ = window.show();
            let _ = window.set_focus();
            return;
        }
    }

    info!("Creating new window");

    #[cfg(target_os = "windows")]
    let window = tauri::WebviewWindowBuilder::new(
              &app_handle,
              "main".to_string(),
              tauri::WebviewUrl::App("index.html".into()),
          )
          .title("Clash Verge")
          .inner_size(890.0, 700.0)
          .min_inner_size(620.0, 550.0)
          .decorations(false)
          .maximizable(true)
          .additional_browser_args("--enable-features=msWebView2EnableDraggableRegions --disable-features=OverscrollHistoryNavigation,msExperimentalScrolling")
          .transparent(true)
          .shadow(true)
          .build();

    #[cfg(target_os = "macos")]
    let window = tauri::WebviewWindowBuilder::new(
        &app_handle,
        "main".to_string(),
        tauri::WebviewUrl::App("index.html".into()),
    )
    .decorations(true)
    .hidden_title(true)
    .title_bar_style(tauri::TitleBarStyle::Overlay)
    .inner_size(890.0, 700.0)
    .min_inner_size(620.0, 550.0)
    .shadow(true)
    .build();

    #[cfg(target_os = "linux")]
    let window = tauri::WebviewWindowBuilder::new(
        &app_handle,
        "main".to_string(),
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("Annuus Artificial Intelligence Kit")
    .decorations(false)
    .inner_size(890.0, 700.0)
    .min_inner_size(620.0, 550.0)
    .transparent(true)
    .build();

    match window {
        Ok(window) => {
            info!("Window created successfully, attempting to show");
            let _ = window.show();
            let _ = window.set_focus();

            // 设置窗口状态监控，实时保存窗口位置和大小
            add_window_state_listener(&app_handle);
        }
        Err(e) => {
            error!(target: "app", "Failed to create window: {:?}", e);
        }
    }
}

pub fn quit(code: i32) {
    let app_handle = AppManager::instance().get_handler();
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.close();
    }
    app_handle.exit(code);
}

pub fn add_window_state_listener(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    let app_handle_clone = app.clone();

    // 监听窗口移动事件
    let app_handle_move = app_handle_clone.clone();
    window.on_window_event(move |event| {
        match event {
            // 窗口移动时保存状态
            tauri::WindowEvent::Moved(_) => {
                let _ = app_handle_move.save_window_state(StateFlags::all());
            }
            // 窗口调整大小时保存状态
            tauri::WindowEvent::Resized(_) => {
                let _ = app_handle_move.save_window_state(StateFlags::all());
            }
            // 其他可能改变窗口状态的事件
            tauri::WindowEvent::ScaleFactorChanged { .. } => {
                let _ = app_handle_move.save_window_state(StateFlags::all());
            }
            // 窗口关闭时保存
            tauri::WindowEvent::CloseRequested { .. } => {
                let _ = app_handle_move.save_window_state(StateFlags::all());
            }
            _ => {}
        }
    });
}
