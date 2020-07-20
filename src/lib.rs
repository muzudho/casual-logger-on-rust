//! This logger with **few settings** to repeat practice of many programming tutorials.  
//! Not for product use.  

// Publish:
//
// (1) `cargo test`
// (2a) `cargo run --example example1`
// (2b) `cargo run --example example2`
// (2c) `cargo run --example fatal`
// (2d) `cargo run --example important`
// (2e) `cargo run --example overall`
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

mod parser;

use crate::parser::Parser;
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
const NEW_LINE_ESCAPED_CHARS: &'static [char; 4] = &['\\', 'r', '\\', 'n'];
#[cfg(not(windows))]
const NEW_LINE: &'static str = "\n";
#[cfg(not(windows))]
const NEW_LINE_ESCAPED_CHARS: &'static [char; 2] = &['\\', 'n'];

/// The higher this level, the more will be omitted.  
///
/// |<-- Low Level ------------------------- High level -->|  
/// |<-- High priority ------------------- Low priority -->|  
/// | Fatal < Error < Warn < Notice < Info < Debug < Trace |  
#[derive(Clone, Copy)]
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
    /// Table buffer.
    static ref QUEUE_T: Mutex<VecDeque<(InternalTable)>> = Mutex::new(VecDeque::<(InternalTable)>::new());
    static ref QUEUE_F: Mutex<VecDeque<(InternalTable)>> = Mutex::new(VecDeque::<(InternalTable)>::new());
    static ref RESERVE_TARGET: Mutex<ReserveTarget> = Mutex::new(ReserveTarget::default());
    static ref SIGNAL_CAN_FLUSH: Mutex<SignalCanFlush> = Mutex::new(SignalCanFlush::default());
    /// Without dot.
    static ref RE_TOML_KEY: Mutex<Regex> = Mutex::new(Regex::new(r"^[A-Za-z0-9_-]+$").unwrap());
    static ref RE_WHITE_SPACE: Mutex<Regex> = Mutex::new(Regex::new(r"\s").unwrap());
    // Wait for logging to complete.
    //static ref PARTICIPANTING_THREADS_COUNTER: Mutex<ParticipatingThreadsCounter> = Mutex::new(ParticipatingThreadsCounter::default());
    /// Optimization.
    static ref OPT_STATE: Mutex<OptState> = Mutex::new(OptState::default());
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

struct InternalTable {
    /// Automatic. Thread ID. However, Note that you are not limited to numbers.
    thread_id: String,
    /// Automatic.
    seq: u128,
    /// Clone.
    table: Table,
}
impl InternalTable {
    fn new(thread_id: &str, seq: u128, table: &Table) -> Self {
        InternalTable {
            thread_id: thread_id.to_string(),
            seq: seq,
            table: table.clone(),
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
    /*
    pub fn convert_multi_byte_string(value: &str) -> String {
        let bytes: &[u8] = value.as_bytes();
        // convert bytes => str
        // let res = bytes.iter().map(|&s| s as char).collect::<String>();
        let converted: String = if let Ok(converted) = String::from_utf8(bytes.to_vec()) {
            converted
        } else {
            value.to_string()
        };
        println!(
            "Value=|{}|{}| Converted=|{}|{}|",
            value,
            value.len(),
            converted,
            converted.len()
        );
        converted
    }
    */
    #[deprecated(since = "0.4.1", note = "This is private method")]
    pub fn format_str_value(value: &str) -> String {
        Parser::format_str_value(value)
    }
    /// Correct the key automatically.
    fn correct_key(key: &str) -> String {
        if let Ok(logger) = LOGGER.lock() {
            match Logger::get_optimization(&logger) {
                Opt::Release => {
                    return key.to_string();
                }
                _ => {}
            }
        };

        // Check
        // TODO Dotted key support is difficult.
        if let Ok(re_toml_key) = RE_TOML_KEY.lock() {
            if re_toml_key.is_match(key) {
                // Ok.
                return key.to_string();
            }
        }

        // TODO Auto correct
        if let Ok(re_white_space) = RE_WHITE_SPACE.lock() {
            format!(
                "\"{}\"",
                Parser::escape_double_quotation(&re_white_space.replace_all(key, " "))
            )
        } else {
            // TODO Error
            key.to_string()
        }
    }
    /// Insert string value.
    pub fn str<'a>(&'a mut self, key: &'a str, value: &'a str) -> &'a mut Self {
        self.sorted_map.insert(
            // Log detail level.
            Table::correct_key(key),
            // Message.
            Table::format_str_value(value).to_string(),
        );

        self
    }
    /// Insert literal string value.
    pub fn literal<'a>(&'a mut self, key: &'a str, value: &'a str) -> &'a mut Self {
        self.sorted_map.insert(
            // Log detail level.
            Table::correct_key(key),
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
            if !logger.file_name_important {
                logger.file_prefix = prefix.to_string();
            }
        }
    }

    /// Example:  
    ///
    /// If 'tic-tac-toe-2020-07-11.log.toml', This is 'tic-tac-toe'.  
    pub fn get_file_name() -> Result<String, String> {
        match LOGGER.lock() {
            Ok(logger) => Ok(logger.file_prefix.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// The file name cannot be changed later.  
    /// ファイル名は後で変更できません。  
    ///
    /// See also: `Log::set_file_name()`.  
    pub fn set_file_name_important(prefix: &str) {
        Log::set_file_name(prefix);
        if let Ok(mut logger) = LOGGER.lock() {
            logger.file_name_important = true;
        }
    }

    /// Log file extension.  
    /// '.log.toml' or '.log'.  
    /// If you don't like the .toml extension, change.  
    pub fn set_file_ext(ext: Extension) {
        if let Ok(mut logger) = LOGGER.lock() {
            if !logger.file_ext_important {
                match ext {
                    Extension::LogToml => {
                        logger.file_suffix = ".log".to_string();
                        logger.file_extension = ".toml".to_string();
                    }
                    Extension::Log => {
                        logger.file_suffix = "".to_string();
                        logger.file_extension = ".log".to_string();
                    }
                }
            }
        }
    }

    /// The file extension cannot be changed later.  
    /// ファイル名は後で変更できません。  
    ///
    /// See also: `Log::set_file_ext()`.  
    pub fn set_file_ext_important(ext: Extension) {
        Log::set_file_ext(ext);
        if let Ok(mut logger) = LOGGER.lock() {
            logger.file_ext_important = true;
        }
    }

    /// Example:  
    ///
    /// If 'tic-tac-toe-2020-07-11.log.toml', This is '.log.toml'.  
    pub fn get_file_ext_str() -> Result<String, String> {
        match LOGGER.lock() {
            Ok(logger) => Ok(format!("{}{}", logger.file_suffix, logger.file_extension)),
            Err(e) => Err(e.to_string()),
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
            if !logger.level_important {
                logger.level = level;
            }
        }
    }

    /// The level cannot be changed later.  
    /// レベルは後で変更できません。  
    ///
    /// See also: `Log::set_level()`.  
    pub fn set_level_important(level: Level) {
        Log::set_level(level);
        if let Ok(mut logger) = LOGGER.lock() {
            logger.level_important = true;
        }
    }

    /// Example:  
    ///
    /// If 'tic-tac-toe-2020-07-11.log.toml', This is '.log.toml'.  
    pub fn get_level() -> Result<Level, String> {
        match LOGGER.lock() {
            Ok(logger) => Ok(logger.level),
            Err(e) => Err(e.to_string()),
        }
    }

    /// You probably don't need to set this. Default: 7.  
    /// Check the StartDate in the file name and delete it if it is old.  
    pub fn set_retention_days(days: u32) {
        if let Ok(mut logger) = LOGGER.lock() {
            if !logger.retention_days_important {
                logger.retention_days = days as i64;
            }
        }
    }

    /// The file retention days cannot be changed later.  
    /// ファイル保持日数は後で変更できません。  
    ///
    /// See also: `Log::set_retention_days()`.  
    pub fn set_retention_days_important(retention_days: u32) {
        Log::set_retention_days(retention_days);
        if let Ok(mut logger) = LOGGER.lock() {
            logger.retention_days_important = true;
        }
    }

    /// The file retention days.  
    /// ファイル保持日数。  
    pub fn get_retention_days() -> Result<u32, String> {
        match LOGGER.lock() {
            Ok(logger) => Ok(logger.retention_days as u32),
            Err(e) => Err(e.to_string()),
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
    #[deprecated(
        since = "0.4.7",
        note = "Please use the casual_logger::Log::set_opt(Opt::Development) method instead"
    )]
    pub fn set_development(during_development: bool) {
        if let Ok(mut logger) = LOGGER.lock() {
            logger.development = during_development;
        }
    }

    /// Optimization.
    pub fn set_opt(optimization: Opt) {
        if let Ok(mut opt_state) = OPT_STATE.lock() {
            opt_state.set(optimization);
        }
    }

    /// # Returns
    ///
    /// Number of deleted log files.
    pub fn remove_old_logs() -> usize {
        let remove_num = if let Ok(logger) = LOGGER.lock() {
            // Do not call 'Log::xxxxx()' in this code block.
            let remove_num = logger.remove_old_logs();
            match Logger::get_optimization(&logger) {
                Opt::Development => {
                    if 0 < remove_num {
                        println!("casual_logger: Remove {} log file(s).", remove_num);
                    }
                }
                _ => {}
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
    /// See also: Log::set_timeout_secs(), Log::set_opt().  
    #[deprecated(
        since = "0.5.1",
        note = "Please use the casual_logger::Log::flush() method instead"
    )]
    pub fn wait() {
        Log::flush();
    }

    /// Wait for logging to complete.  
    ///
    /// See also: Log::set_timeout_secs(), Log::set_opt().  
    pub fn flush() {
        let (timeout_secs, opt) = if let Ok(logger) = LOGGER.lock() {
            (
                Logger::get_timeout_sec(&logger),
                Logger::get_optimization(&logger),
            )
        } else {
            // Error
            (0, Opt::BeginnersSupport)
        };

        Log::wait_for_logging_to_complete(timeout_secs, |secs, message| {
            // Do not call 'Log::xxxxx()' in this code block.
            match opt {
                Opt::Development => {
                    eprintln!("casual_logger: {} sec(s). {}", secs, message,);
                }
                _ => {}
            }
        });
    }

    fn print_message(queue_len: Option<usize>) -> String {
        format!(
            "{}",
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
        .to_string()
    }
    /// Wait for logging to complete.
    #[deprecated(
        since = "0.3.2",
        note = "Please use the casual_logger::Log::flush() method instead"
    )]
    pub fn wait_for_logging_to_complete<F>(timeout_secs: u64, count_down: F)
    where
        F: Fn(u64, String),
    {
        let mut elapsed_milli_secs = 0;
        let mut empty_que_count = 0;
        // let mut participating_threads_count = 0;
        // || 0 < participating_threads_count
        while empty_que_count < 2 && elapsed_milli_secs < timeout_secs * 1000 {
            let mut queue_len = None;
            if let Ok(reserve_target) = RESERVE_TARGET.lock() {
                if reserve_target.is_t() {
                    if let Ok(queue) = QUEUE_T.lock() {
                        if queue.is_empty() {
                            // Completed.
                            break;
                        }
                        queue_len = Some(queue.len());
                    }
                } else {
                    if let Ok(queue) = QUEUE_F.lock() {
                        if queue.is_empty() {
                            // Completed.
                            break;
                        }
                        queue_len = Some(queue.len());
                    }
                }
            }

            // Out of QUEUE.lock().
            if let Some(completed) = Log::flush_target_queue() {
                if completed {
                    // Reset.
                    empty_que_count = 0;
                } else {
                    empty_que_count += 1;
                }
            } else {
                // TODO Error.
                // Reset.
                empty_que_count = 0;
            }
            if elapsed_milli_secs % 1000 == 0 {
                count_down(elapsed_milli_secs / 1000, Log::print_message(queue_len));
            }

            /*
            participating_threads_count =
                if let Ok(participating_threads_counter) = PARTICIPANTING_THREADS_COUNTER.lock() {
                    participating_threads_counter.get_thread_count()
                } else {
                    // Error
                    0
                };
            */
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
        Log::flush();
        message.to_string()
    }
    /// Fatal level. There is a trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    #[allow(dead_code)]
    pub fn fatalln(message: &str) -> String {
        // Fatal runs at any level.
        Log::reserve(&Table::new(Level::Fatal, message, true));
        // Wait for logging to complete or to timeout.
        Log::flush();
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
        Log::flush();
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
        Log::flush();
        // Append trailing newline.
        format!("{}{}", message, NEW_LINE).to_string()
    }

    fn reserve(table: &Table) {
        /*
        if let Ok(mut participating_threads_counter) = PARTICIPANTING_THREADS_COUNTER.lock() {
            participating_threads_counter.increase_thread_count();
        }
        */

        let seq = SEQ.with(move |seq| {
            let old = *seq.borrow();
            *seq.borrow_mut() += 1;
            old
        });

        // Out side of SEQ.with().
        let internal_table =
            InternalTable::new(&format!("{:?}", thread::current().id()), seq, &table);

        if let Ok(reseve_target) = RESERVE_TARGET.lock() {
            if reseve_target.is_t() {
                if let Ok(mut queue) = QUEUE_T.lock() {
                    queue.push_front(internal_table);
                }
            } else {
                if let Ok(mut queue) = QUEUE_F.lock() {
                    queue.push_front(internal_table);
                }
            }
        } else {
            // TODO Error
            return;
        }

        /*
        if let Ok(mut participating_threads_counter) = PARTICIPANTING_THREADS_COUNTER.lock() {
            participating_threads_counter.decrease_thread_count();
        }
        */

        if let Ok(mut signal) = SIGNAL_CAN_FLUSH.lock() {
            if signal.can_flush() {
                signal.set_can_flush(false);
                thread::spawn(move || {
                    Log::flush_target_queue();
                });
            }
        }
    }

    /// Write a some strings from the queue.
    ///
    /// # Returns
    ///
    /// Some(true) - Complete.
    /// Some(false) - Not work.
    /// None - Error.
    fn flush_target_queue() -> Option<bool> {
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
                    if let Some(internal_table) = queue.pop_back() {
                        str_buf.push_str(&Log::convert_table_to_string(&internal_table));
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
                    if let Some(internal_table) = queue.pop_back() {
                        str_buf.push_str(&Log::convert_table_to_string(&internal_table));
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

    fn convert_table_to_string(wrapper: &InternalTable) -> String {
        let message = if wrapper.table.message_trailing_newline {
            // There is a trailing newline.
            format!("{}{}", wrapper.table.message, NEW_LINE)
        } else {
            wrapper.table.message.to_string()
        };
        // Write as TOML.
        // Table name.
        let mut toml = format!(
            // Table name to keep for ordering.
            // For example, you can parse it easily by writing the table name like a GET query.
            "[\"Now={}&Pid={}&Thr={}&Seq={}\"]
{} = {}
",
            // If you use ISO8601, It's "%Y-%m-%dT%H:%M:%S%z". However, it does not set the date format.
            // Make it easier to read.
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            // Process ID.
            process::id(),
            // Thread ID. However, Note that you are not limited to numbers.
            wrapper.thread_id,
            // Line number. This is to avoid duplication.
            wrapper.seq,
            wrapper.table.level,
            Table::format_str_value(&message)
        )
        .to_string();
        for (k, formatted_v) in &wrapper.table.sorted_map {
            toml.push_str(&format!(
                "{} = {}
",
                k, formatted_v
            ));
        }
        toml.push_str(
            "
",
        );
        toml
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
    /// The file name cannot be changed later.  
    /// ファイル名は後で変更できません。  
    file_name_important: bool,
    /// For example, the short name of your application.
    file_prefix: String,
    /// The file suffix, extension cannot be changed later.  
    /// 接尾辞、拡張子は後で変更できません。  
    file_ext_important: bool,
    /// For example, '.log'. To be safe, include a word that clearly states that you can delete the file.
    file_suffix: String,
    /// If you don't like the .toml extension, leave the suffix empty and the .log extension.
    file_extension: String,
    /// The level cannot be changed later.  
    /// レベルは後で変更できません。  
    level_important: bool,
    /// Activation.
    #[deprecated(
        since = "0.3.7",
        note = "Please use the casual_logger::Log::set_level() method instead"
    )]
    pub level: Level,
    /// The file retention days cannot be changed later.  
    /// ファイル保持日数は後で変更できません。  
    retention_days_important: bool,
    /// File retention days. Delete the file after day from StartDate.
    #[deprecated(
        since = "0.3.8",
        note = "Please use the casual_logger::Log::set_retention_days() method instead"
    )]
    pub retention_days: i64,
    /// Controll file.
    log_file: Option<LogFile>,
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
        note = "Please use the casual_logger::Log::set_opt(Opt::Development) method instead"
    )]
    pub development: bool,
}
impl Default for Logger {
    fn default() -> Self {
        let prefix = "default";
        let suffix = ".log";
        let extension = ".toml";
        Logger {
            file_name_important: false,
            file_prefix: prefix.to_string(),
            file_ext_important: false,
            file_suffix: suffix.to_string(),
            file_extension: extension.to_string(),
            level_important: false,
            level: Level::Trace,
            retention_days_important: false,
            retention_days: 7,
            timeout_secs: 30,
            fatal_timeout_secs: 30,
            development: false,
            log_file: None,
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

    fn get_optimization(logger: &Logger) -> Opt {
        if logger.development == true {
            Opt::Development
        } else {
            if let Ok(opt_state) = OPT_STATE.lock() {
                opt_state.get()
            } else {
                // Error
                Opt::BeginnersSupport
            }
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
    #[deprecated(
        since = "0.5.2",
        note = "Please use the casual_logger::Log::get_file_name() method instead"
    )]
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
    pub fn get_file_extension(&self) -> &str {
        &self.file_extension
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
    pub fn set_file_name(&mut self, prefix: &str, suffix: &str, extension: &str) {
        if !self.file_name_important {
            self.file_prefix = prefix.to_string();
        }
        if !self.file_ext_important {
            self.file_suffix = suffix.to_string();
            self.file_extension = extension.to_string();
        }
    }
    /// Create new file, or get exists file.  
    fn new_today_file(
        file_prefix: &str,
        file_suffix: &str,
        file_extension: &str,
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
                file_extension
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
        //      extension = ".toml"
        let re = if let Ok(x) = Regex::new(&format!(
            "./{}-{}{}{}",
            self.file_prefix, r"(\d{4})-(\d{2})-(\d{2})", self.file_suffix, self.file_extension
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
                Logger::new_today_file(&self.file_prefix, &self.file_suffix, &self.file_extension);
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

/*
struct ParticipatingThreadsCounter {
    /// Number of threads not yet finished.
    thread_count: u32,
}
impl Default for ParticipatingThreadsCounter {
    fn default() -> Self {
        ParticipatingThreadsCounter { thread_count: 0 }
    }
}
impl ParticipatingThreadsCounter {
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
*/

/// Optimization.
#[derive(Clone, Copy)]
pub enum Opt {
    /// Displays the work running in the background to standard output.
    Development,
    /// Corrects TOML format errors automatically.
    BeginnersSupport,
    /// It limits functions and improves execution speed.
    Release,
}

/// Optimization.
struct OptState {
    /// Optimization.
    opt: Opt,
}
impl Default for OptState {
    fn default() -> Self {
        OptState {
            opt: Opt::BeginnersSupport,
        }
    }
}
impl OptState {
    fn get(&self) -> Opt {
        self.opt
    }
    fn set(&mut self, val: Opt) {
        self.opt = val;
    }
}
