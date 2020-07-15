//! Performance check

use casual_logger::{Extension, Log, LOGGER};
use std::thread;
use std::time::Instant;
// use sys_info::mem_info;

fn main() {
    let stopwatch = Instant::now();
    Log::set_file_name("performance-check");
    Log::set_file_ext(Extension::Log);
    Log::set_retention_days(2);
    if let Ok(mut logger) = LOGGER.lock() {
        logger.timeout_secs = 30;
        logger.development = true;
    }
    Log::remove_old_logs();

    let mut count = 0;
    // Single thread test.
    let size = 100_000;
    for i in 0..size {
        Log::infoln(&format!("Hello, world!! {}", i));
        count += 1;
    }

    // Multi thread test.
    let size = 30_000;
    thread::spawn(move || {
        for i in 0..size {
            Log::infoln(&format!("Good morning! {}", i));
            count += 1;
        }
    });
    thread::spawn(move || {
        for i in 0..size {
            Log::infoln(&format!("Good afternoon! {}", i));
            count += 1;
        }
    });
    thread::spawn(move || {
        for i in 0..size {
            Log::infoln(&format!("Good night! {}", i));
            count += 1;
        }
    });

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
        count,
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
