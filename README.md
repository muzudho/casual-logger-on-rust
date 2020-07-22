# casual_logger

This logger with **few settings** to repeat practice of many programming tutorials.  
Not for product use.  
このロガーは、多くのプログラミングチュートリアルの練習を  
繰り返したい人向けに、設定の手間が少ないようにしています。  

Japanese before machine translation is attached.  
機械翻訳する前の日本語を添えます。  

## Example 1

Minimal case.  
最小の例。  

Code:  

```rust
//! You can copy and paste and use immediately.  
//! コピー＆ペーストしてすぐに使用できます。  

use casual_logger::Log;

fn main() {
    Log::remove_old_logs();

    Log::info("Hello, world!!");

    Log::flush();
}
```

Output `default-2020-07-22.log.toml` automatically generated:  

```toml
["Now=2020-07-22T22:53:40+0900&Pid=13728&Thr=ThreadId(1)&Seq=1"]
Info = 'Hello, world!!'


```

## Example 2

Using TOML for log file is not bad except for production use.  
It's painful to spend 1.5 times as much time searching the tree in the game,  
but what you really want is to fix a bug.  
It is easy to add information to TOML and it is easy to see.  
ログファイルにTOMLを使用することは本番環境での使用を除いて悪くありません。  
ゲームの木探索時間が1.5倍になるのは辛いですが、  
本当に欲しいのは、バグを取ることです。  
TOMLに情報を追加するのは簡単で、しかも見やすいです。  

Code:  

```rust
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
```

Output `today-s-plan-2020-07-22.log.toml` automatically generated:  

```toml
["Now=2020-07-22T23:06:11+0900&Pid=13696&Thr=ThreadId(1)&Seq=1"]
Info = 'ShoppingToday'
FluorescentLight = -7000
Remark = '''
Buy shelves in the near month.
Replace the washing machine after a few years.
近い月に棚。
数年後に洗濯機買い替え。
'''
Rent = -40000
Salary = 190000
VacuumCleaner = -53000
VacuumCleanerPricesAtOtherStores = [ -63_000, -4_000, -10_000 ]


```

## Example 3

For practical purposes, specify the file retention days and log level.  
Specify important to force the file name.  
This is important when using 'casual_logger' in multiple libraries.  
Important designation is on a first-come-first-served basis.  
実際には、ファイルの保存日数とログレベルを指定します。  
重要を指定して、ファイル名を強制します。  
これは、複数のライブラリで 'カジュアル・ロガー' を使用する場合に重要です。  
重要指定は、先着順です。  

Code:  

```rust
//! If someone used "casual_logger" in some library,  
//! see how to override the settings.  
//! もし他のライブラリで誰かが 'casual_logger' を使って  
//! いたなら、設定を上書きする方法を確認してください。  

use casual_logger::{Extension, Level, Log, Table};

fn main() {
    // By specifying important, the setting will be
    // effective on a first come first serve basis.
    // You can always make it important,
    // so if you get lost, always omit important...
    // 重要を指定することで、設定は早い者勝ちで有効になります。
    // いつでも important にできるので、
    // 迷ったら常に important を省きましょう……
    Log::set_file_name_important("lesson1"); // For app use.
    Log::set_file_name("mischief1"); // For library use.

    Log::set_file_ext_important(Extension::LogToml);
    Log::set_file_ext(Extension::Log);

    Log::set_retention_days_important(2);
    Log::set_retention_days(31);

    Log::set_level_important(Level::Info);
    Log::set_level(Level::Notice);

    Log::remove_old_logs();

    // If there are more arguments, make a pre-judgment.
    // 引数が増えたら前判定しましょう。
    if Log::enabled(Level::Info) {
        Log::info_t(
            "This is an Application.",
            Table::default()
                .str(
                    "FileName",
                    &Log::get_file_name() //
                        .unwrap_or_else(|err| err),
                )
                .str(
                    "Extension",
                    &Log::get_file_ext_str() //
                        .unwrap_or_else(|err| err),
                )
                .uint(
                    "RetentionDays",
                    Log::get_retention_days() //
                        .unwrap_or_else(|_| 0)
                        .into(),
                )
                .str(
                    "Level",
                    &match Log::get_level() {
                        Ok(level) => format!(
                            "{:?}", //
                            level
                        )
                        .to_string(),
                        Err(e) => e.to_string(),
                    },
                ),
        );
    }

    Log::flush();
}
```

Output `lesson1-2020-07-23.log.toml` automatically generated:  

```toml
["Now=2020-07-23T00:05:03+0900&Pid=6316&Thr=ThreadId(1)&Seq=1"]
Info = 'This is an Application.'
Extension = '.log.toml'
FileName = 'lesson1'
Level = 'Info'
RetentionDays = 2


```

## Abstract

The concept used by beginners.

### 1. Used in one example, throw away

* **There is no** configuration file.
* **Rotate** log by date automatically.
* **Delete** old log files. (semi-automatic)
* Log files can **only be placed** in the working directory.
* Write policy is **one application, one log file**.  
    書込み方針は、 **１アプリケーション１ログファイル** です。  
    * Priority 1: First important log file.  
        優先順位１: 最初に重要指定したログ・ファイル。
    * Priority 2: Last specified log file.  
        優先順位２: 最後に指定したログ・ファイル。

### 2. Human readable log

* TOML does not spoil it.

### 3. Possibility as a tutorial

* **Short introduction**.
* Write the log as a TOML table, it can be **easily parsed**.

### Disclaim

* (1) In trade off for processing speed:
  * **Don't forget `Log::flush()`** for logging to complete at **end of program**.
  * `Log::flush()` is a waiting time of **20 milli second or more** before the logger ends.
* (2) In trade off for ease of introduction:
  * You can break the toml format. **Do not validate**.
* (3) In trade off for intelligence suggestion by text editor:
  * It **differs** from the standard Rust log interface.
* (4) In trade off for not stopping running:
  * If the log export fails, the **error is ignored**.

### Tested environment

* OS: `Windows 10`.
* Editor: `Visual studio code`.

## At first, Overall view

Your code:  

```rust
//! All features are described in one copy and paste.
//! すべての機能が1つのコピー＆ペーストで説明されています。

use casual_logger::{Extension, Level, Log, Opt, Table};

fn main() {
    // Example of Log file name:
    // ログ・ファイル名の例:
    //
    //      'tic-tac-toe-2020-07-11.log.toml'
    //       1----------           3--------
    //                  2----------
    //
    //       1 Prefix              3 Extention
    //         接頭辞                拡張子
    //                  2 StartDate
    //                    開始日
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

    // Wait for logging to complete or to timeout.
    Log::flush();
}
```

Output `./tic-tac-toe-2020-07-22.log.toml` auto generated:  

```toml
["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=1"]
Info = '''
Hello, world!!
こんにちわ、世界！！
'''

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=2"]
Info = "x is 100.\r\n"

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=3"]
Trace = '( 1)TRACE'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=4"]
Trace = "( 2)trace-line\r\n"

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=5"]
Debug = '( 3)DEBUG'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=6"]
Debug = "( 4)debug-line\r\n"

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=7"]
Info = '( 5)INFO'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=8"]
Info = "( 6)info-line\r\n"

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=9"]
Notice = '( 7)NOTICE'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=10"]
Notice = "( 8)notice-line\r\n"

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=11"]
Warn = '( 9)WARN'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=12"]
Warn = "(10)warn-line\r\n"

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=13"]
Error = '(11)ERROR'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=14"]
Error = "(12)error-line\r\n"

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=15"]
Fatal = '(13)FATAL'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=16"]
Fatal = "(14)fatal-line\r\n"

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=17"]
Trace = '(7)Trace on (7)Trace.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=18"]
Debug = '(6)Debug on (7)Trace.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=19"]
Info = '(5)Info on (7)Trace.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=20"]
Notice = '(4)Notice on (7)Trace.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=21"]
Warn = '(3)Warn on (7)Trace.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=22"]
Error = '(2)Error on (7)Trace.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=23"]
Fatal = '(1)Fatal on (7)Trace.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=24"]
Debug = '(6)Debug on (6)debug.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=25"]
Info = '(5)Info on (6)debug.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=26"]
Notice = '(4)Notice on (6)debug.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=27"]
Warn = '(3)Warn on (6)debug.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=28"]
Error = '(2)Error on (6)debug.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=29"]
Fatal = '(1)Fatal on (6)debug.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=30"]
Info = '(5)Info on (5)Info.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=31"]
Notice = '(4)Notice on (5)Info.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=32"]
Warn = '(3)Warn on (5)Info.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=33"]
Error = '(2)Error on (5)Info.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=34"]
Fatal = '(1)Fatal on (5)Info.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=35"]
Notice = '(4)Notice on (4)Notice.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=36"]
Warn = '(3)Warn on (4)Notice.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=37"]
Error = '(2)Error on (4)Notice.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=38"]
Fatal = '(1)Fatal on (4)Notice.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=39"]
Warn = '(3)Warn on (3)Warn.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=40"]
Error = '(2)Error on (3)Warn.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=41"]
Fatal = '(1)Fatal on (3)Warn.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=42"]
Error = '(2)Error on (2)Error.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=43"]
Fatal = '(1)Fatal on (2)Error.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=44"]
Fatal = '(1)Fatal on (1)Fatal.'

["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=45"]
Info = 'ShoppingToday'
Condition = 'A'
FluorescentLight = -7000
PaidRent = true
Remark = '''
Buy shelves in the near month.
Replace the washing machine after a few years.
近い月に棚。
数年後に洗濯機買い替え。
'''
Rent = -40000
Salary = 190000
ShelveDepth = 46.5
VacuumCleaner = -53000
VacuumCleanerPricesAtOtherStores = [ -63_000, -4_000, -10_000 ]
  ["Now=2020-07-22T22:58:07+0900&Pid=10388&Thr=ThreadId(1)&Seq=45".RestFood]
  BottoleOfTea = 1
  FrozenRamen = 2
  Kimchi = 1


```

Output to terminal:  

```plain
casual_logger: 0 sec(s). 15 table(s) left.
casual_logger: 0 sec(s). 1 table(s) left.
casual_logger: 0 sec(s). 7 table(s) left.
casual_logger: 0 sec(s). 6 table(s) left.
casual_logger: 0 sec(s). 5 table(s) left.
casual_logger: 0 sec(s). 4 table(s) left.
casual_logger: 0 sec(s). 3 table(s) left.
casual_logger: 0 sec(s). 2 table(s) left.
casual_logger: 0 sec(s). 1 table(s) left.
casual_logger: 0 sec(s). 1 table(s) left.
```

It is designed to use `Log::fatal()` as the first argument for `panic!()`. It is the abnormal termination of the program. There is a waiting time.  

## At second, Description

Code:  

```rust
use casual_logger::{Extension, Level, Log, Table};
```

At the timing of the first writing, a file with a  
time stamp in its name is automatically generated.  

Example of Log file name:  
ログ・ファイル名の例:  

```plain
      tic-tac-toe-2020-07-22.log.toml
      1----------           3--------
                 2----------

      1 Prefix              3 Extention
        接頭辞                拡張子
                 2 StartDate
                   開始日
```

### File name

| Part          | Name      | Description       | Default     |
| ------------- | --------- | ----------------- | ----------- |
| `./`          | file path | Working directory |             |
|               |           | only.             |             |
| `tic-tac-toe` | Prefix    | Editable.         | `default`   |
| `-2020-07-22` | StartDate | Auto generated.   |             |
| `.log.toml`   | Extension | `.log.toml` or    | `.log.toml` |
|               |           | `.log`.           |             |

It is difficult to explain the **file path** for beginners.  
Therefore, it does not move.  

Excite yourself with a **prefix**.  

**StartDate** is basically today.  
If the rotation fails, it is the start date.

**`.log`** to be safe, include a word that  
clearly states that you can delete the file.  

If you don't like the .toml **extension**, leave  
the suffix empty and the .log extension.  

Extension:  

| Enum      | String      | Description                  | Default |
| --------- | ----------- | ---------------------------- | ------- |
| `Log`     | `.log`      | For logs that are too large  |         |
|           |             | to be colored in the editor. |         |
| `LogToml` | `.log.toml` | Toml format.                 | Default |

Set up, Code:  

```rust
fn main() {
    // ...

    // Prefix.
    Log::set_file_name("tic-tac-toe");
    // Extension.
    Log::set_file_ext(Extension::LogToml);

    // ...
}
```

### Log rotation

Code:  

```rust
    Log::set_retention_days(2);
    Log::remove_old_logs();
```

Example:  

* `retention_days` is 2.
* Today is 2020-07-12.
* Call `Log::remove_old_logs()` method.
* The `./default-2020-07-09.log.toml` file will be deleted.
* The `./default-2020-07-10.log.toml` remains.
* Delete old files by date in filename.

| Name             | Description                | Default |
| ---------------- | -------------------------- | ------- |
| `retention_days` | After this number of days, | `7`     |
|                  | the file will be deleted.  |         |

### Log level

Code:  

```rust
    Log::set_level(Level::Trace);
```

| Name    | Description            | Default |
| ------- | ---------------------- | ------- |
| `level` | Used to switch between | `Trace` |
|         | write and non-write.   |         |

Example:  

* There are 7 log levels.
  * `|Fatal< Error < Warn < Notice < Info < Debug <Trace|`
  * `|<-- Small ------------------------------ Large -->|`
  * `|<-- Concise -------------------------- Verbose -->|`
  * `|<-- Low Level --------------------- High level -->|`
  * `|<-- High priority --------------- Low priority -->|`

| Level    | Examle of use.                                     |
| -------- | -------------------------------------------------- |
| `Fatal`  | If the program cannot continue.                    |
| `Error`  | I didn't get the expected result,                  |
|          | so I'll continue with the other method.            |
| `Warn`   | It will be abnormal soon,                          |
|          | but there is no problem and you can ignore it.     |
|          | For example:                                       |
|          | (1) He reported that it took longer to access      |
|          | than expected.                                     |
|          | (2) Report that capacity is approaching the limit. |
| `Notice` | It must be enabled in the server production        |
|          | environment.                                       |
|          | Record of passing important points correctly.      |
|          | We are monitoring that it is working properly.     |
| `Info`   | Report highlights.                                 |
|          | Everything that needs to be reported regularly in  |
|          | the production environment.                        |
| `Debug`  | It should be in a place with many accidents.       |
|          | This level is disabled in production environments. |
|          | Leave it in the source and enable it for           |
|          | troubleshooting.                                   |
|          | Often, this is the production level of a desktop   |
|          | operating environment.                             |
| `Trace`  | Not included in the distribution.                  |
|          | Remove this level from the source after using it   |
|          | for debugging.                                     |
|          | If you want to find a bug in the program,          |
|          | write a lot.                                       |

Code:  

```rust
    // Multi-line string.
    // The suffix "ln" adds a newline at the end.
    Log::infoln(
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
```

A piece of advice.  

```rust
    // Fatal is Panic! Can be used as the first argument of.
    panic!(Log::fatal(&format!("Invalid number=|{}|", 99)));
```

Fatal returns a string so you can try to record a panic message.  
However, the last log may not be written if the program exits first.  
See also: **Log::set_timeout_secs()** method.  

Code:  

```rust
    // Wait for seconds logging to complete.
    // By default it's set to 30 seconds,
    // so you probably don't need to set it.
    Log::set_timeout_secs(30);
```

### Usage of Table

| Static method | Description        |
| ------------- | ------------------ |
| `::default()` | Create a instance. |

| Instance method        | Description                    |
| ---------------------- | ------------------------------ |
| `.str(key, value)`     | Insert a string.               |
|                        | Multi-line string are          |
|                        | output with multiple lines.    |
| `.literal(key, value)` | Not enclose this value in      |
|                        | quotation marks.               |
|                        | You can break the toml format. |
|                        | Do not validate.               |

Do not include spaces in the **key**. TOML collapses.  

It is difficult to explain to beginners how to use TOML.  
If you make a TOML that cannot be parsed **literal**ly,  
please correct it.  

Code:  

```rust
    // TOML say a table. So-called map.
    // Use table by '_t' suffix.
    // TOMLのテーブルは、いわゆるマップです。
    // '_t' を末尾に付けて、テーブルを使用します。
    Log::infoln_t(
        "The sky is from top to bottom!!
上から下まで空です！！",
        Table::default()
            .str(
                // Do not include spaces in your key.
                "ABird",
                "fly in the sky.",
            )
            // Not enclose this value in quotation marks.
            .literal("NumberOfSwimmingFish", "2")
            .str(
                "ThreeMonkeys",
                "climb
a tall
tree.",
            ),
    );
```

Output:  

```toml
["Now=2020-07-12 18:35:23&Pid=20872&Thr=ThreadId(1)&Seq=18"]
Info = """
The sky is from top to bottom!!
上から下まで空です！！\r\n
"""
ABird = "fly in the sky."
NumberOfSwimmingFish = 2
ThreeMonkeys = """
climb
a tall
tree.
"""

```

### Don't forget flush for logging to complete at end of program

Code:  

```rust
    // Wait for logging to complete or to timeout.
    Log::flush();
```

If you do not flush,  
the program will exit before writing all the logs.  

## TODO

* [ ] Dogfooding.
* [x] Japanese(Multi-byte string) support.
* [ ] More minimal.
* [ ] Remove deprecated features.
  * [x] 0.6.0
* [ ] Error handling check.
* [ ] Toml cover.
  * [x] Primitive type.
  * [ ] Array.
  * [x] Dotted key support (Sub table only).
* [x] Add '_important()' method.

## Appendix

### Customize method

Code: main.rs  

```rust
use casual_logger::{Level, Log};

pub trait LogExt {
    fn println(s: &str);
}
impl LogExt for Log {
    /// Info level logging and add print to stdout.
    fn println(s: &str) {
        if Log::enabled(Level::Info) {
            println!("{}", s);
        }
        Log::infoln(s);
    }
}
```

Usage: other.rs

```rust
use crate::LogExt;

pub fn test() {
    Log::println("Hello, world!!");
}
```
