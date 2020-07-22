//! Tables are easier to see if they are not nested..  
//! テーブルは入れ子にしない方が見やすいです。  

use casual_logger::{ArrayOfTable, Log, Table};

fn main() {
    Log::set_file_name("complex-toml");
    Log::remove_old_logs();

    // WIP. The top level does not support array of table.
    // 作業中。トップレベルはテーブルの配列に対応していません。
    Log::info_t(
        "ImInTrouble",
        // It's just a table.
        // ただのテーブルです。
        Table::default()
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
            )
            .sub_aot(
                "IHaveToCleanMyRoom",
                ArrayOfTable::default()
                    .table(Table::default().str("Name", "Kitchen").bool("Clean", false))
                    .table(Table::default().str("Name", "Bath").bool("Wash", false))
                    .table(Table::default().str("Name", "Toilet").bool("Brush", false)),
            ),
    );

    Log::flush();
}
