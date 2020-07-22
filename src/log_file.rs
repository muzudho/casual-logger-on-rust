use chrono::{Date, Local};
use std::fs::File;

/// Used for editing and locking files.
pub struct LogFile {
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
