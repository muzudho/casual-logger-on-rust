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

    Log::wait();
}
