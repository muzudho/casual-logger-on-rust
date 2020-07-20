//! Test important.

use casual_logger::{Extension, Level, Log, Opt};

fn main() {
    // File name.
    Log::set_file_name_important("test-important");
    Log::set_file_name("mischief1");
    Log::debug(&format!("file_name=|{}|", Log::get_file_name().unwrap()));

    // File extension.
    Log::set_file_ext_important(Extension::Log);
    Log::set_file_ext(Extension::LogToml);
    Log::debug(&format!("file_ext=|{}|", Log::get_file_ext_str().unwrap()));

    // Level.
    Log::set_level_important(Level::Debug);
    Log::set_level(Level::Error);
    Log::debug(&format!("level=|{}|", Log::get_level().unwrap()));

    // File retention days.
    Log::set_retention_days_important(3);
    Log::set_retention_days(17);
    Log::debug(&format!(
        "retention_days=|{}|",
        Log::get_retention_days().unwrap()
    ));

    // Timeout seconds.
    Log::set_timeout_secs_important(45);
    Log::set_timeout_secs(70);
    Log::debug(&format!(
        "timeout_secs=|{}|",
        Log::get_timeout_secs().unwrap()
    ));

    // TODO Optimize.
    Log::set_opt_important(Opt::Release);
    Log::set_opt(Opt::Development);
    Log::debug(&format!("opt=|{:?}|", Log::get_opt().unwrap()));

    // Finish.
    Log::remove_old_logs();

    Log::flush();
}
