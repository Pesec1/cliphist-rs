use std::sync::atomic::{AtomicBool, Ordering};

static LOGGING_ENABLED: AtomicBool = AtomicBool::new(false);


pub fn enable_logging() {
    LOGGING_ENABLED.store(true, Ordering::Relaxed);
}

pub fn is_enabled() -> bool {
    return LOGGING_ENABLED.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        if $crate::logger::is_enabled() {
            println!($($arg)*);
        }
    };
}
