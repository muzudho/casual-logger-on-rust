# casual_logger

What a bother. I want to logging it **without setting it**. Not a product.  
なんて面倒だ。 **設定せず** にロギングしたい。 製品じゃないし。  

## Example 1

You can copy and paste and use immediately.  
コピー＆ペーストしてすぐに使用できます。  

Your code:  

```rust
use casual_logger::Log;

fn main() {
    Log::remove_old_logs();

    Log::info("Hello, world!!");

    Log::flush();
}
```

Output `./default-2020-07-23.log.toml` automatically generated:  

```toml
["Now=2020-07-23T19:49:05+0900&Pid=2820&Thr=ThreadId(1)&Seq=1"]
Info = 'Hello, world!!'


```

Terminal:  

```plain
casual_logger   | Remove 0 log file(s).
                | If you don't want this message, set `Log::set_opt(Opt::Release);`.
```

## Example 2

Sometimes you want the logger to be quiet.  
ロガーに静かにしていて欲しいときもありますね。  

Your code:  

```rust
//! There are 7 log levels.  
//! ログレベルは７段階です。  

use casual_logger::{Level, Log};

fn main() {
    Log::remove_old_logs();
    Log::set_level(Level::Notice); // Set.

    Log::trace("Stain on the wall of the room."); // Ignore it.
    Log::debug("There is no place to store clothes."); // Ignore it.
    Log::info("I turned on the air conditioner."); // Ignore it.
    Log::notice("The bath temperature is 44 degrees."); // Write.
    Log::warn("The refrigerator is empty."); // Write.
    Log::error("Where did you put my train pass?"); // Write.
    panic!(Log::fatal("I haven't set an alarm clock.")); // Write.

    // Log::flush(); // Log::Fatal() will flushes instead.
}
```

Output `./default-2020-07-23.log.toml` automatically generated:  

```toml
["Now=2020-07-23T19:51:22+0900&Pid=13560&Thr=ThreadId(1)&Seq=1"]
Notice = 'The bath temperature is 44 degrees.'

["Now=2020-07-23T19:51:22+0900&Pid=13560&Thr=ThreadId(1)&Seq=2"]
Warn = 'The refrigerator is empty.'

["Now=2020-07-23T19:51:22+0900&Pid=13560&Thr=ThreadId(1)&Seq=3"]
Error = 'Where did you put my train pass?'

["Now=2020-07-23T19:51:22+0900&Pid=13560&Thr=ThreadId(1)&Seq=4"]
Fatal = "I haven't set an alarm clock."


```

Terminal:  

```plain
casual_logger   | Remove 0 log file(s).
                | If you don't want this message, set `Log::set_opt(Opt::Release);`.
thread 'main' panicked at 'I haven't set an alarm clock.', examples\example2.rs:16:5
stack backtrace:
   0: backtrace::backtrace::trace_unsynchronized
...omitted...
```

## Example 3

Isn't it possible to take a variable length argument using a macro?  
マクロを使って可変長引数を取れるようにしないのですか？  

Question:  

```rust
    let key = "YourWeight";
    let value = 97.0;

    // Macro.
    println!("key={} value={}", key, value);

    // What a bother.
    Log::info(&format!("key={} value={}", key, value));
```

There is an alternative. Do the following:  
代替案があります。次のようにしてください:  

Your code:  

```rust
use casual_logger::{Log, Table};

fn main() {
    Log::remove_old_logs();

    let key = "YourWeight";
    let value = 97.0;

    Log::info_t(
        "",
        Table::default() //
            .str("key", key)
            .float("value", value),
    );

    Log::flush();
}
```

Output `./default-2020-07-25.log.toml` automatically generated:  

```toml
["Now=2020-07-25T04:37:30+0900&Pid=6500&Thr=ThreadId(1)&Seq=1"]
Info = ''
key = 'YourWeight'
value = 97


```

Terminal:  

```plain
casual_logger   | Remove 0 log file(s).
                | If you don't want this message, set `Log::set_opt(Opt::Release);`.
```

## Example 4

Is the log file TOML?  
ログファイルはTOMLですか？  

Yes.  
はい。  

Your code:  

```rust
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
```

Output `./today-s-plan-2020-07-23.log.toml` automatically generated:  

```toml
["Now=2020-07-23T19:52:34+0900&Pid=1232&Thr=ThreadId(1)&Seq=1"]
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

Terminal:  

```plain
casual_logger   | Remove 0 log file(s).
                | If you don't want this message, set `Log::set_opt(Opt::Release);`.
```

## Example 5

When will the log file disappear?  
ログファイルはいつ消えるの？  

This is when someone execute `Log::remove_old_logs();`.  
By default, it keeps files up to 7 days old.  
By file name, not by timestamp.  
`Log::remove_old_logs()` を実行したときです。  
デフォルトで７日前の日付まで残ります。  
タイムスタンプではなく、ファイル名によって。  

Before test:  

```plain
./default-2020-07-15.log.toml
./default-2020-07-16.log.toml
./default-2020-07-17.log.toml
./default-2020-07-18.log.toml
./default-2020-07-19.log.toml
./default-2020-07-20.log.toml
./default-2020-07-21.log.toml
./default-2020-07-22.log.toml
./default-2020-07-23.log.toml
```

Your code:  

```rust
use casual_logger::Log;

fn main() {
    // If set to 1, it will remain until yesterday.
    // If set to 0, it will remain until today.
    // If set to -1, it will be deleted until today.
    // If set to -2, it will be deleted until tomorrow.
    // 1 にすると昨日の分まで残る。
    // 0 にすると今日の分まで残る。
    // -1 にすると今日の分まで消える。
    // -2 にすると明日の分まで消える。
    Log::set_retention_days(-1);

    // Execute the deletion.
    // 削除を実行します。
    Log::remove_old_logs();

    Log::info("Hooray!");

    Log::flush();
}
```

Output `./default-2020-07-23.log.toml` automatically generated:  

```toml
["Now=2020-07-23T20:07:50+0900&Pid=19172&Thr=ThreadId(1)&Seq=1"]
Info = 'Hooray!'


```

Terminal:  

```plain
casual_logger   | Remove 9 log file(s).
                | If you don't want this message, set `Log::set_opt(Opt::Release);`.
```

After test:  

```plain
./default-2020-07-23.log.toml
```

## Example 6

We do not recommend making it complicated.  
複雑にすることはお勧めしません。  

Your code:  

```rust
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
```

Output `./complex-toml-2020-07-25.log.toml` automatically generated:  

```toml
["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1"]
Info = "I'm in trouble."
  [["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".IHaveToCleanMyRoom]]
  Clean = false
  Name = 'Kitchen'
  [["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".IHaveToCleanMyRoom]]
  Name = 'Bath'
  Wash = false
  [["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".IHaveToCleanMyRoom]]
  Brush = false
  Name = 'Toilet'
  ["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".RestFood]
  BottoleOfTea = 1
  FrozenRamen = 2
  Kimchi = 1
  [["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".SubArrayOfSubTable]]
    ["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".SubArrayOfSubTable.0.SameName]
    Clean = false
    Name = 'Kitchen'
  [["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".SubArrayOfSubTable]]
    ["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".SubArrayOfSubTable.1.SameName]
    Name = 'Bath'
    Wash = false
  [["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".SubArrayOfSubTable]]
    ["Now=2020-07-25T09:36:35+0900&Pid=11084&Thr=ThreadId(1)&Seq=1".SubArrayOfSubTable.2.SameName]
    Brush = false
    Name = 'Toilet'


```

Terminal:  

```plain
casual_logger   | Remove 0 log file(s).
                | If you don't want this message, set `Log::set_opt(Opt::Release);`.
```

## Example 7

Q. Is there no configuration file in 'casual_logger' ?  
Q. 'casual_logger' に設定ファイルは無いのですか？  

A. There is no configuration file.  
A. 設定ファイルはありません。  

Q. What if I want to change the settings?  
Q. 設定を変えたくなったらどうすればいいのですか？  

A. Rebuild the source or create a configuration file by yourself.  
A. ソースをビルドし直すか、設定ファイルを自作してください。  

Q. What a bother. Why?  
Q. なんて面倒だ。どうして？  

A. Because it is a logger for those who have trouble setting.  
Not suitable for applications with settings.  
A. 設定が面倒な人のためのロガーだからです。  
設定のあるアプリケーションには適していません。  

The setting items are as follows:  
設定項目は以下の通りです:  

Your code:  

```rust
//! There is no configuration file.  
//! 設定ファイルはありません。  

use casual_logger::{Extension, Level, Log, Opt};

fn main() {
    Log::set_file_name("hello");
    Log::set_file_ext(Extension::Log);
    Log::set_retention_days(31);
    Log::remove_old_logs();

    Log::set_level(Level::Notice);
    Log::set_timeout_secs(60);
    Log::set_opt(Opt::Release);

    Log::notice("Hello, world!!");

    Log::flush();
}
```

Output `./hello-2020-07-25.log` automatically generated:  

```toml
["Now=2020-07-25T08:46:04+0900&Pid=13044&Thr=ThreadId(1)&Seq=1"]
Notice = 'Hello, world!!'


```

Terminal:  

```plain
casual_logger   | Remove 0 log file(s).
                | If you don't want this message, set `Log::set_opt(Opt::Release);`.
```

## Example 8

Q. What if someone else used 'casual_logger' in another library?  
Q. もし他のライブラリで誰かが 'casual_logger' を使っていたなら、  
どうなるでしょうか？  

A. I think the logs will be crossed.  
A. ログは混線すると思います。  

Q. How can I avoid cross logging?  
Q. ログの混線を避けるにはどうすればよいですか？  

A. When embedding 'casual_logger' in a library, why not comment out  
something less serious than the Error level?  
Or it is a good option to replace it with another logger.  
A. 'casual_logger' をライブラリに埋め込むときは、Errorレベルより  
深刻でないものをコメントアウトするのはどうでしょうか？  
または、別のロガーに置き換えることをお勧めします。  

'casual_logger' does not maintain idempotency of functions, but it is first come first served:  
'casual_logger' では関数の冪等性を保つのではなく、早い者勝ちの方針です:  

Your code:  

```rust
//! See how to override the settings.  
//! 設定を上書きする方法を確認してください。  

use casual_logger::{Extension, Level, Log, Opt, Table};

fn main() {
    // By specifying important, the setting will be
    // effective on a first come first serve basis.
    // You can always make it important,
    // so if you get lost, always omit important...
    // 重要を指定することで、設定は早い者勝ちで有効になります。
    // いつでも important にできるので、
    // 迷ったら常に important を省きましょう……
    Log::set_file_name_important("important-example"); // Ok.
    Log::set_file_name("mischief1"); // Ignore it.

    Log::set_file_ext_important(Extension::LogToml); // Ok.
    Log::set_file_ext(Extension::Log); // Ignore it.

    Log::set_retention_days_important(2); // Ok.
    Log::set_retention_days(31); // Ignore it.

    // Delete the old log after setting the file name
    // and extension.
    // ファイル名、拡張子を設定したあとで、古いログを
    // 削除しましょう。
    Log::remove_old_logs();

    Log::set_level_important(Level::Info); // Ok.
    Log::set_level(Level::Notice); // Ignore it.

    Log::set_opt_important(Opt::Release); // Ok.
    Log::set_opt(Opt::Development); // Ignore it.

    // Now for confirmation. Just use the log.
    // If there are more arguments, make a pre-judgment.
    // さあ確認です。ちょうどログが使えます。
    // 引数が増えたら前判定しましょう。
    if Log::enabled(Level::Info) {
        Log::info_t(
            "This is an Application.",
            Table::default()
                .str(
                    "FileNameStem",
                    &Log::get_file_name() //
                        .unwrap_or_else(|err| err),
                )
                .str(
                    "Extension",
                    &Log::get_file_ext_str() //
                        .unwrap_or_else(|err| err),
                )
                .int(
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
                )
                .str(
                    "Optimization",
                    &match Log::get_opt() {
                        Ok(opt) => format!(
                            "{:?}", //
                            opt
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

Output `./important-example-2020-07-25.log.toml` automatically generated:  

```toml
["Now=2020-07-25T08:09:26+0900&Pid=18944&Thr=ThreadId(1)&Seq=1"]
Info = 'This is an Application.'
Extension = '.log.toml'
FileNameStem = 'important-example'
Level = 'Info'
Optimization = 'Release'
RetentionDays = 2


```

Terminal:  

```plain
casual_logger   | Remove 0 log file(s).
                | If you don't want this message, set `Log::set_opt(Opt::Release);`.
```

## Quality policy

Fail faster and improve faster.  
失敗するなら早い方がいい。  

### 1. casual_logger is entry model

Don't know how to log well? That's right. Rest assured.  
You can't find it even if You look it up because logging is ad hoc.  
うまくログを取る方法が分かりませんか？ ですよね。 安心してください。  
調べても見つからないのはロギングが場当たり的だからです。  

Some shoes don't fit your size. There are differences.  
What makes loggers hard is finding the right one for the job.  
自分のサイズに合わない靴もある程度の違いです。  
ロガーを困難にするのは、その仕事に適したロガーを見つけることです。  

More specifically, it is difficult to adjust.  
`casual_logger` aims to be a logger with almost inflexible.  
もっと言えば、調整が難しいのです。  
`casual_logger` は、ほとんど調整できないロガーを目指します。  

Since the hammer is fixed, please look for a nail that is easy to hit.  
ハンマーの方を固定するので、叩きやすい釘を探してください。  

" No settings, use immediately.  
Used in one example, throw away. "  
「 設定なし、すぐに使用。  
エグザンプルを1つ終われば、捨てるだけです 」  

* **There is no** configuration file.  
    設定ファイルはありません。
* Log files can **only be placed** in the working directory.  
    ログファイルは作業ディレクトリに置きます。
* **Rotate** log by date automatically.  
    ログファイルは日付で順繰りに作られます。
* **Delete** old log files. (semi-automatic)  
    半自動でログファイルを削除します。
* Write policy is **one application, one log file**.  
    書込み方針は、 **１アプリケーション１ログファイル** です。  
    * Priority 1: First important log file.  
        優先順位１: 最初に重要指定したログ・ファイル。
    * Priority 2: Last specified log file.  
        優先順位２: 最後に指定したログ・ファイル。
* It is for those who want to learn while using it.  
    使いながら覚える人向けです。

### 2. Connect to the competition

When you see a tournament, you'll want to try it with an entry model.  
大会を見たら、エントリーモデルでもあなたは試したくなる。  

* If the log level is set to fatal, there will be little  
    performance degradation. Good luck.  
    ログレベルをファータルにしてしまえば性能劣化はあんまりない。
* While setting to `Opt::Release`,  
    No message is output to standard output or error output.  
    Because there is a possibility that it will be a foul at the competition.  
    `Opt::Release`, に設定中は、  
    標準出力、エラー出力にメッセージを出しません。  
    大会で反則になる可能性があるからです。

### 3. Encourage you

You want to get more and more logs.  
あなたは どんどん ログを取りたくなる。  

* Human readable log. TOML does not spoil it.  
    人間が読めるログ。 TOMLはそれを台無しにしません。
* Write the log as a TOML table, it can be **easily parsed**.  
    ログをTOMLテーブルとして書き込みます。**解析しやすい**です。

## Disclaim

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

use casual_logger::{ArrayOfTable, Extension, Level, Log, Opt, Table};

fn main() {
    // Example of Log file name:
    // ログ・ファイル名の例:
    //
    // +
    // | tic-tac-toe-2020-07-11.log.toml
    // | 1----------           3---
    // |            2----------    4----
    // |
    // | 1 Prefix              3 Suffix
    // |   接頭辞                接尾辞
    // |            2 StartDate    4 Extention
    // |              開始日         拡張子
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
    // Log file suffix and extension:
    // 接尾辞、拡張子:
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
```

Output `./tic-tac-toe-2020-07-30.log.toml` automatically generated:  

```toml
["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=1"]
Info = '''
Hello, world!!
こんにちわ、世界！！
'''

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=2"]
Info = "x is 100.\\r\\n"

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=3"]
Trace = '( 1)TRACE'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=4"]
Trace = "( 2)trace-line\\r\\n"

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=5"]
Debug = '( 3)DEBUG'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=6"]
Debug = "( 4)debug-line\\r\\n"

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=7"]
Info = '( 5)INFO'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=8"]
Info = "( 6)info-line\\r\\n"

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=9"]
Notice = '( 7)NOTICE'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=10"]
Notice = "( 8)notice-line\\r\\n"

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=11"]
Warn = '( 9)WARN'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=12"]
Warn = "(10)warn-line\\r\\n"

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=13"]
Error = '(11)ERROR'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=14"]
Error = "(12)error-line\\r\\n"

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=15"]
Fatal = '(13)FATAL'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=16"]
Fatal = "(14)fatal-line\\r\\n"

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=17"]
Trace = '(7)Trace on (7)Trace.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=18"]
Debug = '(6)Debug on (7)Trace.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=19"]
Info = '(5)Info on (7)Trace.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=20"]
Notice = '(4)Notice on (7)Trace.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=21"]
Warn = '(3)Warn on (7)Trace.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=22"]
Error = '(2)Error on (7)Trace.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=23"]
Fatal = '(1)Fatal on (7)Trace.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=24"]
Debug = '(6)Debug on (6)debug.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=25"]
Info = '(5)Info on (6)debug.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=26"]
Notice = '(4)Notice on (6)debug.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=27"]
Warn = '(3)Warn on (6)debug.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=28"]
Error = '(2)Error on (6)debug.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=29"]
Fatal = '(1)Fatal on (6)debug.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=30"]
Info = '(5)Info on (5)Info.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=31"]
Notice = '(4)Notice on (5)Info.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=32"]
Warn = '(3)Warn on (5)Info.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=33"]
Error = '(2)Error on (5)Info.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=34"]
Fatal = '(1)Fatal on (5)Info.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=35"]
Notice = '(4)Notice on (4)Notice.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=36"]
Warn = '(3)Warn on (4)Notice.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=37"]
Error = '(2)Error on (4)Notice.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=38"]
Fatal = '(1)Fatal on (4)Notice.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=39"]
Warn = '(3)Warn on (3)Warn.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=40"]
Error = '(2)Error on (3)Warn.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=41"]
Fatal = '(1)Fatal on (3)Warn.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=42"]
Error = '(2)Error on (2)Error.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=43"]
Fatal = '(1)Fatal on (2)Error.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=44"]
Fatal = '(1)Fatal on (1)Fatal.'

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=45"]
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

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=46"]
Info = "I'm in trouble."
  [["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=46".IHaveToCleanMyRoom]]
  Clean = false
  Name = 'Kitchen'
  [["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=46".IHaveToCleanMyRoom]]
  Name = 'Bath'
  Wash = false
  [["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=46".IHaveToCleanMyRoom]]
  Brush = false
  Name = 'Toilet'
  ["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=46".RestFood]
  BottoleOfTea = 1
  FrozenRamen = 2
  Kimchi = 1

["Now=2020-07-30T01:49:36+0900&Pid=22136&Thr=ThreadId(1)&Seq=47"]
Info = "Primitive type conversion example.\\r\\n"
i128 = 1
i16 = 1
i32 = 1
i64 = 1
i8 = 1
isize = 1
u128 = 1
u16 = 1
u32 = 1
u64 = 1
u8 = 1
usize = 1


```

Terminal:  

```plain
casual_logger   | Remove 0 log file(s).
                | If you don't want this message, set `Log::set_opt(Opt::Release);`.
```

It is designed to use `Log::fatal()` as the first argument for `panic!()`. It is the abnormal termination of the program. There is a waiting time.  

## At second, Description

Your code:  

```rust
use casual_logger::{Extension, Level, Log, Table};
```

At the timing of the first writing, a file with a  
time stamp in its name is automatically generated.  

Example of Log file name:  
ログ・ファイル名の例:  

```plain
tic-tac-toe-2020-07-22.log.toml
1----------           3---
           2----------    4----

1 Prefix              3 Suffix
  接頭辞                接尾辞
           2 StartDate    4 Extention
             開始日         拡張子
```

### File name

| Part          | Name       | Description       | Default     |
| ------------- | ---------- | ----------------- | ----------- |
| `./`          | file path  | Working directory |             |
|               |            | only.             |             |
| `tic-tac-toe` | Prefix     | Editable.         | `default`   |
| `-2020-07-22` | StartDate  | Auto generated.   |             |
| `.log.toml`   | Suffix and | `.log.toml` or    | `.log.toml` |
|               | Extension  | `.log`.           |             |

It is difficult to explain the **file path** for beginners.  
Therefore, it does not move.  

Excite yourself with a **prefix**.  

**StartDate** is basically today.  
If the rotation fails, it is the start date.

**`.log`** to be safe, include a word that  
clearly states that you can delete the file.  

If you don't like the .toml **extension**, leave  
the suffix empty and the .log extension.  

Suffix and Extension:  

| Enum      | String      | Description                  | Default |
| --------- | ----------- | ---------------------------- | ------- |
| `Log`     | `.log`      | For logs that are too large  |         |
|           |             | to be colored in the editor. |         |
| `LogToml` | `.log.toml` | Toml format.                 | Default |

Your code:  

```rust
fn main() {
    // ...

    // Prefix.
    Log::set_file_name("tic-tac-toe");
    // Suffix and Extension.
    Log::set_file_ext(Extension::LogToml);

    // ...
}
```

### Log rotation

Your code:  

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

Your code:  

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

Your code:  

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

Your code:  

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
| `.bool(key, value)`    | Insert a boolean.              |
| `.char(key, value)`    | Insert a character.            |
| `.float(key, value)`   | Insert a float.                |
| `.int(key, value)`     | Insert a signed integer.       |
| `.literal(key, value)` | Not enclose this value in      |
|                        | quotation marks.               |
|                        | You can break the toml format. |
|                        | Do not validate.               |
| `.str(key, value)`     | Insert a string.               |
|                        | Multi-line string are          |
|                        | output with multiple lines.    |
| `.sub_t(key, table)`   | Insert a sub table.            |
| `.uint(key, value)`    | Insert a unsigned integer.     |

Do not include spaces in the **key**. TOML collapses.  

It is difficult to explain to beginners how to use TOML.  
If you make a TOML that cannot be parsed **literal**ly,  
please correct it.  

Your code:  

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

Your code:  

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

Your code: main.rs  

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
