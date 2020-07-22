//! For test.
//! テスト用。

use casual_logger::Log;

fn main() {
    Log::set_file_name("test-fatal");
    Log::remove_old_logs();

    panic!(Log::fatal("Critical message!"));
}
