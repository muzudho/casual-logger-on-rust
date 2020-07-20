//! You can copy and paste and use immediately.  
//! コピー＆ペーストしてすぐに使用できます。  

use casual_logger::{Level, Log, Table};

fn main() {
    Log::set_file_name_important("lesson1"); // For app use.
    Log::set_file_name("mischief1"); // For library use.
    Log::set_retention_days(2);
    Log::set_level(Level::Info);
    Log::remove_old_logs();

    Log::info_t(
        "Result",
        Table::default()
            .uint("Age", 200018)
            .str("Condition", "It's ok.")
            .bool("Lung breathing", true)
            .char("Rank", 'A')
            .str("Area", "Rever side")
            .str("Weather", "Rain")
            .int("Elevation", -40)
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
