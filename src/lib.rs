//! What a bother. I want to logging it **without setting it**. Not a product.  
//! なんて面倒だ。 **設定せず** にロギングしたい。 製品じゃないし。  

// Publish:
//
// (1) `cargo test`
// (2a1) `cargo run --example example1`
// (2a2) `cargo run --example example2`
// (2a3) `cargo run --example example3`
// (2a4) `cargo run --example example4`
// (2a5) `cargo run --example example5`
// (2c) `cargo run --example fatal`
// (2d) `cargo run --example important`
// (2e) `cargo run --example overall`
// (2f) `cargo run --example performance`
// (2g) `cargo run --example toml_cover`
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

mod config;
mod log_file;
mod stringifier;
mod table;

use crate::config::Logger;
use crate::config::LOGGER;
use crate::stringifier::Stringifier;
use crate::table::InternalTable;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt;
use std::io::{BufWriter, Write};
use std::sync::Mutex;
use std::thread;
// use sys_info::mem_info;

// For multi-platform. Windows, or not.
#[cfg(windows)]
const NEW_LINE: &'static str = "\r\n";
// #[cfg(windows)]
// const NEW_LINE_ESCAPED_CHARS: &'static [char; 4] = &['\\', 'r', '\\', 'n'];
#[cfg(not(windows))]
const NEW_LINE: &'static str = "\n";
// #[cfg(not(windows))]
// const NEW_LINE_ESCAPED_CHARS: &'static [char; 2] = &['\\', 'n'];

/// The log level is `Level::Trace` by default.  
/// ログレベルはデフォルトで `Level::Trace` です。  
pub const DEFAULT_LOG_LEVEL: Level = Level::Trace;

/// The file retention period is 7 days by default.  
/// ファイルの保存期間は、デフォルトで7日です。  
pub const DEFAULT_RETENTION_DAYS: u32 = 7;

/// The default timeout is 30 seconds.  
/// Used for Log::flush() wait time.  
/// タイムアウトのデフォルトは30秒です。  
/// Log::flush() の待機時間に使われます。  
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// The optimization is `Opt::BeginnersSupport` by default.  
/// 最適化はデフォルトで `Opt::BeginnersSupport` です。  
pub const DEFAULT_OPTIMIZATION: Opt = Opt::BeginnersSupport;

/// The higher this level, the more detailed the log.  
/// このレベルが高いほど、ログはより詳細になります。  
///
/// |<-- Low Level ------------------------- High level -->|  
/// |<-- High priority ------------------- Low priority -->|  
/// | Fatal < Error < Warn < Notice < Info < Debug < Trace |  
#[derive(Clone, Copy, Debug)]
pub enum Level {
    /// If the program cannot continue.  
    Fatal,
    /// I didn't get the expected result, so I'll continue with the other method.  
    Error,
    /// It will be abnormal soon, but there is no problem and you can ignore it.  
    /// For example:  
    /// * He reported that it took longer to access than expected.
    /// * Report that capacity is approaching the limit.
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
    /// Table buffer.
    static ref QUEUE_T: Mutex<VecDeque<InternalTable>> = Mutex::new(VecDeque::<InternalTable>::new());
    static ref QUEUE_F: Mutex<VecDeque<InternalTable>> = Mutex::new(VecDeque::<InternalTable>::new());
    static ref RESERVE_TARGET: Mutex<ReserveTarget> = Mutex::new(ReserveTarget::default());
    static ref SIGNAL_CAN_FLUSH: Mutex<SignalCanFlush> = Mutex::new(SignalCanFlush::default());
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

/// TODO Array of Table.  
/// テーブルの配列。  
#[derive(Clone)]
pub struct ArrayOfTable {
    tables: Vec<Table>,
}

/// TOML table included in the log file. The form is not validated.  
/// However, if "Log::set_opt(Opt::BeginnersSupport)" is set,  
/// it will intervene with some automatic correction.  
/// ログファイルに含まれるTOMLテーブル。書式の妥当性検証はしません。  
/// ただし、"Log::set_opt(Opt::BeginnersSupport)" が設定されている場合は、  
/// ある程度の自動修正で介入します。  
#[derive(Clone)]
pub struct Table {
    /// The base name is added when writing the log.  
    /// ログを書くときにベース名が付きます。  
    base_name: String,
    level: Level,
    message: String,
    message_trailing_newline: bool,
    sorted_map: Option<BTreeMap<String, String>>,
    sub_tables: Option<BTreeMap<String, InternalTable>>,
}
impl Table {
    /// Create a new table.  
    /// 新しいテーブルを作成します。  
    ///
    /// # Arguments
    ///
    /// * `level` - Log level.  
    ///             ログ・レベル。  
    /// * `trailing_newline` - Trailing newline.  
    ///                         改行の有無。  
    fn new(level: Level, message: &str, trailing_newline: bool, base_name: &str) -> Self {
        Table {
            base_name: base_name.to_string(),
            level: level,
            message: message.to_string(),
            message_trailing_newline: trailing_newline,
            sorted_map: None,
            sub_tables: None,
        }
    }

    fn get_sorted_map<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut BTreeMap<String, String>),
    {
        if let None = self.sorted_map {
            self.sorted_map = Some(BTreeMap::new());
        }

        if let Some(sorted_map) = &mut self.sorted_map {
            callback(sorted_map);
        }
    }

    fn get_sub_tables<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut BTreeMap<String, InternalTable>),
    {
        if let None = self.sub_tables {
            self.sub_tables = Some(BTreeMap::new());
        }

        if let Some(sub_tables) = &mut self.sub_tables {
            callback(sub_tables);
        }
    }
}

/// Easy to use logging.  
/// 使いやすいロギング。  
pub struct Log {}
impl Log {
    /// Set the log file name prefix.  
    /// ログ・ファイル名接頭辞を設定します。  
    ///
    /// Example of Log file name:  
    /// ログ・ファイル名の例:  
    ///
    ///       tic-tac-toe-2020-07-11.log.toml  
    ///       1----------           3--------  
    ///                  2----------  
    ///
    ///       1 Prefix              3 Extention  
    ///         接頭辞                拡張子  
    ///                  2 StartDate  
    ///                    開始日  
    ///
    /// **StartDate** is basically today.  
    /// If the rotation fails, it is the start date.
    ///
    /// **`.log`** to be safe, include a word that  
    /// clearly states that you can delete the file.  
    ///
    /// See also: 'Log::set_file_name_important()'.  
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

    /// For application use. No for library use.  
    /// アプリケーションでの使用向け。 ライブラリ向けではありません。  
    ///
    /// Set the log file name prefix. The file name cannot be changed later.  
    /// ログ・ファイル名接頭辞を設定します。ファイル名は後で変更できません。  
    ///
    /// Example of Log file name:  
    /// ログ・ファイル名の例:  
    ///
    ///       tic-tac-toe-2020-07-11.log.toml  
    ///       1----------           3--------  
    ///                  2----------  
    ///
    ///       1 Prefix              3 Extention  
    ///         接頭辞                拡張子  
    ///                  2 StartDate  
    ///                    開始日  
    ///
    /// **StartDate** is basically today.  
    /// If the rotation fails, it is the start date.  
    ///
    /// **`.log`** to be safe, include a word that  
    /// clearly states that you can delete the file.  
    ///
    /// See also: 'Log::set_file_name()'.  
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
                        logger.file_extension = ".log.toml".to_string();
                    }
                    Extension::Log => {
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
            Ok(logger) => Ok(logger.file_extension.to_string()),
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
                logger.retention_days = days;
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
            if !logger.timeout_secs_important {
                logger.timeout_secs = secs;
            }
        }
    }

    /// The timeout seconds cannot be changed later.  
    /// タイムアウト秒は後で変更できません。  
    ///
    /// See also: `Log::set_timeout_secs()`.  
    pub fn set_timeout_secs_important(secs: u64) {
        Log::set_timeout_secs(secs);
        if let Ok(mut logger) = LOGGER.lock() {
            logger.timeout_secs_important = true;
        }
    }

    /// The timeout seconds.  
    /// タイムアウト秒。  
    pub fn get_timeout_secs() -> Result<u64, String> {
        match LOGGER.lock() {
            Ok(logger) => Ok(logger.timeout_secs),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Optimization.
    pub fn set_opt(optimization: Opt) {
        if let Ok(mut opt_state) = OPT_STATE.lock() {
            if !opt_state.opt_important {
                opt_state.set(optimization);
            }
        }
    }

    /// The optimization cannot be changed later.  
    /// 最適化は後で変更できません。  
    ///
    /// See also: `Log::set_opt()`.  
    pub fn set_opt_important(optimization: Opt) {
        Log::set_opt(optimization);
        if let Ok(mut opt_state) = OPT_STATE.lock() {
            opt_state.opt_important = true;
        }
    }

    /// Optimization.  
    /// 最適化。  
    pub fn get_opt() -> Result<Opt, String> {
        match OPT_STATE.lock() {
            Ok(opt_state) => Ok(opt_state.opt),
            Err(e) => Err(e.to_string()),
        }
    }

    /// # Returns
    ///
    /// Number of deleted log files.
    pub fn remove_old_logs() -> usize {
        let remove_num = if let Ok(logger) = LOGGER.lock() {
            // Do not call 'Log::xxxxx()' in this code block.

            let remove_num = logger.remove_old_logs();

            match Logger::get_optimization() {
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
            (logger.timeout_secs, Logger::get_optimization())
        } else {
            // Error
            (0, Opt::BeginnersSupport)
        };

        Log::wait_for_logging_to_complete(timeout_secs, |secs, message| {
            // Do not call 'Log::xxxxx()' in this code block.
            match opt {
                Opt::Development => {
                    println!("casual_logger: {} sec(s). {}", secs, message,);
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
    fn wait_for_logging_to_complete<F>(timeout_secs: u64, count_down: F)
    where
        F: Fn(u64, String),
    {
        let mut elapsed_milli_secs = 0;

        // Wait a moment for the thread just created to write.
        // 今作成されたスレッドが書き込むのを少し待ちます。
        thread::sleep(std::time::Duration::from_millis(20));
        elapsed_milli_secs += 20;

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
    pub fn trace(message: &str) {
        if Log::enabled(Level::Trace) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Trace,
                message,
                false,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Trace level. There is a trailing newline.
    pub fn traceln(message: &str) {
        if Log::enabled(Level::Trace) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Trace,
                message,
                true,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Trace level. No trailing newline. Use table.
    pub fn trace_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Trace) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Trace;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Trace level. There is a trailing newline. Use table.
    pub fn traceln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Trace) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Trace;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Debug level. No trailing newline.
    pub fn debug(message: &str) {
        if Log::enabled(Level::Debug) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Debug,
                message,
                false,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Debug level. There is a trailing newline.
    pub fn debugln(message: &str) {
        if Log::enabled(Level::Debug) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Debug,
                message,
                true,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Debug level. No trailing newline. Use table.
    pub fn debug_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Debug) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Debug;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Debug level. There is a trailing newline. Use table.
    pub fn debugln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Debug) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Debug;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Info level. No trailing newline.
    pub fn info(message: &str) {
        if Log::enabled(Level::Info) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Info,
                message,
                false,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Info level. There is a trailing newline.
    pub fn infoln(message: &str) {
        if Log::enabled(Level::Info) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Info,
                message,
                true,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Info level. No trailing newline. Use table.
    pub fn info_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Info) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Info;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Info level. There is a trailing newline. Use table.
    pub fn infoln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Info) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Info;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(&InternalTable::from_table(table));
        }
    }
    /// Notice level. No trailing newline.
    pub fn notice(message: &str) {
        if Log::enabled(Level::Notice) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Notice,
                message,
                false,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Notice level. There is a trailing newline.
    pub fn noticeln(message: &str) {
        if Log::enabled(Level::Notice) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Notice,
                message,
                true,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }
    /// Notice level. No trailing newline. Use table.
    pub fn notice_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Notice) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Notice;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Notice level. There is a trailing newline. Use table.
    pub fn noticeln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Notice) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Notice;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Warning level. No trailing newline.
    pub fn warn(message: &str) {
        if Log::enabled(Level::Warn) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Warn,
                message,
                false,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Warning level. There is a trailing newline.
    pub fn warnln(message: &str) {
        if Log::enabled(Level::Warn) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Warn,
                message,
                true,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Warning level. No trailing newline. Use table.
    pub fn warn_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Warn) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Warn;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Warning level. There is a trailing newline. Use table.
    pub fn warnln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Warn) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Warn;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Error level. No trailing newline.
    pub fn error(message: &str) {
        if Log::enabled(Level::Error) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Error,
                message,
                false,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Error level. There is a trailing newline.
    pub fn errorln(message: &str) {
        if Log::enabled(Level::Error) {
            Log::reserve(&InternalTable::from_table(&Table::new(
                Level::Error,
                message,
                true,
                &Stringifier::create_identify_table_name(Logger::create_seq()),
            )));
        }
    }

    /// Error level. No trailing newline. Use table.
    pub fn error_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Error) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Error;
            table.message = message.to_string();
            table.message_trailing_newline = false;
            Log::reserve(&InternalTable::from_table(table));
        }
    }

    /// Error level. There is a trailing newline. Use table.
    pub fn errorln_t(message: &str, table: &mut Table) {
        if Log::enabled(Level::Error) {
            table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
            table.level = Level::Error;
            table.message = message.to_string();
            table.message_trailing_newline = true;
            Log::reserve(&InternalTable::from_table(table));
        }
    }
    /// Fatal level. No trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    pub fn fatal(message: &str) -> String {
        // Fatal runs at any level.
        Log::reserve(&InternalTable::from_table(&Table::new(
            Level::Fatal,
            message,
            false,
            &Stringifier::create_identify_table_name(Logger::create_seq()),
        )));
        // Wait for logging to complete or to timeout.
        Log::flush();
        message.to_string()
    }
    /// Fatal level. There is a trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    pub fn fatalln(message: &str) -> String {
        // Fatal runs at any level.
        Log::reserve(&InternalTable::from_table(&Table::new(
            Level::Fatal,
            message,
            true,
            &Stringifier::create_identify_table_name(Logger::create_seq()),
        )));
        // Wait for logging to complete or to timeout.
        Log::flush();
        // Append trailing newline.
        format!("{}{}", message, NEW_LINE).to_string()
    }

    /// Fatal level. No trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    pub fn fatal_t(message: &str, table: &mut Table) -> String {
        // Fatal runs at any level.
        table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
        table.level = Level::Fatal;
        table.message = message.to_string();
        table.message_trailing_newline = false;
        Log::reserve(&InternalTable::from_table(table));
        // Wait for logging to complete or to timeout.
        Log::flush();
        message.to_string()
    }
    /// Fatal level. There is a trailing newline.
    /// Fatal is Panic! Can be used as the first argument of.
    pub fn fatalln_t(message: &str, table: &mut Table) -> String {
        // Fatal runs at any level.
        table.base_name = Stringifier::create_identify_table_name(Logger::create_seq());
        table.level = Level::Fatal;
        table.message = message.to_string();
        table.message_trailing_newline = true;
        Log::reserve(&InternalTable::from_table(table));
        // Wait for logging to complete or to timeout.
        Log::flush();
        // Append trailing newline.
        format!("{}{}", message, NEW_LINE).to_string()
    }

    fn reserve(i_table: &InternalTable) {
        /*
        if let Ok(mut participating_threads_counter) = PARTICIPANTING_THREADS_COUNTER.lock() {
            participating_threads_counter.increase_thread_count();
        }
        */

        if let Ok(reseve_target) = RESERVE_TARGET.lock() {
            if reseve_target.is_t() {
                if let Ok(mut queue) = QUEUE_T.lock() {
                    queue.push_front(i_table.clone());
                }
            } else {
                if let Ok(mut queue) = QUEUE_F.lock() {
                    queue.push_front(i_table.clone());
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
                        str_buf.push_str(&internal_table.stringify());
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
                        str_buf.push_str(&internal_table.stringify());
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
}

/// The extension of the log file.  
/// ログファイルの拡張子です。  
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
/// 最適化。  
#[derive(Clone, Copy, Debug)]
pub enum Opt {
    /// Displays the work running in the background to standard output.  
    /// バックグラウンドで実行中の作業を標準出力に表示します。  
    Development,
    /// Corrects TOML format errors automatically.  
    /// TOML形式のエラーを自動的に修正します。  
    BeginnersSupport,
    /// It limits functions and improves execution speed.  
    /// 機能を制限し、実行速度を向上させます。  
    Release,
}

/// Optimization.
struct OptState {
    /// The optimization cannot be changed later.  
    /// 最適化は後で変更できません。  
    opt_important: bool,
    /// Optimization.  
    /// 最適化。  
    opt: Opt,
}
impl Default for OptState {
    fn default() -> Self {
        OptState {
            opt_important: false,
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
