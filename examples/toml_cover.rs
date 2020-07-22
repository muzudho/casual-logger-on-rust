//! Toml cover check.
//! [TOML v1.0.0-rc.1](https://toml.io/en/v1.0.0-rc.1)

use casual_logger::{ArrayOfTable, Log, Opt, Table};

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

    // Sub table test.
    Log::info_t(
        "Message 1.",
        Table::default()
            .int("a1", 1)
            .sub_t(
                "a2",
                Table::default()
                    .int("a2b1", 21)
                    .sub_t("a2b2", Table::default().int("a2b2c1", 221))
                    .sub_t("a2b3", Table::default().int("a2b3c1", 231)),
            )
            .sub_t(
                "a3",
                Table::default()
                    .int("a3b1", 31)
                    .sub_t("a3b2", Table::default().int("a3b2c1", 321)),
            ),
    );

    // Array of Table test. (Sub only)
    Log::trace_t(
        "TestArrayOfTable",
        Table::default()
            .sub_aot(
                "z1",
                &ArrayOfTable::default()
                    .table(
                        &Table::default()
                            .int("Apple", 1)
                            .int("Alice", 18)
                            .int("Alpha", 100),
                    )
                    .table(
                        &Table::default()
                            .int("Banana", 2)
                            .int("Bob", 19)
                            .int("Beta", 200),
                    )
                    .table(
                        &Table::default()
                            .int("Cherry", 3)
                            .int("Charley", 20)
                            .int("Gamma", 300),
                    ),
            )
            .sub_aot(
                "z2",
                &ArrayOfTable::default()
                    .table(
                        &Table::default()
                            .int("The Apple", -1)
                            .int("The Alice", -18)
                            .int("The Alpha", -100),
                    )
                    .table(
                        &Table::default()
                            .int("The Banana", -2)
                            .int("The Bob", -19)
                            .int("The Beta", -200),
                    )
                    .table(
                        &Table::default()
                            .int("The Cherry", -3)
                            .int("The Charley", -20)
                            .int("The Gamma", -300),
                    ),
            ),
    );

    Log::flush();
}
