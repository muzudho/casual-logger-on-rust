//! For test.
//! テスト用。

use casual_logger::{Log, Table};

fn main() {
    Log::set_file_name("test-fatal");
    Log::remove_old_logs();

    // panic!(Log::fatal("Critical message!"));

    panic!(Log::fatal_t(
        "Critical message!",
        Table::default().int("Apple", 1)
    ));
}
