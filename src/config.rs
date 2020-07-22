use crate::log_file::LogFile;
use crate::stringifier::Stringifier;
use crate::table::ArrayOfTable;
use crate::table::InternalTable;
use crate::Level;
use crate::Opt;
use crate::OPT_STATE;
use crate::SEQ;
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
use std::sync::Mutex;
use std::thread;

lazy_static! {
    /// Logger grobal variable.
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::default());
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
    /// The file name cannot be changed later.  
    /// ファイル名は後で変更できません。  
    pub file_name_important: bool,
    /// For example, the short name of your application.
    pub file_prefix: String,
    /// The file suffix, extension cannot be changed later.  
    /// 接尾辞、拡張子は後で変更できません。  
    pub file_ext_important: bool,
    /// For example, '.log'. To be safe, include a word that clearly states that you can delete the file.
    pub file_suffix: String,
    /// If you don't like the .toml extension, leave the suffix empty and the .log extension.
    pub file_extension: String,
    /// The level cannot be changed later.  
    /// レベルは後で変更できません。  
    pub level_important: bool,
    /// Activation.
    #[deprecated(
        since = "0.3.7",
        note = "Please use the casual_logger::Log::set_level() method instead"
    )]
    pub level: Level,
    /// The file retention days cannot be changed later.  
    /// ファイル保持日数は後で変更できません。  
    pub retention_days_important: bool,
    /// File retention days. Delete the file after day from StartDate.
    #[deprecated(
        since = "0.3.8",
        note = "Please use the casual_logger::Log::set_retention_days() method instead"
    )]
    pub retention_days: i64,
    /// The timeout seconds cannot be changed later.  
    /// タイムアウト秒は後で変更できません。  
    pub timeout_secs_important: bool,
    /// Timeout seconds.
    #[deprecated(
        since = "0.3.9",
        note = "Please use the casual_logger::Log::set_timeout_secs() method instead"
    )]
    pub timeout_secs: u64,
    /// Timeout seconds when fatal.
    #[deprecated(
        since = "0.3.2",
        note = "Please use the casual_logger::Log::set_timeout_secs() method instead"
    )]
    pub fatal_timeout_secs: u64,
    /// Set to true to allow Casual_logger to output information to stdout and stderr.
    #[deprecated(
        since = "0.3.10",
        note = "Please use the casual_logger::Log::set_opt(Opt::Development) method instead"
    )]
    pub development: bool,
    /// Controll file.
    log_file: Option<LogFile>,
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
            timeout_secs_important: false,
            timeout_secs: 30,
            fatal_timeout_secs: 30,
            development: false,
            log_file: None,
        }
    }
}
impl Logger {
    /// Automatic sequential number on thread.
    pub fn create_seq() -> u128 {
        SEQ.with(move |seq| {
            let old = *seq.borrow();
            *seq.borrow_mut() += 1;
            old
        })
    }
    pub fn get_timeout_sec(logger: &Logger) -> u64 {
        if logger.timeout_secs != 30 {
            logger.timeout_secs
        } else if logger.fatal_timeout_secs != 30 {
            logger.fatal_timeout_secs
        } else {
            logger.timeout_secs
        }
    }

    pub fn get_optimization(logger: &Logger) -> Opt {
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
    pub fn current_file(&mut self) -> &File {
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