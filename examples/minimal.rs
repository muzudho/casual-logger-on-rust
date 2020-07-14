//! The smallest example.

use casual_logger::{Log, LOGGER};

fn main() {
    if let Ok(logger) = LOGGER.lock() {
        logger.remove_old_logs();
    } else {
        // Do not delete old logs.
    };

    Log::infoln("Hello, world!!");

    // Wait for logging to complete or to timeout.
    Log::wait();
}
