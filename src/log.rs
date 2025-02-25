/// Logs a debug message when running in debug mode.
/// In release builds, this function is compiled out.
#[cfg(debug_assertions)]
pub(crate) fn log_debug(message: &str) {
    println!("[DBG] {message}");
}

/// Stub for debug logging in release builds.
#[cfg(not(debug_assertions))]
pub(crate) fn log_debug(_: &str) {}

/// Logs an info message when running in debug mode.
/// In release builds, this function is compiled out.
#[cfg(debug_assertions)]
pub(crate) fn log_info(message: &str) {
    println!("[INF] {message}");
}

/// Stub for info logging in release builds.
#[cfg(not(debug_assertions))]
pub(crate) fn log_info(_: &str) {}

/// Logs an error message when running in debug mode.
/// In release builds, this function is compiled out.
#[cfg(debug_assertions)]
pub(crate) fn log_error(message: &str) {
    println!("[ERR] {message}");
}

/// Stub for error logging in release builds.
#[cfg(not(debug_assertions))]
pub(crate) fn log_error(_: &str) {}
