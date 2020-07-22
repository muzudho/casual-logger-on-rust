//! Used for editing and locking files.  
//! ファイルの編集とロックに使用されます。  
use chrono::{Date, Local};
use std::fs::File;

/// Used for editing and locking files.  
/// ファイルの編集とロックに使用されます。  
pub struct LogFile {
    /// Used for file name and deletion. Year, Month, Day.  
    /// ファイル名と削除に使用されます。年月日。  
    pub start_date: Date<Local>,
    /// Used for editing and locking files.  
    /// ファイルの編集とロックに使用されます。  
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
