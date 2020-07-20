//! Performance check

use casual_logger::{Extension, Log, Opt};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
// use sys_info::mem_info;

fn main() {
    let stopwatch = Instant::now();
    Log::set_file_name("performance-check");
    Log::set_file_ext(Extension::Log);
    Log::set_retention_days(2);
    // Log::set_opt(Opt::Development); // 12477 ms.
    // Log::set_opt(Opt::BeginnersSupport); // 7657 ms. --> 9589 ms.
    Log::set_opt(Opt::Release); // 10998 ms. --> 10409 ms. --> 14035 ms. --> 13975 ms.
    Log::remove_old_logs();
    println!("Notice  | Start!");

    // Multi thread test.
    let size = 100_000;
    let (sender1, receiver1) = mpsc::channel();
    thread::spawn(move || {
        let mut count_1 = 0;
        for i in 0..size {
            Log::infoln(&format!("Good morning! {}", i + 1));
            count_1 += 1;
        }
        if let Err(msg) = sender1.send(count_1) {
            panic!(msg);
        }
    });

    let (sender2, receiver2) = mpsc::channel();
    thread::spawn(move || {
        let mut count_2 = 0;
        for i in 0..size {
            Log::infoln(&format!("Good afternoon! {}", i + 1));
            count_2 += 1;
        }
        if let Err(msg) = sender2.send(count_2) {
            panic!(msg);
        }
    });

    let (sender3, receiver3) = mpsc::channel();
    thread::spawn(move || {
        let mut count_3 = 0;
        for i in 0..size {
            Log::infoln(&format!("Good night! {}", i + 1));
            count_3 += 1;
        }
        if let Err(msg) = sender3.send(count_3) {
            panic!(msg);
        }
    });

    let mut count_0 = 0;
    // Single thread test.
    let size = 300_000;
    for i in 0..size {
        Log::infoln(&format!("Hello, world!! {}", i + 1));
        count_0 += 1;
    }

    // Wait for logging to complete or to timeout.
    Log::flush();

    // Block.
    let count_1 = if let Ok(count_1) = receiver1.recv() {
        count_1
    } else {
        0
    };
    let count_2 = if let Ok(count_2) = receiver2.recv() {
        count_2
    } else {
        0
    };
    let count_3 = if let Ok(count_3) = receiver3.recv() {
        count_3
    } else {
        0
    };

    /*
    // Fatal is Panic! Can be used as the first argument of.
    panic!(Log::fatal(&format!(
        "Panic successful. Performance: {} ms.",
        stopwatch.elapsed().as_millis()
    )));
    */

    println!(
        "Performance: |{}|{}|{}|{}| records, {} ms.",
        count_0,
        count_1,
        count_2,
        count_3,
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
