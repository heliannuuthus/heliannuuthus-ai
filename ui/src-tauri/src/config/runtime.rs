use tauri::{App, Manager};

use crate::{
    core::{handler::Handler, hotkey},
    suspect_error, AppManager,
};

use super::window::create;

pub async fn setup_runtime(app: &mut App) {
    #[cfg(target_os = "macos")]
    {
        AppManager::instance().init(app.app_handle().clone());
        AppManager::instance().enable_activation_policy_accessory();
    }
    Handler::instance().init(app.app_handle().clone());

    suspect_error!(hotkey::Hotkey::instance().init());
    create();
}
