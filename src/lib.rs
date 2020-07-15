//! This logger with **few settings** to repeat practice of many programming tutorials.  
//! Not for product use.  

// Publish:
//
// (1) `cargo test`
// (2) `cargo run --example example`
// (3) Open auto-generated log file. I check it.
// (4) Remove the log file.
// (5) Version up on Cargo.toml.
// (6) `cargo doc --open`
// (7) Comit to Git-hub.
// (8) `cargo publish --dry-run`
// (9) `cargo publish`

#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;
// extern crate sys_info;

use chrono::{Date, Duration, Local, TimeZone};
use regex::Regex;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::ops::Add;
use std::path::Path;
use std::process;
use std::sync::Mutex;
use std::thread;
// use sys_info::mem_info;

// For multi-platform. Windows, or not.
#[cfg(windows)]
const NEW_LINE: &'static str = "\r\n";
#[cfg(windows)]
const NEW_LINE_SEQUENCE: &'static str = "\\r\\n";
#[cfg(not(windows))]
const NEW_LINE: &'static str = "\n";
#[cfg(not(windows))]
const NEW_LINE_SEQUENCE: &'static str = "\\n";

/// The higher this level, the more will be omitted.  
///
/// |<-- Low Level ------------------------- High level -->|  
/// |<-- High priority ------------------- Low priority -->|  
/// | Fatal < Error < Warn < Notice < Info < Debug < Trace |  
#[derive(Clone)]
pub enum Level {
    /// If the program cannot continue.
    #[allow(dead_code)]
    Fatal,
    /// I didn't get the expected result, so I'll continue with the other method.
    Error,
    /// It will be abnormal soon, but there is no problem and you can ignore it.
    /// For example:
    ///     * He reported that it took longer to access than expected.
    ///     * Report that capacity is approaching the limit.
    Warn,
    /// It must be enabled in the server production environment.
    /// Record of passing important points correctly.
    /// We are monitoring that it is working properly.
    Notice,
    /// Report highlights.
    /// Everything that needs to be reported regularly in the production environment.
    Info,
    /// It should be in a place with many accidents.
    /// This level is disabled in production environments.
    /// Leave it in the source and enable it for troubleshooting.
    /// Often, this is the production level of a desktop operating environment.
    Debug,
    /// Not included in the distribution.
    /// Remove this level from the source after using it for debugging.
    /// If you want to find a bug in the program, write a lot.
    Trace,
}
impl Level {
    pub fn number(&self) -> usize {
        match self {
            Level::Fatal => 1,
            Level::Error => 2,
            Level::Warn => 3,
            Level::Notice => 4,
            Level::Info => 5,
            Level::Debug => 6,
            Level::Trace => 7,
        }
    }
}
impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::Fatal => write!(f, "Fatal"),
            Level::Error => write!(f, "Error"),
            Level::Warn => write!(f, "Warn"),
            Level::Notice => write!(f, "Notice"),
            Level::Info => write!(f, "Info"),
            Level::Debug => write!(f, "Debug"),
            Level::Trace => write!(f, "Trace"),
        }
    }
}

// ```
// # Add one line to Cargo.toml
// [dependencies]
// lazy_static = "1.0.0"
//
// // Write the following two lines near the beginning of main.rs
// #[macro_use]
// extern crate lazy_static;
// ```
//
// * References
//      * [How can I use mutable lazy_static?](https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/3)
lazy_static! {
    #[deprecated(
        since = "0.3.10",
        note = "Please use the casual_logger::Log::xxxx() methods instead"
    )]
    /// Logger grobal variable.
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::default());
    /// Wait for logging to complete.
    static ref POOL: Mutex<Pool> = Mutex::new(Pool::default());
    /// Table buffer.
    static ref QUEUE_T: Mutex<VecDeque<(TableHeader,Table)>> = Mutex::new(VecDeque::<(TableHeader,Table)>::new());
    static ref QUEUE_F: Mutex<VecDeque<(TableHeader,Table)>> = Mutex::new(VecDeque::<(TableHeader,Table)>::new());
    static ref RESERVE_TARGET: Mutex<ReserveTarget> = Mutex::new(ReserveTarget::default());
    static ref SIGNAL_CAN_FLUSH: Mutex<SignalCanFlush> = Mutex::new(SignalCanFlush::default());
}
// Use the line number in the log.
//
// Thread local scope variable.
//
// * References
//      * [ミュータブルなスレッドローカルデータを thread_local!() マクロと RefCell で実現する](https://qiita.com/tatsuya6502/items/bed3702517b36afbdbca)
thread_local!(static SEQ: RefCell<u128> = {
    RefCell::new(1)
});

pub struct TableHeader {
    /// Thread ID. However, Note that you are not limited to numbers.
    thread_id: String,
    seq: u128,
}
impl TableHeader {
    pub fn new(thread_id: &str, seq: u128) -> Self {
        TableHeader {
            thread_id: thread_id.to_string(),
            seq: seq,
        }
    }
}
/// TOML table included in the log file. Do not validate.
#[derive(Clone)]
pub struct Table {
    level: Level,
    message: String,
    message_trailing_newline: bool,
    sorted_map: BTreeMap<String, String>,
}
impl Default for Table {
    fn default() -> Self {
        Table {
            sorted_map: BTreeMap::new(),
            level: Level::Trace,
            message: "".to_string(),
            message_trailing_newline: false,
        }
    }
}
impl Table {
    fn new(level: Level, message: &str, trailing_newline: bool) -> Self {
        Table {
            sorted_map: BTreeMap::new(),
            level: level,
            message: message.to_string(),
            message_trailing_newline: trailing_newline,
        }
    }
    pub fn format_str_value(value: &str) -> String {
        // Escape the trailing newline at last.
        let mut body = if value[value.len() - NEW_LINE.len()..] == *NEW_LINE {
            // Do.
            format!("{}{}", value.trim_end(), NEW_LINE_SEQUENCE)
        } else {
            // Don't.
            value.to_string()
        };
        // Escape the double quotation.
        body = body.replace("\"", "\\\"");
        if 1 < value.lines().count() {
            // Multi-line string.
            format!(
                "\"\"\"
{}
\"\"\"",
                body
            )
        } else {
            // One liner.
            format!("\"{}\"", body)
        }
    }
    /// Insert string value.
    pub fn str<'a>(&'a mut self, key: &'a str, value: &'a str) -> &'a mut Self {
        self.sorted_map.insert(
            // Log detail level.
            key.to_string(),
            // Message.
            Table::format_str_value(value).to_string(),
        );

        self
    }
    /// Insert literal string value.
    pub fn literal<'a>(&'a mut self, key: &'a str, value: &'a str) -> &'a mut Self {
        self.sorted_map.insert(
            // Log detail level.
            key.to_string(),
            // Message.
            value.to_string(),
        );

        self
    }
}

/// Easy to use logging.
pub struct Log {}
impl Log {
    /// Log file name prefix.  
    ///
    /// Example: 'tic-tac-toe-2020-07-11.log.toml'  
    /// - Prefix: 'tic-tac-toe'  
    /// - StartDate: '-2020-07-11' automatically.  
    /// - Suffix: '.log' - To be safe, include a word that  
    ///         clearly states that you can delete the file.  
    /// - Extention: '.toml'  
    pub fn set_file_name(prefix: &str) {
        if let Ok(mut logger) = LOGGER.lock() {
            logger.file_prefix = prefix.to_string();
        }
    }

    /// Log file extension.  
    /// '.log.toml' or '.log'.  
    /// If you don't like the .toml extension, change.  
    pub fn set_file_ext(ext: Extension) {
        if let Ok(mut logger) = LOGGER.lock() {
            match ext {
                Extension::LogToml => {
                    logger.file_suffix = ".log".to_string();
                    logger.file_extention = ".toml".to_string();
                }
                Extension::Log => {
                    logger.file_suffix = "".to_string();
                    logger.file_extention = ".log".to_string();
                }
            }
        }
    }

    /// Logs with lower priority than this level will not  
    /// be written.  
    ///
    /// |<-- Low Level --------------------- High level -->|  
    /// |<-- High priority --------------- Low priority -->|  
    /// |Fatal< Error < Warn < Notice < Info < Debug <Trace|  
    pub fn set_level(level: Level) {
        if let Ok(mut logger) = LOGGER.lock() {
            logger.level = level;
        }
    }

    /// You probably don't need to set this. Default: 7.  
    /// Check the StartDate in the file name and delete it if it is old.  
    pub fn set_retention_days(days: u32) {
        if let Ok(mut logger) = LOGGER.lock() {
            logger.retention_days = days as i64;
        }
    }

    /// You probably don't need to set this. Default: 30.  
    /// Wait for seconds logging to complete.  
    pub fn set_timeout_secs(secs: u64) {
        if let Ok(mut logger) = LOGGER.lock() {
            logger.timeout_secs = secs;
        }
    }

    /// You probably don't need to set this. Default: false.  
    /// Set to true to allow Casual_logger to  
    /// output information to stdout and stderr.  
    pub fn set_development(during_development: bool) {
        if let Ok(mut logger) = LOGGER.lock() {
            logger.development = during_development;
        }
    }

    /// # Returns
    ///
    /// Number of deleted log files.
    pub fn remove_old_logs() -> usize {
        let remove_num = if let Ok(logger) = LOGGER.lock() {
            // Do not call 'Log::xxxxx()' in this code block.
            let remove_num = logger.remove_old_logs();
            if logger.development && 0 < remove_num {
                println!("casual_logger: Remove {} log file(s).", remove_num);
            }
            remove_num
        } else {
            // Setup failed. Continue with the default settings.
            0
        };
        remove_num
    }
    /// Wait for logging to complete.
    ///
    /// See also: Log::set_timeout_secs(), Log::set_development().
    pub fn wait() {
        let (timeout_secs, development) = if let Ok(logger) = LOGGER.lock() {
            (Logger::get_timeout_sec(&logger), logger.development)
        } else {
            (0, false)
        };

        Log::wait_for_logging_to_complete(timeout_secs, |secs, message| {
            // Do not call 'Log::xxxxx()' in this code block.
            if development {
                eprintln!("casual_logger: {} sec(s). {}", secs, message,);
            }
        });
    }
    /// Wait for logging to complete.
    #[deprecated(
        since = "0.3.2",
        note = "Please use the casual_logger::Log::wait() method instead"
    )]
    pub fn wait_for_logging_to_complete<F>(timeout_secs: u64, count_down: F)
    where
        F: Fn(u64, String),
    {
        let mut elapsed_milli_secs = 0;
        let mut empty_que_count = 0;
        while empty_que_count < 2 && elapsed_milli_secs < timeout_secs * 1000 {
            let mut thr_num = None;
            let mut queue_len = None;
            if let Ok(pool) = POOL.lock() {
                let thr_num_val = pool.get_thread_count();
                thr_num = Some(thr_num_val);
                if thr_num_val < 1 {
                    if let Ok(reserve_target) = RESERVE_TARGET.lock() {
                        if reserve_target.is_t() {
                            if let Ok(queue) = QUEUE_T.lock() {
                                if queue.is_empty() {
                                    // count_down(elapsed_secs, "Completed.".to_string());
                                    break;
                                }
                                queue_len = Some(queue.len());
                            }
                        } else {
                            if let Ok(queue) = QUEUE_F.lock() {
                                if queue.is_empty() {
                                    // count_down(elapsed_secs, "Completed.".to_string());
                                    break;
                                }
                                queue_len = Some(queue.len());
                            }
                        }
                    }

                    // Out of QUEUE.lock().
                    if let Some(completed) = Log::flush() {
                        if completed {
                            empty_que_count = 0;
                        } else {
                            empty_que_count += 1;
                        }
                    } else {
                        // TODO Error.
                    }
                }
            }
            if elapsed_milli_secs % 1000 == 0 {
                count_down(
                    elapsed_milli_secs / 1000,
                    format!(
                        "{}{}",
                        if let Some(thr_num_val) = thr_num {
                            if 0 < thr_num_val {
                                format!("Wait for {} thread(s). ", thr_num_val)
                            } else {
                                "".to_string()
                            }
                        } else {
                            "".to_string()
                        },
                        if let Some(queue_len_val) = queue_len {
                            if 0 < queue_len_val {
                                format!("{} table(s) left. ", queue_len_val)
                            } else {
                                "".to_string()
                            }
                        } else {
                            "".to_string()
                        },
                        /*
                        if let Ok(mem) = mem_info() {
                            format!(
                                "Mem=|Total {}|Avail {}|Buffers {}|Cached {}|Free {}|SwapFree {}|SwapTotal {}| ",
                                mem.total, mem.avail, mem.buffers, mem.cached, mem.free, mem.swap_free, mem.swap_total
                            )
                        } else {
                            "".to_string()
                        }
                        */
                    )
                    .trim_end()
                    .to_string(),
                );
            }

            thread::sleep(std::time::Duration::from_millis(20));
            elapsed_milli_secs += 20;
        }
    }

    /// Check level.
    pub fn enabled(level: Level) -> bool {
        if let Ok(logger) = LOGGER.lock() {
            if logger.enabled(level) {
                return true;
            }
        }
        false
    }

    /// Trace level. No trailing newline.
    #[allow(dead_code)]
    pub fn trace(message: &str) {
        if Log::enabled(Level::Trace) {
            Log::reserve(&Table::new(Level::Trace, message, false));
        }
    }

    /// Trace level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn traceln(message: &str) {
        if Log::enabled(Level::Trace) {
            Log::reserve(&Table::new(Level::Trace, message, true));
        }
    }

    /// Trace level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn trace_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Trace) {
            table.level = Level::Trace;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(table);
        }
    }

    /// Trace level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn traceln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Trace) {
            table.level = Level::Trace;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(table);
        }
    }

    /// Debug level. No trailing newline.
    #[allow(dead_code)]
    pub fn debug(message: &str) {
        if Log::enabled(Level::Debug) {
            Log::reserve(&Table::new(Level::Debug, message, false));
        }
    }

    /// Debug level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn debugln(message: &str) {
        if Log::enabled(Level::Debug) {
            Log::reserve(&Table::new(Level::Debug, message, true));
        }
    }

    /// Debug level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn debug_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Debug) {
            table.level = Level::Debug;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(table);
        }
    }

    /// Debug level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn debugln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Debug) {
            table.level = Level::Debug;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(table);
        }
    }

    /// Info level. No trailing newline.
    #[allow(dead_code)]
    pub fn info(message: &str) {
        if Log::enabled(Level::Info) {
            Log::reserve(&Table::new(Level::Info, message, false));
        }
    }

    /// Info level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn infoln(message: &str) {
        if Log::enabled(Level::Info) {
            Log::reserve(&Table::new(Level::Info, message, true));
        }
    }

    /// Info level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn info_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Info) {
            table.level = Level::Info;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(table);
        }
    }

    /// Info level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn infoln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Info) {
            table.level = Level::Info;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(table);
        }
    }
    /// Notice level. No trailing newline.
    #[allow(dead_code)]
    pub fn notice(message: &str) {
        if Log::enabled(Level::Notice) {
            Log::reserve(&Table::new(Level::Notice, message, false));
        }
    }

    /// Notice level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn noticeln(message: &str) {
        if Log::enabled(Level::Notice) {
            Log::reserve(&Table::new(Level::Notice, message, true));
        }
    }
    /// Notice level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn notice_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Notice) {
            table.level = Level::Notice;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(table);
        }
    }

    /// Notice level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn noticeln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Notice) {
            table.level = Level::Notice;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(table);
        }
    }

    /// Warning level. No trailing newline.
    #[allow(dead_code)]
    pub fn warn(message: &str) {
        if Log::enabled(Level::Warn) {
            Log::reserve(&Table::new(Level::Warn, message, false));
        }
    }

    /// Warning level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn warnln(message: &str) {
        if Log::enabled(Level::Warn) {
            Log::reserve(&Table::new(Level::Warn, message, true));
        }
    }

    /// Warning level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn warn_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Warn) {
            table.level = Level::Warn;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(table);
        }
    }

    /// Warning level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn warnln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Warn) {
            table.level = Level::Warn;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(table);
        }
    }

    /// Error level. No trailing newline.
    #[allow(dead_code)]
    pub fn error(message: &str) {
        if Log::enabled(Level::Error) {
            Log::reserve(&Table::new(Level::Error, message, false));
        }
    }

    /// Error level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn errorln(message: &str) {
        if Log::enabled(Level::Error) {
            Log::reserve(&Table::new(Level::Error, message, true));
        }
    }

    /// Error level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn error_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Error) {
            table.level = Level::Error;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(table);
        }
    }

    /// Error level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn errorln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Error) {
            table.level = Level::Error;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(table);
        }
    }
    /// Fatal level. No trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    #[allow(dead_code)]
    pub fn fatal(message: &str) -> String {
        // Fatal runs at any level.
        Log::reserve(&Table::new(Level::Fatal, message, false));
        // Wait for logging to complete or to timeout.
        Log::wait();
        message.to_string()
    }
    /// Fatal level. There is a trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    #[allow(dead_code)]
    pub fn fatalln(message: &str) -> String {
        // Fatal runs at any level.
        Log::reserve(&Table::new(Level::Fatal, message, true));
        // Wait for logging to complete or to timeout.
        Log::wait();
        // Append trailing newline.
        format!("{}{}", message, NEW_LINE).to_string()
    }

    /// Fatal level. No trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    #[allow(dead_code)]
    pub fn fatal_t(message: &str, table: &mut Table) -> String {
        // Fatal runs at any level.
        table.level = Level::Fatal;
        table.message = message.to_string();
        table.message_trailing_newline = false;
        Log::reserve(table);
        // Wait for logging to complete or to timeout.
        Log::wait();
        message.to_string()
    }
    /// Fatal level. There is a trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    #[allow(dead_code)]
    pub fn fatalln_t(message: &str, table: &mut Table) -> String {
        // Fatal runs at any level.
        table.level = Level::Fatal;
        table.message = message.to_string();
        table.message_trailing_newline = true;
        Log::reserve(table);
        // Wait for logging to complete or to timeout.
        Log::wait();
        // Append trailing newline.
        format!("{}{}", message, NEW_LINE).to_string()
    }

    fn reserve(table: &Table) {
        let table_clone = table.clone();

        SEQ.with(move |seq| {
            let header = TableHeader::new(&format!("{:?}", thread::current().id()), *seq.borrow());

            if let Ok(reseve_target) = RESERVE_TARGET.lock() {
                if reseve_target.is_t() {
                    if let Ok(mut queue) = QUEUE_T.lock() {
                        queue.push_front((header, table_clone));
                    }
                } else {
                    if let Ok(mut queue) = QUEUE_F.lock() {
                        queue.push_front((header, table_clone));
                    }
                }
            }

            if let Ok(mut signal) = SIGNAL_CAN_FLUSH.lock() {
                if signal.can_flush() {
                    signal.set_can_flush(false);
                    if let Ok(mut pool) = POOL.lock() {
                        pool.increase_thread_count();
                    }
                    thread::spawn(move || {
                        Log::flush();
                        if let Ok(mut pool) = POOL.lock() {
                            pool.decrease_thread_count();
                        }
                    });
                }
            }

            *seq.borrow_mut() += 1;
        });
    }

    /// Write a some strings from the queue.
    ///
    /// # Returns
    ///
    /// Some(true) - Complete.
    /// Some(false) - Not work.
    /// None - Error.
    fn flush() -> Option<bool> {
        // By buffering, the number of file writes is reduced.
        let mut str_buf = String::new();

        // Switch.
        let flush_target = if let Ok(mut reserve_target) = RESERVE_TARGET.lock() {
            let old = reserve_target.is_t();
            reserve_target.switch();
            old
        } else {
            // TODO Error.
            return None;
        };

        let mut count = 0;
        if flush_target {
            if let Ok(mut queue) = QUEUE_T.lock() {
                loop {
                    if let Some(table_tuple) = queue.pop_back() {
                        str_buf.push_str(&Log::convert_table_to_string(
                            &table_tuple.0,
                            &table_tuple.1,
                        ));
                        count += 1;
                    } else {
                        break;
                    }
                }
            } else {
                // TODO Error.
            }
        } else {
            if let Ok(mut queue) = QUEUE_F.lock() {
                loop {
                    if let Some(table_tuple) = queue.pop_back() {
                        str_buf.push_str(&Log::convert_table_to_string(
                            &table_tuple.0,
                            &table_tuple.1,
                        ));
                        count += 1;
                    } else {
                        break;
                    }
                }
            } else {
                // TODO Error.
            }
        }
        // Flush! (Outside the lock on the Queue.)
        // Write to a log file.
        // This is time consuming and should be done in a separate thread.
        if let Ok(mut logger) = LOGGER.lock() {
            let mut file_buf = BufWriter::new(logger.current_file());
            // write_all method required to use 'use std::io::Write;'.
            if let Err(_why) = file_buf.write_all(str_buf.as_bytes()) {
                // Nothing is output even if log writing fails.
                // Submitting a message to the competition can result in fouls.
                // panic!("couldn't write log. : {}",Error::description(&why)),
                return None;
            }
            if let Ok(mut signal) = SIGNAL_CAN_FLUSH.lock() {
                signal.set_can_flush(true);
            }
        }

        Some(0 < count)
    }

    fn convert_table_to_string(header: &TableHeader, table: &Table) -> String {
        let message = if table.message_trailing_newline {
            // There is a trailing newline.
            format!("{}{}", table.message, NEW_LINE)
        } else {
            table.message.to_string()
        };
        // Write as TOML.
        // Table name.
        let mut toml = format!(
            // Table name to keep for ordering.
            // For example, you can parse it easily by writing the table name like a GET query.
            "[\"Now={}&Pid={}&Thr={}&Seq={}\"]
",
            // If you use ISO8601, It's "%Y-%m-%dT%H:%M:%S%z". However, it does not set the date format.
            // Make it easier to read.
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            // Process ID.
            process::id(),
            // Thread ID. However, Note that you are not limited to numbers.
            header.thread_id,
            // Line number. This is to avoid duplication.
            header.seq,
        );
        toml += &format!(
            "{} = {}
",
            table.level,
            Table::format_str_value(&message).to_string()
        )
        .to_string();
        for (k, v) in &table.sorted_map {
            toml += &format!(
                "{} = {}
",
                k, v
            )
            .to_string();
        }
        toml += "
";
        toml
    }
}

struct Pool {
    /// Number of threads not yet finished.
    thread_count: u32,
}
impl Default for Pool {
    fn default() -> Self {
        Pool { thread_count: 0 }
    }
}
impl Pool {
    fn increase_thread_count(&mut self) {
        self.thread_count += 1;
    }
    fn decrease_thread_count(&mut self) {
        self.thread_count -= 1;
    }
    pub fn get_thread_count(&self) -> u32 {
        self.thread_count
    }
}

/// Configuration.
///
/// All: 'tic-tac-toe-2020-07-11.log.toml'
/// Prefix: 'tic-tac-toe'
/// StartDate: '-2020-07-11'
/// Suffix: '.log'
/// Extention: '.toml'
///
/// If you don't like the .toml extension, leave the suffix empty and the .log extension.
#[deprecated(
    since = "0.4.0",
    note = "Please use the casual_logger::Log::xxxx() methods instead"
)]
pub struct Logger {
    /// For example, the short name of your application.
    file_prefix: String,
    /// For example, '.log'. To be safe, include a word that clearly states that you can delete the file.
    file_suffix: String,
    /// If you don't like the .toml extension, leave the suffix empty and the .log extension.
    file_extention: String,
    /// File retention days. Delete the file after day from StartDate.
    #[deprecated(
        since = "0.3.8",
        note = "Please use the casual_logger::Log::set_retention_days() method instead"
    )]
    pub retention_days: i64,
    /// Controll file.
    log_file: Option<LogFile>,
    /// Activation.
    #[deprecated(
        since = "0.3.7",
        note = "Please use the casual_logger::Log::set_level() method instead"
    )]
    pub level: Level,
    /// Timeout seconds when fatal.
    #[deprecated(
        since = "0.3.2",
        note = "Please use the casual_logger::Log::set_timeout_secs() method instead"
    )]
    pub fatal_timeout_secs: u64,
    /// Timeout seconds.
    #[deprecated(
        since = "0.3.9",
        note = "Please use the casual_logger::Log::set_timeout_secs() method instead"
    )]
    pub timeout_secs: u64,
    /// Set to true to allow Casual_logger to output information to stdout and stderr.
    #[deprecated(
        since = "0.3.10",
        note = "Please use the casual_logger::Log::set_development() method instead"
    )]
    pub development: bool,
}
impl Default for Logger {
    fn default() -> Self {
        let prefix = "default";
        let suffix = ".log";
        let extention = ".toml";
        Logger {
            file_prefix: prefix.to_string(),
            file_suffix: suffix.to_string(),
            file_extention: extention.to_string(),
            retention_days: 7,
            log_file: None,
            level: Level::Trace,
            fatal_timeout_secs: 30,
            timeout_secs: 30,
            development: false,
        }
    }
}
impl Logger {
    fn get_timeout_sec(logger: &Logger) -> u64 {
        if logger.timeout_secs != 30 {
            logger.timeout_secs
        } else if logger.fatal_timeout_secs != 30 {
            logger.fatal_timeout_secs
        } else {
            logger.timeout_secs
        }
    }
    /// Check level.
    pub fn enabled(&self, level: Level) -> bool {
        if level.number() <= self.level.number() {
            return true;
        }
        false
    }

    /// Example:
    ///
    /// If 'tic-tac-toe-2020-07-11.log.toml', This is 'tic-tac-toe'.
    #[allow(dead_code)]
    pub fn get_file_prefix(&self) -> &str {
        &self.file_prefix
    }
    /// Example:
    ///
    /// If 'tic-tac-toe-2020-07-11.log.toml', This is '.log'.
    #[allow(dead_code)]
    pub fn get_file_suffix(&self) -> &str {
        &self.file_suffix
    }
    /// Example:
    ///
    /// If 'tic-tac-toe-2020-07-11.log.toml', This is '.toml'.
    #[allow(dead_code)]
    pub fn get_file_extention(&self) -> &str {
        &self.file_extention
    }
    /// Set name except StartDate.
    ///
    /// Example: 'tic-tac-toe-2020-07-11.log.toml'
    /// - Prefix: 'tic-tac-toe'
    /// - StartDate: '-2020-07-11'
    /// - Suffix: '.log'
    /// - Extention: '.toml'
    #[deprecated(
        since = "0.3.6",
        note = "Please use the casual_logger::Log::set_file_name() or casual_logger::Log::set_toml_ext() method instead"
    )]
    #[allow(dead_code)]
    pub fn set_file_name(&mut self, prefix: &str, suffix: &str, extention: &str) {
        self.file_prefix = prefix.to_string();
        self.file_suffix = suffix.to_string();
        self.file_extention = extention.to_string();
    }
    /// Create new file, or get exists file.
    fn new_today_file(
        file_prefix: &str,
        file_suffix: &str,
        file_extention: &str,
    ) -> (Date<Local>, File) {
        let start_date = Local::today();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            // Example: 'default-2020-07-11.log.toml'.
            .open(Path::new(&format!(
                "{}-{}{}{}",
                file_prefix,
                start_date.format("%Y-%m-%d"),
                file_suffix,
                file_extention
            )))
            .unwrap();
        (start_date, file)
    }
    /// For log rotation.
    #[deprecated(
        since = "0.3.3",
        note = "Please use the casual_logger::Log::remove_old_logs() method instead"
    )]
    pub fn remove_old_logs(&self) -> usize {
        // Removed files count.
        let mut count = 0;
        // Example:
        //      all = './tic-tac-toe-2020-07-11.log.toml'
        //      prefix = "tic-tac-toe"
        //      now = "-2020-07-11"
        //      suffix = ".log"
        //      extention = ".toml"
        let re = if let Ok(x) = Regex::new(&format!(
            "./{}-{}{}{}",
            self.file_prefix, r"(\d{4})-(\d{2})-(\d{2})", self.file_suffix, self.file_extention
        )) {
            x
        } else {
            return 0;
        };
        // file, directory paths:
        let paths = if let Ok(x) = fs::read_dir("./") {
            x
        } else {
            return 0;
        };
        for path in paths {
            let name = if let Ok(x) = path {
                x.path().display().to_string()
            } else {
                continue;
            };
            // File name pattern match:
            if let Some(caps) = re.captures(&name) {
                // Extract year, month, day.
                let year: i32 = if let Some(cap) = caps.get(1) {
                    if let Ok(n) = cap.as_str().parse() {
                        n
                    } else {
                        0
                    }
                } else {
                    0
                };
                let month: u32 = if let Some(cap) = caps.get(2) {
                    if let Ok(n) = cap.as_str().parse() {
                        n
                    } else {
                        0
                    }
                } else {
                    0
                };
                let day: u32 = if let Some(cap) = caps.get(3) {
                    if let Ok(n) = cap.as_str().parse() {
                        n
                    } else {
                        0
                    }
                } else {
                    0
                };
                if month != 0 && day != 0 {
                    let file_date = Local.ymd(year, month, day);

                    // Over the retention days.
                    if file_date.add(Duration::days(self.retention_days)) < Local::today() {
                        // Remove file.
                        if let Ok(_why) = fs::remove_file(name) {
                            // Nothing is output even if log writing fails.
                            // Submitting a message to the competition can result in fouls.
                            // println!("! {:?}", why.kind());
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    /// Get file, or rotation file.
    fn current_file(&mut self) -> &File {
        // Check day.
        let date_changed = if let Some(log_file) = &self.log_file {
            log_file.start_date < Local::today()
        } else {
            false
        };
        // Remove file, if day changed.
        if date_changed {
            self.log_file = None;
        }

        // New file, if file removed or new.
        if let None = self.log_file {
            let (start_date, file) =
                Logger::new_today_file(&self.file_prefix, &self.file_suffix, &self.file_extention);
            self.log_file = Some(LogFile::new(start_date, file));
        }

        // Return file handle.
        &self.log_file.as_ref().unwrap().file
    }
}

/// Used for editing and locking files.
struct LogFile {
    /// Used for file name and deletion. Year, Month, Day.
    pub start_date: Date<Local>,
    /// Used for editing and locking files.
    pub file: File,
}
impl LogFile {
    pub fn new(start_date: Date<Local>, file: File) -> Self {
        LogFile {
            start_date: start_date,
            file: file,
        }
    }
}

/// File extension.
pub enum Extension {
    /// *.log
    Log,
    /// *.log.toml
    LogToml,
}

/// The queue number is a Boolean, not a number.
#[derive(Clone, Copy)]
struct ReserveTarget {
    target: bool,
}
impl Default for ReserveTarget {
    fn default() -> Self {
        ReserveTarget { target: false }
    }
}
impl ReserveTarget {
    fn is_t(self) -> bool {
        self.target
    }
    fn switch(&mut self) {
        self.target = !self.target;
    }
}

struct SignalCanFlush {
    can_flush: bool,
}
impl Default for SignalCanFlush {
    fn default() -> Self {
        SignalCanFlush { can_flush: true }
    }
}
impl SignalCanFlush {
    pub fn can_flush(&self) -> bool {
        self.can_flush
    }
    pub fn set_can_flush(&mut self, val: bool) {
        self.can_flush = val;
    }
}
