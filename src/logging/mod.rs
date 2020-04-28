use log::{Record, Level, Metadata, SetLoggerError};
use yew::services::console::ConsoleService;

pub struct ConsoleLogger{
    pub should_log: bool
}

pub static mut LOGGER: ConsoleLogger = ConsoleLogger{ should_log: true };

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {

            let target = if record.target().len() > 0 {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };

            if !target.contains("yew") {
                if self.should_log {
                    ConsoleService::new().log(&format!("[{}][{}] {}", target, record.level(), record.args()));
                }
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() -> Result<(), SetLoggerError> {
    unsafe {
        log::set_logger(&LOGGER)?;
    }
    log::set_max_level(Level::Trace.to_level_filter());
    Ok(())
}
