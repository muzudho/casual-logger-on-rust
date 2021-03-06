//! Test important.

use casual_logger::{
    Extension, Level, Log, Opt, DEFAULT_LOG_LEVEL, DEFAULT_OPTIMIZATION, DEFAULT_RETENTION_DAYS,
    DEFAULT_TIMEOUT_SECS,
};

fn main() {
    // File name.
    Log::set_file_name_important("test-important");
    Log::set_file_name("mischief1");
    Log::debug(&format!(
        "file_name=|{}|",
        Log::get_file_name().unwrap_or_else(|err| err)
    ));

    // File extension.
    Log::set_file_ext_important(Extension::Log);
    Log::set_file_ext(Extension::LogToml);
    Log::debug(&format!(
        "file_ext=|{}|",
        Log::get_file_ext_str().unwrap_or_else(|err| err)
    ));

    // Level.
    Log::set_level_important(Level::Debug);
    Log::set_level(Level::Error);
    Log::debug(&format!(
        "level=|{}|",
        Log::get_level().unwrap_or_else(|_| DEFAULT_LOG_LEVEL)
    ));

    // File retention days.
    Log::set_retention_days_important(3);
    Log::set_retention_days(17);
    Log::debug(&format!(
        "retention_days=|{}|",
        Log::get_retention_days().unwrap_or_else(|_| DEFAULT_RETENTION_DAYS)
    ));

    // Timeout seconds.
    Log::set_timeout_secs_important(45);
    Log::set_timeout_secs(70);
    Log::debug(&format!(
        "timeout_secs=|{}|",
        Log::get_timeout_secs().unwrap_or_else(|_| DEFAULT_TIMEOUT_SECS)
    ));

    // Optimize.
    Log::set_opt_important(Opt::Release);
    Log::set_opt(Opt::Development);
    Log::debug(&format!(
        "opt=|{:?}|",
        Log::get_opt().unwrap_or_else(|_| DEFAULT_OPTIMIZATION)
    ));

    // Finish.
    Log::remove_old_logs();

    Log::flush();
}
