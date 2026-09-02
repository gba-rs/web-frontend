use log::LevelFilter;

pub fn init_logger() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("failed to initialize logger");
    log::set_max_level(LevelFilter::Off);
}

pub fn set_logging_enabled(enabled: bool) {
    log::set_max_level(if enabled { LevelFilter::Trace } else { LevelFilter::Off });
}
