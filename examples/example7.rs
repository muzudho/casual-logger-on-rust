//! There is no configuration file.  
//! 設定ファイルはありません。  

use casual_logger::{Extension, Level, Log, Opt};

fn main() {
    Log::set_file_name("hello");
    Log::set_file_ext(Extension::Log);
    Log::set_retention_days(31);
    Log::remove_old_logs();

    Log::set_level(Level::Notice);
    Log::set_timeout_secs(60);
    Log::set_opt(Opt::Release);

    Log::notice("Hello, world!!");

    Log::flush();
}
