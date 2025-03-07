use std::fs;
use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
struct Timer;

impl FormatTime for Timer {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = Utc::now();
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

pub fn init() -> Result<()> {
    let log_dir = "logs";
    if !Path::new(log_dir).exists() {
        fs::create_dir_all(log_dir).expect("Failed to create log directory");
    }

    let file_appender =
        RollingFileAppender::new(Rotation::DAILY, log_dir, "heliannuuthus-ai-ui.log");
    let file_layer = fmt::layer()
        .with_writer(file_appender)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_level(true)
        .with_timer(Timer);

    // 控制台日志层
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_level(true)
        .with_timer(Timer);

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(file_layer)
        .with(stdout_layer)
        .try_init()
        .expect("Failed to initialize tracing");

    Ok(())
}
