//! Learn how to use TOML.  
//! TOMLの使い方を学びます。  

use casual_logger::{Log, Table};

fn main() {
    Log::set_file_name("today-s-plan");
    Log::remove_old_logs();

    // TOML say a table. So-called map.
    // Use table by '_t' suffix.
    // TOMLのテーブルは、いわゆるマップです。
    // '_t' を末尾に付けて、テーブルを使用します。
    Log::info_t(
        // Key is alphanumeric underscore hyphen.
        // A-Z, a-z, 0-9, _, -.
        // キーに使える文字は英数字下線ハイフンです。
        "ShoppingToday",
        Table::default()
            // Japanese YEN.
            // 日本円。
            .int("FluorescentLight", -7_000)
            .int("VacuumCleaner", -53_000)
            // Do not validate value. Unsafe.
            // 構文チェックされません。慎重に。
            .literal(
                "VacuumCleanerPricesAtOtherStores",
                "[ -63_000, -4_000, -10_000 ]",
            )
            .int("Rent", -40_000)
            .uint("Salary", 190_000)
            .char("Condition", 'A')
            .str(
                "Remark",
                "Buy shelves in the near month.
Replace the washing machine after a few years.
近い月に棚。
数年後に洗濯機買い替え。",
            )
            .float("ShelveDepth", 46.5)
            .bool("PaidRent", true)
            // It is easier to see if you do
            // not use a sub table.
            // サブテーブルを使用しない方が
            // 見やすいです。
            .sub_t(
                "RestFood",
                Table::default()
                    .int("FrozenRamen", 2)
                    .int("BottoleOfTea", 1)
                    .int("Kimchi", 1),
            ),
    );

    Log::flush();
}
