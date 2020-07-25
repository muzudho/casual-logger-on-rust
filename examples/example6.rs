//! Tables are easier to see if they are not nested.  
//! テーブルは入れ子にしない方が見やすいです。  

use casual_logger::{ArrayOfTable, Log, Table};

fn main() {
    Log::set_file_name("complex-toml");
    Log::remove_old_logs();

    // The top level does not support array of table.
    // Must be a table.
    // トップレベルはテーブルの配列に対応していません。
    // 必ずテーブルです。
    Log::info_t(
        // A message.
        // メッセージ。
        "I'm in trouble.",
        // It's just a table.
        // ただのテーブルです。
        Table::default()
            // Sub table.
            // サブテーブル。
            .sub_t(
                "RestFood",
                Table::default()
                    .int("FrozenRamen", 2)
                    .int("BottoleOfTea", 1)
                    .int("Kimchi", 1),
            )
            // Sub array of table.
            // テーブルのサブ配列です。
            .sub_aot(
                "IHaveToCleanMyRoom",
                ArrayOfTable::default()
                    .table(Table::default().str("Name", "Kitchen").bool("Clean", false))
                    .table(Table::default().str("Name", "Bath").bool("Wash", false))
                    .table(Table::default().str("Name", "Toilet").bool("Brush", false)),
            )
            // Sub array of sub table.
            // サブ・テーブルのサブ配列です。
            .sub_aot(
                "SubArrayOfSubTable",
                ArrayOfTable::default()
                    .table(Table::default().sub_t(
                        "SameName",
                        Table::default().str("Name", "Kitchen").bool("Clean", false),
                    ))
                    .table(Table::default().sub_t(
                        "SameName",
                        Table::default().str("Name", "Bath").bool("Wash", false),
                    ))
                    .table(Table::default().sub_t(
                        "SameName",
                        Table::default().str("Name", "Toilet").bool("Brush", false),
                    )),
            ),
    );

    Log::flush();
}
