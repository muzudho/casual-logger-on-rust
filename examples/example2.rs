//! You can copy and paste and use immediately.

use casual_logger::{Level, Log, Table};

fn main() {
    Log::set_file_name_important("lesson1");
    Log::set_file_name("mischief1");
    Log::set_retention_days(2);
    Log::set_level(Level::Info);
    Log::remove_old_logs();

    Log::info_t(
        "Result",
        Table::default()
            .str("Rank", "A")
            .str("Area", "Mountain")
            .str("Weather", "Rain")
            // Do not validate value. Unsafe.
            .literal("Point", "[ 800, 300, 500 ]")
            .str(
                "Message",
                "Hell, world!!
こんにちわ、世界！！",
            ),
    );

    Log::flush();
}
