//! Toml cover check.
//! [TOML v1.0.0-rc.1](https://toml.io/en/v1.0.0-rc.1)

use casual_logger::{Log, Opt, Table};

fn main() {
    Log::set_file_name("toml-cover");
    Log::set_retention_days(2);
    Log::set_opt(Opt::Development);
    Log::remove_old_logs();

    // String.
    Log::infoln_t(
        "String test",
        Table::default()
            .int("Distance", -128)
            .uint("Age", 200018)
            .float("Weight", 45.5)
            .bool("Tall", false)
            .char("Initial", 'A')
            .str("FilePathOnWindows", "C:\\User\\Muzudho")
            .str("FilePathOnWindowsInDSQ", "''C:\\User\\Muzudho''")
            .str("FilePathOnLinuxOS", "/etc/nginx/conf.d")
            .str("FilePathOnLinuxOSInDSQ", "''/etc/nginx/conf.d''")
            .str("SinglePlain", "末尾改行なし")
            .str("SinglePlainLn", "末尾改行あり\r\n")
            .str(
                "MultiLinePlain",
                "１行目
２行目
３行目",
            )
            .str(
                "MultiLineContainsTripleSingleQuotation",
                "１行目
'''２行目'''
３行目",
            )
            .str(
                "MultiLineContainsTripleSingleQuotationLn",
                "１行目
'''２行目'''
３行目
",
            )
            .str(
                "MultiLinePlainLn",
                "１行目
２行目
３行目
",
            )
            .str("DoubleQuotation", "\"quoted\"")
            .str("NewLine", "\r\n")
            .str(" Middle ", "Ignored space at left and right.")
            .str(
                "Apple . Banana",
                "Space quoted dot. I don't recommend it, but it's okay.",
            )
            .str("Dotted.Key", "Correct."),
    );

    // Illegal keys. Auto correct check.
    Log::info_t(
        "Illegal key test",
        Table::default()
            .str("キー", "Japanese.")
            .str("House key", "Space.")
            .str(
                "Bicycle
key",
                "Contains newline.",
            ),
    );

    // TODO WIP.
    Log::info_t(
        "SubTableTest",
        Table::default()
            .str("name", "apple")
            // Sub table.
            .subt(
                "physical",
                Table::default().str("color", "red").str("shape", "round"),
            ),
    );
    /*
    // TODO WIP. Delete.
    Log::trace_s(
        &Separation::default()
            .table("Alice", &Table::default().int("Apple", 1))
            .table("Bob", &Table::default().int("Banana", 2))
            .table("Charley", &Table::default().int("Cherry", 3)),
    );
    */

    Log::flush();
}
