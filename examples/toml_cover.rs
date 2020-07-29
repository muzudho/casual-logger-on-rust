//! Toml cover check.
//! [TOML v1.0.0-rc.1](https://toml.io/en/v1.0.0-rc.1)
//!
//! Run: `cargo run --example toml_cover`.

use casual_logger::{ArrayOfTable, Log, Table};

fn main() {
    Log::set_file_name("toml-cover");
    Log::set_retention_days(-1);
    Log::remove_old_logs();

    // Same key test.
    Log::infoln_t(
        "Same key test.",
        Table::default()
            .bool("Bool", false)
            .bool("Bool", true)
            .char("Char", 'a')
            .char("Char", 'b')
            .float("Float", 1.0)
            .float("Float", 2.0)
            .int("Int", 1)
            .int("Int", 2)
            .isize("Isize", 1)
            .isize("Isize", 2)
            .literal("Literal", "1")
            .literal("Literal", "2")
            .str("Str", "1")
            .str("Str", "2")
            .sub_t("Sub_t", &Table::default())
            .sub_t("Sub_t", &Table::default())
            .sub_aot("Sub_aot", &ArrayOfTable::default())
            .sub_aot("Sub_aot", &ArrayOfTable::default())
            .uint("Uint", 1)
            .uint("Uint", 2)
            .usize("Usize", 1)
            .usize("Usize", 2),
    );

    // Primitive type conversion example.
    // プリミティブ型変換の例。
    let i8_ = 1_i8;
    let i16_ = 1_i16;
    let i32_ = 1_i32;
    let i64_ = 1_i64;
    let i128_ = 1_i128;
    let isize_ = 1_isize;
    let u8_ = 1_u8;
    let u16_ = 1_u16;
    let u32_ = 1_u32;
    let u64_ = 1_u64;
    let u128_ = 1_u128;
    let usize_ = 1_usize;
    Log::infoln_t(
        "Primitive type conversion example.",
        Table::default()
            .int("i8", i8_.into())
            .int("i16", i16_.into())
            .int("i32", i32_.into())
            .int("i64", i64_.into())
            .int("i128", i128_)
            .isize("isize", isize_)
            .uint("u8", u8_.into())
            .uint("u16", u16_.into())
            .uint("u32", u32_.into())
            .uint("u64", u64_.into())
            .uint("u128", u128_)
            .usize("usize", usize_),
    );

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
