//! TOML tables are typed maps.  
//! TOMLのテーブルは型付きのマップだ。  

use casual_logger::{Log, Table};

fn main() {
    Log::set_file_name("today-s-plan");
    Log::remove_old_logs();

    // Just add'_t'.
    // '_t' を付けただけ。
    Log::info_t(
        "ShoppingToday", // A-Z, a-z, 0-9, _, -.
        Table::default()
            // Japanese YEN.
            // 日本円。
            .int("FluorescentLight", -7_000)
            .int("VacuumCleaner", -53_000)
            // '.literal()' is no validate. carefully.
            // 構文チェックされません。慎重に。
            .literal(
                "VacuumCleanerPricesAtOtherStores",
                "[ -63_000, -4_000, -10_000 ]",
            )
            .int("Rent", -40_000)
            .uint("Salary", 190_000)
            .str(
                "Remark",
                "Buy shelves in the near month.
Replace the washing machine after a few years.
近い月に棚。
数年後に洗濯機買い替え。",
            ),
    );

    Log::flush();
}
