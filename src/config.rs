use crate::log_file::LogFile;
use crate::Level;
use crate::Opt;
use crate::OPT_STATE;
use crate::SEQ;
use chrono::{Date, Duration, Local, TimeZone};
use regex::Regex;
use std::fs;
use std::fs::{File, OpenOptions};
use std::ops::Add;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    /// Logger grobal variable.
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::default());
}

/// Configuration.  
///
/// Example of Log file name:
/// ログ・ファイル名の例:
///
///      'tic-tac-toe-2020-07-11.log.toml'
///       1----------           3--------
///                  2----------
///
///       1 Prefix              3 Extention
///         接頭辞                拡張子
///                  2 StartDate
///                    開始日
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
    /// '.log.toml' or '.log'.
    pub file_extension: String,
    /// The level cannot be changed later.  
    /// レベルは後で変更できません。  
    pub level_important: bool,
    /// Activation.
    pub level: Level,
    /// The file retention days cannot be changed later.  
    /// ファイル保持日数は後で変更できません。  
    pub retention_days_important: bool,
    /// File retention days. Delete the file after day from StartDate.
    pub retention_days: i64,
    /// The timeout seconds cannot be changed later.  
    /// タイムアウト秒は後で変更できません。  
    pub timeout_secs_important: bool,
    /// Timeout seconds.
    pub timeout_secs: u64,
    /// Controll file.
    log_file: Option<LogFile>,
}
impl Default for Logger {
    fn default() -> Self {
        Logger {
            file_name_important: false,
            file_prefix: "default".to_string(),
            file_ext_important: false,
            file_extension: ".log.toml".to_string(),
            level_important: false,
            level: Level::Trace,
            retention_days_important: false,
            retention_days: 7,
            timeout_secs_important: false,
            timeout_secs: 30,
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

    pub fn get_optimization() -> Opt {
        if let Ok(opt_state) = OPT_STATE.lock() {
            opt_state.get()
        } else {
            // Error
            Opt::BeginnersSupport
        }
    }

    /// Check level.  
    pub fn enabled(&self, level: Level) -> bool {
        if level.number() <= self.level.number() {
            return true;
        }
        false
    }

    /// Create new file, or get exists file.  
    fn new_today_file(file_prefix: &str, file_extension: &str) -> (Date<Local>, File) {
        let start_date = Local::today();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            // Example: 'default-2020-07-11.log.toml'.
            .open(Path::new(&format!(
                "{}-{}{}",
                file_prefix,
                start_date.format("%Y-%m-%d"),
                file_extension
            )))
            .unwrap();
        (start_date, file)
    }
    /// For log rotation.
    /// Please use the casual_logger::Log::remove_old_logs() method.
    pub fn remove_old_logs(&self) -> usize {
        // Removed files count.
        let mut count = 0;
        // Example:
        //      all = './tic-tac-toe-2020-07-11.log.toml'
        //      prefix = "tic-tac-toe"
        //      now = "-2020-07-11"
        //      extension = ".log.toml" or ".log"
        let re = if let Ok(x) = Regex::new(&format!(
            "./{}-{}{}",
            self.file_prefix, r"(\d{4})-(\d{2})-(\d{2})", self.file_extension
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
                Logger::new_today_file(&self.file_prefix, &self.file_extension);
            self.log_file = Some(LogFile::new(start_date, file));
        }

        // Return file handle.
        &self.log_file.as_ref().unwrap().file
    }
}
