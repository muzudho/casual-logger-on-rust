//! All features are described in one copy and paste.
//! すべての機能が1つのコピー＆ペーストで説明されています。

use casual_logger::{ArrayOfTable, Extension, Level, Log, Opt, Table};

fn main() {
    // Example of Log file name:
    // ログ・ファイル名の例:
    //
    // +
    // | tic-tac-toe-2020-07-11.log.toml
    // | 1----------           3--------
    // |            2----------
    // |
    // | 1 Prefix              3 Extention
    // |   接頭辞                拡張子
    // |            2 StartDate
    // |              開始日
    // +
    //
    // - StartDate is automatically added.
    //   開始日は自動で付きます。
    //
    // Set the file name prefix.
    // ファイル名に接頭辞を付けてください。
    Log::set_file_name("tic-tac-toe");
    // Methods with a trailing'_important'
    // can negate later changes.
    // 末尾に '_important' の付いたメソッドは、
    // 後の変更を無効にできます。
    // Log::set_file_name_important("tic-tac-toe");
    //
    // Log file extension:
    // 拡張子:
    //
    // '.log.toml' or '.log'.
    // '.log' for safety, include a word that
    // clearly states that you can delete the file.
    // If you don't like the .toml extension, change.
    // '.log.toml' か '.log' かを選べます。
    // 消してもよいファイルであることを明示するため、
    // ファイル名に '.log' は必ず含めます。
    Log::set_file_ext(Extension::LogToml);
    //
    // Logs with lower priority than this level will not
    // be written.
    //
    // |<-- Low Level --------------------- High level -->|
    // |<-- High priority --------------- Low priority -->|
    // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
    Log::set_level(Level::Trace);

    // Remove old log files. This is determined by the
    // StartDate in the filename.
    Log::set_retention_days(2);

    // Wait for seconds logging to complete.
    // By default it's set to 30 seconds,
    // so you probably don't need to set it.
    Log::set_timeout_secs(30);

    // Opt::Development
    // Displays the work running in the background to
    // standard output.
    //
    // Opt::BeginnersSupport
    // Corrects TOML format errors automatically.
    // Displays the auto correct to standard output.
    // Default. so you probably don't need to set it.
    //
    // Opt::Release
    // Disables beginner support to improve execution
    // speed.
    // Beginner support may be faster if there are
    // formatting errors.
    Log::set_opt(Opt::Development);

    // Remove old log files. This is determined by the
    // StartDate in the filename.
    Log::remove_old_logs();

    // Multi-line string.
    // The suffix "ln" adds a newline at the end.
    Log::info(
        "Hello, world!!
こんにちわ、世界！！",
    );

    // After explicitly checking the level.
    if Log::enabled(Level::Info) {
        let x = 100; // Time-consuming preparation, here.
        Log::infoln(&format!("x is {}.", x));
    }

    // The level is implicitly confirmed.
    Log::trace("( 1)TRACE");
    Log::traceln("( 2)trace-line");
    Log::debug("( 3)DEBUG");
    Log::debugln("( 4)debug-line");
    Log::info("( 5)INFO");
    Log::infoln("( 6)info-line");
    Log::notice("( 7)NOTICE");
    Log::noticeln("( 8)notice-line");
    Log::warn("( 9)WARN");
    Log::warnln("(10)warn-line");
    Log::error("(11)ERROR");
    Log::errorln("(12)error-line");
    Log::fatal("(13)FATAL");
    Log::fatalln("(14)fatal-line");

    // Fatal is Panic! Can be used as the first argument of.
    // panic!(Log::fatal(&format!("Invalid number=|{}|", 99)));

    // Note: It's usually weird to change the level twice in
    // a program.
    // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
    // |                                             *****|
    Log::set_level(Level::Trace);

    Log::trace("(7)Trace on (7)Trace.");
    Log::debug("(6)Debug on (7)Trace.");
    Log::info("(5)Info on (7)Trace.");
    Log::notice("(4)Notice on (7)Trace.");
    Log::warn("(3)Warn on (7)Trace.");
    Log::error("(2)Error on (7)Trace.");
    Log::fatal("(1)Fatal on (7)Trace.");

    // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
    // |                                      *****       |
    Log::set_level(Level::Debug);

    Log::trace("(7)Trace on (6)debug. Skip!");
    Log::debug("(6)Debug on (6)debug.");
    Log::info("(5)Info on (6)debug.");
    Log::notice("(4)Notice on (6)debug.");
    Log::warn("(3)Warn on (6)debug.");
    Log::error("(2)Error on (6)debug.");
    Log::fatal("(1)Fatal on (6)debug.");

    // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
    // |                               *****              |
    Log::set_level(Level::Info);

    Log::trace("(7)Trace on (5)Info. Skip!");
    Log::debug("(6)Debug on (5)Info. Skip!");
    Log::info("(5)Info on (5)Info.");
    Log::notice("(4)Notice on (5)Info.");
    Log::warn("(3)Warn on (5)Info.");
    Log::error("(2)Error on (5)Info.");
    Log::fatal("(1)Fatal on (5)Info.");

    // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
    // |                      ******                      |
    Log::set_level(Level::Notice);

    Log::trace("(7)Trace on (4)Notice. Skip!");
    Log::debug("(6)Debug on (4)Notice. Skip!");
    Log::info("(5)Info on (4)Notice. Skip!");
    Log::notice("(4)Notice on (4)Notice.");
    Log::warn("(3)Warn on (4)Notice.");
    Log::error("(2)Error on (4)Notice.");
    Log::fatal("(1)Fatal on (4)Notice.");

    // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
    // |               ****                               |
    Log::set_level(Level::Warn);

    Log::trace("(7)Trace on (3)Warn. Skip!");
    Log::debug("(6)Debug on (3)Warn. Skip!");
    Log::info("(5)Info on (3)Warn. Skip!");
    Log::notice("(4)Notice on (3)Warn. Skip!");
    Log::warn("(3)Warn on (3)Warn.");
    Log::error("(2)Error on (3)Warn.");
    Log::fatal("(1)Fatal on (3)Warn.");

    // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
    // |       *****                                      |
    Log::set_level(Level::Error);

    Log::trace("(7)Trace on (2)Error. Skip!");
    Log::debug("(6)Debug on (2)Error. Skip!");
    Log::info("(5)Info on (2)Error. Skip!");
    Log::notice("(4)Notice on (2)Error. Skip!");
    Log::warn("(3)Warn on (2)Error. Skip!");
    Log::error("(2)Error on (2)Error.");
    Log::fatal("(1)Fatal on (2)Error.");

    // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
    // |*****                                             |
    Log::set_level(Level::Fatal);

    Log::trace("(7)Trace on (1)Fatal. Skip!");
    Log::debug("(6)Debug on (1)Fatal. Skip!");
    Log::info("(5)Info on (1)Fatal. Skip!");
    Log::notice("(4)Notice on (1)Fatal. Skip!");
    Log::warn("(3)Warn on (1)Fatal. Skip!");
    Log::error("(2)Error on (1)Fatal. Skip!");
    Log::fatal("(1)Fatal on (1)Fatal.");

    Log::set_level(Level::Info);
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
            .bool("PaidRent", true),
    );

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
            // テーブルの配列です。
            .sub_aot(
                "IHaveToCleanMyRoom",
                ArrayOfTable::default()
                    .table(Table::default().str("Name", "Kitchen").bool("Clean", false))
                    .table(Table::default().str("Name", "Bath").bool("Wash", false))
                    .table(Table::default().str("Name", "Toilet").bool("Brush", false)),
            ),
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

    // Wait for logging to complete or to timeout.
    Log::flush();
}
