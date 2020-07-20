//! Test important.

use casual_logger::{Extension, Log};

fn main() {
    // File name.
    Log::set_file_name_important("test-important");
    Log::set_file_name("mischief1");
    Log::debug(&format!("file_name=|{}|", Log::get_file_name().unwrap()));

    // File extension.
    Log::set_file_ext_important(Extension::Log);
    Log::set_file_ext(Extension::LogToml);
    Log::debug(&format!("file_ext=|{}|", Log::get_file_ext_str().unwrap()));

    // Finish.
    Log::remove_old_logs();
    Log::flush();
}
