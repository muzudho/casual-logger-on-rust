//! Toml cover check.
//! [TOML v1.0.0-rc.1](https://toml.io/en/v1.0.0-rc.1)
//!
//! Run: `cargo run --example toml_cover`.

use casual_logger::{ArrayOfTable, Log, Table};

fn main() {
    Log::set_file_name("toml-cover");
    Log::set_retention_days(-1);
    Log::remove_old_logs();

    // String.
    Log::infoln_t(
        "String test",
        Table::default()
            .int("n01_Distance", -128)
            .uint("n02_Age", 200018)
            .float("n03_Weight", 45.5)
            .bool("n04_Tall", false)
            .char("n05_Initial", 'A')
            .str("n06_FilePathOnWindows", "C:\\User\\Muzudho")
            .str("n07_FilePathOnWindowsInDSQ", "''C:\\User\\Muzudho''")
            .str("n08_FilePathOnLinuxOS", "/etc/nginx/conf.d")
            .str("n09_FilePathOnLinuxOSInDSQ", "''/etc/nginx/conf.d''")
            .str("n10_SinglePlain", "末尾改行なし")
            .str("n11_SinglePlainLn", "末尾改行あり\r\n")
            .str(
                "n12_MultiLinePlain",
                "１行目
２行目
３行目",
            )
            .str("n13_SingleLineContainsSingleQuotation", "'１行目'")
            .str(
                "n14_SingleLineContainsSingleQuotationLn",
                "'１行目'
",
            )
            .str(
                "n15_MultiLineContainsSingleQuotation",
                "１行目
'２行目'
３行目",
            )
            .str(
                "n16_MultiLineContainsSingleQuotationLn",
                "１行目
'２行目'
３行目
",
            )
            .str(
                "n17_MultiLineContainsTripleSingleQuotation",
                "１行目
'''２行目'''
３行目",
            )
            .str(
                "n18_MultiLineContainsTripleSingleQuotationLn",
                "１行目
'''２行目'''
３行目
",
            )
            .str(
                "n19_MultiLinePlainLn",
                "１行目
２行目
３行目
",
            )
            .str("n20_DoubleQuotation", "\"quoted\"")
            .str("n21_CarriageReturnLineFeed", "\r\n")
            .str("n22_LineFeed", "\n")
            .str(" n23_Middle ", "Ignored space at left and right.")
            .str(
                "n24_Apple . Banana",
                "Space quoted dot. I don't recommend it, but it's okay.",
            )
            .str(
                "n25_Dotted.Key",
                "Dotted key unsupported. Please use sub-table.",
            ),
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

    // Table in Array-of-table test. (Sub only)
    Log::trace_t(
        "Table in Array-of-table test.",
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

    // Sub-table in Array-of-table test. (Sub only)
    Log::trace_t(
        "Table in Array-of-table test.",
        Table::default()
            .sub_aot(
                "z1",
                &ArrayOfTable::default()
                    .table(
                        &Table::default().sub_t(
                            "a",
                            Table::default()
                                .int("Apple", 1)
                                .int("Alice", 18)
                                .int("Alpha", 100),
                        ),
                    )
                    .table(
                        &Table::default().sub_t(
                            "b",
                            Table::default()
                                .int("Banana", 2)
                                .int("Bob", 19)
                                .int("Beta", 200),
                        ),
                    )
                    .table(
                        &Table::default().sub_t(
                            "c",
                            Table::default()
                                .int("Cherry", 3)
                                .int("Charley", 20)
                                .int("Gamma", 300),
                        ),
                    ),
            )
            .sub_aot(
                "z2",
                &ArrayOfTable::default()
                    .table(
                        &Table::default().sub_t(
                            "a",
                            Table::default()
                                .int("The Apple", -1)
                                .int("The Alice", -18)
                                .int("The Alpha", -100),
                        ),
                    )
                    .table(
                        &Table::default().sub_t(
                            "b",
                            Table::default()
                                .int("The Banana", -2)
                                .int("The Bob", -19)
                                .int("The Beta", -200),
                        ),
                    )
                    .table(
                        &Table::default().sub_t(
                            "c",
                            Table::default()
                                .int("The Cherry", -3)
                                .int("The Charley", -20)
                                .int("The Gamma", -300),
                        ),
                    ),
            ),
    );

    Log::flush();
}
