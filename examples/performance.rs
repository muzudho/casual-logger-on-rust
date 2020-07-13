//! Performance check

use casual_logger::{Level, Log, LOGGER};
use std::time::Instant;

fn main() {
    let stopwatch = Instant::now();
    let remove_num = if let Ok(mut logger) = LOGGER.lock() {
        logger.set_file_name("performance-check", ".log", ".toml");
        logger.retention_days = 2;
        logger.level = Level::Trace;
        logger.remove_old_logs()
    } else {
        0
    };
    Log::noticeln(&format!("Remove {} files.", remove_num));

    for _i in 0..10000 {
        Log::infoln("Hello, world!!");
    }

    // Wait for logging to complete. Time out 30 seconds.
    Log::wait_for_logging_to_complete(30, |secs, message| {
        println!("{} sec(s). {}", secs, message);
    });
    println!("Performance: {} ms.", stopwatch.elapsed().as_millis())
}
