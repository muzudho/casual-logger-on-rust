//! This logger is intended to be easy to explain when teaching other example programs to friends.
//! Not for you, for self-study of beginner friends.
//! Of course you can use it.
//! Not for production, but better than not logging.

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

use chrono::{Date, Duration, Local, TimeZone};
use regex::Regex;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use std::process;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

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
    /// Logger grobal variable.
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::default());
    /// Wait for logging to complete.
    static ref POOL: Mutex<Pool> = Mutex::new(Pool::default());
    /// Wait for logging to complete.
    static ref QUEUE: Mutex<VecDeque<Table>> = Mutex::new(VecDeque::<Table>::new());
    /// Erapsed time.
    static ref LAST_FLUSH_TIME: Mutex<LastFlushTime> = Mutex::new(LastFlushTime::default());
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

/// TOML table included in the log file. Do not validate.
#[derive(Clone)]
pub struct Table {
    /// Thread ID. However, Note that you are not limited to numbers.
    thread_id: String,
    seq: u128,
    level: Level,
    message: String,
    message_trailing_newline: bool,
    sorted_map: BTreeMap<String, String>,
}
impl Default for Table {
    fn default() -> Self {
        Table {
            thread_id: "".to_string(),
            seq: 0,
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
            thread_id: "".to_string(),
            seq: 0,
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
    /// # Returns
    ///
    /// Number of deleted log files.
    pub fn remove_old_logs() -> usize {
        let remove_num = if let Ok(logger) = LOGGER.lock() {
            // Do not call 'Log::xxxxx()' in this code block.
            let remove_num = logger.remove_old_logs();
            if logger.development {
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
    /// See also: logger.timeout_secs, Logger.development.
    pub fn wait() {
        let (timeout_secs, development) = if let Ok(logger) = LOGGER.lock() {
            (Logger::get_timeout_sec(&logger), logger.development)
        } else {
            (0, false)
        };

        Log::wait_for_logging_to_complete(timeout_secs, |secs, message| {
            // Do not call 'Log::xxxxx()' in this code block.
            if development {
                eprintln!("casual_logger: {} sec(s). {}", secs, message);
            }
        });
    }
    /// Wait for logging to complete.
    #[deprecated(since = "0.3.2", note = "Please use the wait method instead")]
    pub fn wait_for_logging_to_complete<F>(timeout_secs: u64, count_down: F)
    where
        F: Fn(u64, String),
    {
        let mut elapsed_secs = 0;
        while elapsed_secs < timeout_secs {
            let mut thr_num = None;
            let mut queue_len = None;
            if let Ok(pool) = POOL.lock() {
                let thr_num_val = pool.get_thread_count();
                thr_num = Some(thr_num_val);
                if thr_num_val < 1 {
                    if let Ok(queue) = QUEUE.lock() {
                        if queue.is_empty() {
                            // count_down(elapsed_secs, "Completed.".to_string());
                            break;
                        }
                        queue_len = Some(queue.len());
                    }

                    // Out of QUEUE.lock().
                    Log::flush();
                }
            }
            count_down(
                elapsed_secs,
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
                    }
                )
                .trim_end()
                .to_string(),
            );

            thread::sleep(std::time::Duration::from_secs(1));
            elapsed_secs += 1;
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
            Log::send(&Table::new(Level::Trace, message, false));
        }
    }

    /// Trace level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn traceln(message: &str) {
        if Log::enabled(Level::Trace) {
            Log::send(&Table::new(Level::Trace, message, true));
        }
    }

    /// Trace level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn trace_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Trace) {
            table.level = Level::Trace;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::send(table);
        }
    }

    /// Trace level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn traceln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Trace) {
            table.level = Level::Trace;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::send(table);
        }
    }

    /// Debug level. No trailing newline.
    #[allow(dead_code)]
    pub fn debug(message: &str) {
        if Log::enabled(Level::Debug) {
            Log::send(&Table::new(Level::Debug, message, false));
        }
    }

    /// Debug level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn debugln(message: &str) {
        if Log::enabled(Level::Debug) {
            Log::send(&Table::new(Level::Debug, message, true));
        }
    }

    /// Debug level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn debug_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Debug) {
            table.level = Level::Debug;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::send(table);
        }
    }

    /// Debug level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn debugln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Debug) {
            table.level = Level::Debug;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::send(table);
        }
    }

    /// Info level. No trailing newline.
    #[allow(dead_code)]
    pub fn info(message: &str) {
        if Log::enabled(Level::Info) {
            Log::send(&Table::new(Level::Info, message, false));
        }
    }

    /// Info level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn infoln(message: &str) {
        if Log::enabled(Level::Info) {
            Log::send(&Table::new(Level::Info, message, true));
        }
    }

    /// Info level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn info_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Info) {
            table.level = Level::Info;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::send(table);
        }
    }

    /// Info level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn infoln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Info) {
            table.level = Level::Info;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::send(table);
        }
    }
    /// Notice level. No trailing newline.
    #[allow(dead_code)]
    pub fn notice(message: &str) {
        if Log::enabled(Level::Notice) {
            Log::send(&Table::new(Level::Notice, message, false));
        }
    }

    /// Notice level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn noticeln(message: &str) {
        if Log::enabled(Level::Notice) {
            Log::send(&Table::new(Level::Notice, message, true));
        }
    }
    /// Notice level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn notice_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Notice) {
            table.level = Level::Notice;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::send(table);
        }
    }

    /// Notice level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn noticeln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Notice) {
            table.level = Level::Notice;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::send(table);
        }
    }

    /// Warning level. No trailing newline.
    #[allow(dead_code)]
    pub fn warn(message: &str) {
        if Log::enabled(Level::Warn) {
            Log::send(&Table::new(Level::Warn, message, false));
        }
    }

    /// Warning level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn warnln(message: &str) {
        if Log::enabled(Level::Warn) {
            Log::send(&Table::new(Level::Warn, message, true));
        }
    }

    /// Warning level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn warn_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Warn) {
            table.level = Level::Warn;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::send(table);
        }
    }

    /// Warning level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn warnln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Warn) {
            table.level = Level::Warn;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::send(table);
        }
    }

    /// Error level. No trailing newline.
    #[allow(dead_code)]
    pub fn error(message: &str) {
        if Log::enabled(Level::Error) {
            Log::send(&Table::new(Level::Error, message, false));
        }
    }

    /// Error level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn errorln(message: &str) {
        if Log::enabled(Level::Error) {
            Log::send(&Table::new(Level::Error, message, true));
        }
    }

    /// Error level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn error_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Error) {
            table.level = Level::Error;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::send(table);
        }
    }

    /// Error level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn errorln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Error) {
            table.level = Level::Error;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::send(table);
        }
    }
    /// Fatal level. No trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    #[allow(dead_code)]
    pub fn fatal(message: &str) -> String {
        // Fatal runs at any level.
        Log::send(&Table::new(Level::Fatal, message, false));
        // Wait for logging to complete or to timeout.
        Log::wait();
        message.to_string()
    }
    /// Fatal level. There is a trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    #[allow(dead_code)]
    pub fn fatalln(message: &str) -> String {
        // Fatal runs at any level.
        Log::send(&Table::new(Level::Fatal, message, true));
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
        Log::send(table);
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
        Log::send(table);
        // Wait for logging to complete or to timeout.
        Log::wait();
        // Append trailing newline.
        format!("{}{}", message, NEW_LINE).to_string()
    }

    fn send(table: &Table) {
        let mut table_clone = table.clone();
        table_clone.thread_id = format!("{:?}", thread::current().id());

        SEQ.with(move |seq| {
            table_clone.seq = seq.borrow().clone();

            if let Ok(mut queue) = QUEUE.lock() {
                queue.push_front(table_clone);
            }

            let can_flush = if let Ok(last_flush_time) = LAST_FLUSH_TIME.lock() {
                last_flush_time.can_flush()
            } else {
                false
            };

            if can_flush {
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
            *seq.borrow_mut() += 1;
        });
    }

    /// Write a some strings from the queue.
    fn flush() {
        // Flush! (Outside the lock on the Queue.)
        // Write to a log file.
        // This is time consuming and should be done in a separate thread.
        if let Ok(mut logger) = LOGGER.lock() {
            if let Ok(mut queue) = QUEUE.lock() {
                // By buffering, the number of file writes is reduced.
                let mut toml = String::new();

                // However, it ends with 50 tables.
                // TODO I want to automatically adjust how good it is.
                let stopwatch = Instant::now();
                while stopwatch.elapsed().as_secs() < 2 {
                    if let Some(table) = queue.pop_back() {
                        toml.push_str(&Log::convert_table_to_string(&table));
                    } else {
                        break;
                    }
                }
                // write_all method required to use 'use std::io::Write;'.
                if let Err(_why) = logger.current_file().write_all(toml.as_bytes()) {
                    // Nothing is output even if log writing fails.
                    // Submitting a message to the competition can result in fouls.
                    // panic!("couldn't write log. : {}",Error::description(&why)),
                }
            }
        }

        if let Ok(mut last_flush_time) = LAST_FLUSH_TIME.lock() {
            last_flush_time.reset();
        }
    }

    fn convert_table_to_string(table: &Table) -> String {
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
            table.thread_id,
            // Line number. This is to avoid duplication.
            table.seq,
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
pub struct Logger {
    /// For example, the short name of your application.
    file_prefix: String,
    /// For example, '.log'. To be safe, include a word that clearly states that you can delete the file.
    file_suffix: String,
    /// If you don't like the .toml extension, leave the suffix empty and the .log extension.
    file_extention: String,
    /// File retention days. Delete the file after day from StartDate.
    pub retention_days: i64,
    /// Controll file.
    log_file: Option<LogFile>,
    /// Activation.
    pub level: Level,
    /// Timeout seconds when fatal.
    #[deprecated(since = "0.3.2", note = "Please use the timeout_secs property instead")]
    pub fatal_timeout_secs: u64,
    /// Timeout seconds.
    pub timeout_secs: u64,
    /// Set to true to allow Casual_logger to output information to stdout and stderr.
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
    ///
    /// # Examples
    ///
    /// ```
    /// use casual_logger::{Level, Logger};
    /// use std::sync::Mutex;
    ///
    /// let mut logger = Logger::default();
    /// logger.set_file_name("casual-log-test", ".log", ".toml");
    /// if logger.enabled(Level::Trace) {
    ///     assert_eq!(true, true);
    /// } else {
    ///     assert_eq!(true, false);
    /// }
    ///
    /// logger.level = Level::Debug;
    /// if logger.enabled(Level::Trace) {
    ///     assert_eq!(true, false);
    /// } else {
    ///     assert_eq!(true, true);
    /// }
    /// ```
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
    /// Examples:
    ///
    /// All: 'tic-tac-toe-2020-07-11.log.toml'
    /// Prefix: 'tic-tac-toe'
    /// StartDate: '-2020-07-11'
    /// Suffix: '.log'
    /// Extention: '.toml'
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
        note = "Please use the Log::remove_old_logs() method instead"
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

struct LastFlushTime {
    pub last_flush_time: Instant,
}
impl Default for LastFlushTime {
    fn default() -> Self {
        LastFlushTime {
            last_flush_time: Instant::now(),
        }
    }
}
impl LastFlushTime {
    fn reset(&mut self) {
        if 1 <= self.last_flush_time.elapsed().as_secs() {
            self.last_flush_time = Instant::now();
        }
    }
    fn can_flush(&self) -> bool {
        // println!("elapsed={}", self.last_flush_time.elapsed().as_secs());
        1 <= self.last_flush_time.elapsed().as_secs()
    }
}
