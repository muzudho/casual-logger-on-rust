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
use std::fmt;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use std::process;
use std::sync::{mpsc, Mutex};
use std::thread;

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
}
// Use the line number in the log.
//
// Thread local scope variable.
//
// * References
//      * [ミュータブルなスレッドローカルデータを thread_local!() マクロと RefCell で実現する](https://qiita.com/tatsuya6502/items/bed3702517b36afbdbca)
thread_local!(pub static SEQ: RefCell<u128> = {
    RefCell::new(1)
});

#[derive(Clone)]
pub struct Table {
    level: Level,
    message: String,
    sorted_map: BTreeMap<String, String>,
}
impl Default for Table {
    fn default() -> Self {
        Table {
            sorted_map: BTreeMap::new(),
            level: Level::Trace,
            message: "".to_string(),
        }
    }
}
impl Table {
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

// Easy to use logging.
pub struct Log {}
impl Log {
    /// Wait for logging to complete.
    pub fn wait_for_logging_to_complete<F>(timeout_secs: u64, count_down: F)
    where
        F: Fn(u64, u32),
    {
        let mut elapsed_secs = 0;
        while elapsed_secs < timeout_secs {
            if let Ok(pool) = POOL.lock() {
                let num = pool.get_thread_count();
                count_down(elapsed_secs, num);
                if num < 1 {
                    break;
                }
            }

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
            Log::send(Level::Trace, message);
        }
    }

    /// Trace level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn traceln(message: &str) {
        if Log::enabled(Level::Trace) {
            Log::sendln(Level::Trace, message);
        }
    }

    /// Trace level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn trace_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Trace) {
            Log::send_t(Level::Trace, message, table);
        }
    }

    /// Trace level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn traceln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Trace) {
            Log::sendln_t(Level::Trace, message, table);
        }
    }

    /// Debug level. No trailing newline.
    #[allow(dead_code)]
    pub fn debug(message: &str) {
        if Log::enabled(Level::Debug) {
            Log::send(Level::Debug, message);
        }
    }

    /// Debug level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn debugln(message: &str) {
        if Log::enabled(Level::Debug) {
            Log::sendln(Level::Debug, message);
        }
    }

    /// Debug level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn debug_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Debug) {
            Log::send_t(Level::Debug, message, table);
        }
    }

    /// Debug level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn debugln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Debug) {
            Log::sendln_t(Level::Debug, message, table);
        }
    }

    /// Info level. No trailing newline.
    #[allow(dead_code)]
    pub fn info(message: &str) {
        if Log::enabled(Level::Info) {
            Log::send(Level::Info, message);
        }
    }

    /// Info level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn infoln(message: &str) {
        if Log::enabled(Level::Info) {
            Log::sendln(Level::Info, message);
        }
    }

    /// Info level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn info_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Info) {
            Log::send_t(Level::Info, message, table);
        }
    }

    /// Info level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn infoln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Info) {
            Log::sendln_t(Level::Info, message, table);
        }
    }
    /// Notice level. No trailing newline.
    #[allow(dead_code)]
    pub fn notice(message: &str) {
        if Log::enabled(Level::Notice) {
            Log::send(Level::Notice, message);
        }
    }

    /// Notice level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn noticeln(message: &str) {
        if Log::enabled(Level::Notice) {
            Log::sendln(Level::Notice, message);
        }
    }
    /// Notice level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn notice_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Notice) {
            Log::send_t(Level::Notice, message, table);
        }
    }

    /// Notice level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn noticeln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Notice) {
            Log::sendln_t(Level::Notice, message, table);
        }
    }

    /// Warning level. No trailing newline.
    #[allow(dead_code)]
    pub fn warn(message: &str) {
        if Log::enabled(Level::Warn) {
            Log::send(Level::Warn, message);
        }
    }

    /// Warning level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn warnln(message: &str) {
        if Log::enabled(Level::Warn) {
            Log::sendln(Level::Warn, message);
        }
    }

    /// Warning level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn warn_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Warn) {
            Log::send_t(Level::Warn, message, table);
        }
    }

    /// Warning level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn warnln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Warn) {
            Log::sendln_t(Level::Warn, message, table);
        }
    }

    /// Error level. No trailing newline.
    #[allow(dead_code)]
    pub fn error(message: &str) {
        if Log::enabled(Level::Error) {
            Log::send(Level::Error, message);
        }
    }

    /// Error level. There is a trailing newline.
    #[allow(dead_code)]
    pub fn errorln(message: &str) {
        if Log::enabled(Level::Error) {
            Log::sendln(Level::Error, message);
        }
    }

    /// Error level. No trailing newline. Use table.
    #[allow(dead_code)]
    pub fn error_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Error) {
            Log::send_t(Level::Error, message, table);
        }
    }

    /// Error level. There is a trailing newline. Use table.
    #[allow(dead_code)]
    pub fn errorln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Error) {
            Log::sendln_t(Level::Error, message, table);
        }
    }
    /// Fatal level. No trailing newline.
    /// 'panic!' Pass this as the first argument.
    #[allow(dead_code)]
    pub fn fatal(message: &str) -> String {
        // Fatal runs at any level.
        Log::send(Level::Fatal, message);
        message.to_string()
    }
    /// Fatal level. There is a trailing newline.
    /// 'panic!' Pass this as the first argument.
    #[allow(dead_code)]
    pub fn fatalln(message: &str) -> String {
        // Fatal runs at any level.
        Log::sendln(Level::Fatal, message);
        // Append trailing newline.
        format!("{}{}", message, NEW_LINE).to_string()
    }

    /// Fatal level. No trailing newline.
    /// 'panic!' Pass this as the first argument.
    #[allow(dead_code)]
    pub fn fatal_t(message: &str, table: &mut Table) -> String {
        // Fatal runs at any level.
        Log::send_t(Level::Fatal, message, table);
        message.to_string()
    }
    /// Fatal level. There is a trailing newline.
    /// 'panic!' Pass this as the first argument.
    #[allow(dead_code)]
    pub fn fatalln_t(message: &str, table: &mut Table) -> String {
        // Fatal runs at any level.
        Log::sendln_t(Level::Fatal, message, table);
        // Append trailing newline.
        format!("{}{}", message, NEW_LINE).to_string()
    }

    /// Write to a log file. There is a trailing newline.
    #[allow(dead_code)]
    fn writeln(thread_id: &str, seq: u128, s: &str, level: Level, table: &Table) {
        let s = &format!("{}{}", s, NEW_LINE);
        Log::write(thread_id, seq, s, level, table);
    }
    /// Write to a log file. No trailing newline.
    #[allow(dead_code)]
    fn write(thread_id: &str, seq: u128, s: &str, level: Level, table: &Table) {
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
            thread_id,
            // Line number. This is to avoid duplication.
            seq,
        );
        toml += &format!(
            "{} = {}
",
            level,
            Table::format_str_value(s).to_string()
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
        if let Ok(mut logger) = LOGGER.lock() {
            // write_all method required to use 'use std::io::Write;'.
            if let Err(_why) = logger.current_file().write_all(toml.as_bytes()) {
                // Nothing is output even if log writing fails.
                // Submitting a message to the competition can result in fouls.
                // panic!("couldn't write log. : {}",Error::description(&why)),
            }
        }
    }

    pub fn send(level: Level, message: &str) {
        let thread_id = format!("{:?}", thread::current().id());
        let message_str = message.to_string();
        SEQ.with(|seq| {
            let (sender, receiver) = mpsc::channel();
            thread::spawn(move || {
                if let Ok(seq_clone) = receiver.recv() {
                    let mut table = Table::default();
                    Log::write(&thread_id, seq_clone, &message_str, level, &mut table);
                }
            });
            let seq_clone = seq.borrow().clone();
            if let Err(_) = sender.send(seq_clone) {
                // ignore
            }
            *seq.borrow_mut() += 1;
        });
    }

    pub fn sendln(level: Level, message: &str) {
        let thread_id = format!("{:?}", thread::current().id());
        let message_str = message.to_string();
        if let Ok(mut pool) = POOL.lock() {
            pool.increase_thread_count();
        }

        SEQ.with(|seq| {
            let (sender, receiver) = mpsc::channel();
            thread::spawn(move || {
                if let Ok(seq_clone) = receiver.recv() {
                    let mut table = Table::default();
                    Log::writeln(&thread_id, seq_clone, &message_str, level, &mut table);

                    if let Ok(mut pool) = POOL.lock() {
                        pool.decrease_thread_count();
                    }
                }
            });

            let seq_clone = seq.borrow().clone();
            if let Err(_) = sender.send(seq_clone) {
                // ignore
            }
            *seq.borrow_mut() += 1;
        });
    }

    pub fn send_t(level: Level, message: &str, table: &mut Table) {
        let thread_id = format!("{:?}", thread::current().id());
        let message_str = message.to_string();
        let table_clone = table.clone();
        SEQ.with(move |seq| {
            let (sender, receiver) = mpsc::channel();
            thread::spawn(move || {
                if let Ok(seq_clone) = receiver.recv() {
                    Log::write(&thread_id, seq_clone, &message_str, level, &table_clone);
                }
            });
            let seq_clone = seq.borrow().clone();
            if let Err(_) = sender.send(seq_clone) {
                // ignore
            }
            *seq.borrow_mut() += 1;
        });
    }

    pub fn sendln_t(level: Level, message: &str, table: &mut Table) {
        let thread_id = format!("{:?}", thread::current().id());
        let message_str = message.to_string();
        let table_clone = table.clone();
        SEQ.with(move |seq| {
            let (sender, receiver) = mpsc::channel();
            thread::spawn(move || {
                if let Ok(seq_clone) = receiver.recv() {
                    Log::writeln(&thread_id, seq_clone, &message_str, level, &table_clone);
                }
            });
            let seq_clone = seq.borrow().clone();
            if let Err(_) = sender.send(seq_clone) {
                // ignore
            }
            *seq.borrow_mut() += 1;
        });
    }
}

pub struct Pool {
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

/// Examples:
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
        }
    }
}
impl Logger {
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
    #[allow(dead_code)]
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
                        } else {
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
