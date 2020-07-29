//! Toml cover check.
//! [TOML v1.0.0-rc.1](https://toml.io/en/v1.0.0-rc.1)
//!
//! Run: `cargo run --example one_point`.

use casual_logger::{Log, Table};

fn main() {
    Log::set_file_name("one_point");
    Log::set_retention_days(-1);
    Log::remove_old_logs();

    // Test.
    Log::infoln_t(
        "One-point test",
        Table::default().str(
            "SingleLineContainsSingleQuotationLn",
            "'１行目'
",
        ),
    );

    Log::flush();
}
