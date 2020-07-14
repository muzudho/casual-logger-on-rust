//! Performance check

use casual_logger::{Level, Log, LOGGER};
use std::time::Instant;
// use sys_info::mem_info;

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

    let size = 332_000;
    for _i in 0..size {
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
    println!(
        "Performance: {} records, {} ms.",
        size,
        stopwatch.elapsed().as_millis(),
        /*
        if let Ok(mem) = mem_info() {
            format!(
                "Mem=|Total {}|Avail {}|Buffers {}|Cached {}|Free {}|SwapFree {}|SwapTotal {}|",
                mem.total,
                mem.avail,
                mem.buffers,
                mem.cached,
                mem.free,
                mem.swap_free,
                mem.swap_total
            )
        } else {
            "".to_string()
        }
        */
    )
}
