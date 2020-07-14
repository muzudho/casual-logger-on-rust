//! Performance check

use casual_logger::{Level, Log, LOGGER};
use std::time::Instant;

fn main() {
    let stopwatch = Instant::now();
    if let Ok(mut logger) = LOGGER.lock() {
        logger.set_file_name("performance-check", ".log", ".toml");
        logger.level = Level::Trace;
        logger.timeout_secs = 30;
        logger.development = true;
        logger.retention_days = 2;
    }
    Log::remove_old_logs();

    for _i in 0..100_000 {
        Log::infoln("Hello, world!!");
    }

    /*
    // Fatal is Panic! Can be used as the first argument of.
    panic!(Log::fatal(&format!(
        "Panic successful. Performance: {} ms.",
        stopwatch.elapsed().as_millis()
    )));
    */

    // Wait for logging to complete or to timeout.
    Log::wait();
    println!("Performance: {} ms.", stopwatch.elapsed().as_millis())
}
