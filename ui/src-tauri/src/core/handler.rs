use parking_lot::RwLock;
use std::sync::{Arc, OnceLock};
use tauri::AppHandle;

#[derive(Debug, Default, Clone)]
pub struct Handler {
    pub inner: Arc<RwLock<Option<AppHandle>>>,
}

impl Handler {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<Handler> = OnceLock::new();
        INSTANCE.get_or_init(|| Handler {
            inner: Arc::new(RwLock::new(None)),
        })
    }

    pub fn init(&self, app_handle: AppHandle) {
        let mut handler = self.inner.write();
        *handler = Some(app_handle);
    }

    pub fn get_handler(&self) -> Option<AppHandle> {
        let handler = self.inner.read();
        handler.clone()
    }
}
