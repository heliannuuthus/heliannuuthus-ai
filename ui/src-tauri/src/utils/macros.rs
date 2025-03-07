#[macro_export]
macro_rules! suspect_error {
    ($result: expr) => {
        if let Err(e) = $result {
            tracing::error!("{e}");
        }
    };
}
