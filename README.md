# casual_logger

This logger is intended to be **easy to explain** when teaching other example programs to friends.  
Not for you, for self-study of beginner friends.  
Of course you can use it.  
Not for production, but better than not logging.  

* Only write to 1 file on working directory.
* Rotate by date.
* Delete old files.

## At first, Disclaim

* It differs from the standard Rust log interface.
* Ignore performance for ease of use and **ease of explanation**.
* You **can break** the toml format. Do not validate.
* The **writing order is unstable**. Check the serial "Seq" number.
* If the log export fails, the **error is ignored** and it continues.
* **Don't forget** wait for logging to complete at end of program.

## At second, Overall view

Your code:  

```rust
use casual_logger::{Level, Log, Table, LOGGER};

fn main() {
    let remove_num = if let Ok(mut logger) = LOGGER.lock() {
        // Do not call 'Log::xxxxx()' in this code block.
        //
        // Set file name.
        //
        // All: 'tic-tac-toe-2020-07-12.log.toml'
        // Prefix: 'tic-tac-toe'
        // StartDate: '-2020-07-12' automatically.
        // Suffix: '.log' - To be safe, include a word that
        //         clearly states that you can delete the file.
        // Extention: '.toml'
        //
        // If you don't like the .toml extension, leave the
        // suffix empty and the .log extension.
        logger.set_file_name("tic-tac-toe", ".log", ".toml");

        logger.retention_days = 2;
        // The higher this level, the more will be omitted.
        //
        // |<-- Low Level --------------------- High level -->|
        // |<-- High priority --------------- Low priority -->|
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        logger.level = Level::Trace;
        // Remove old log files. This is determined by the
        // StartDate in the filename.
        logger.remove_old_logs()
    } else {
        0
    };
    Log::noticeln(&format!("Remove {} files.", remove_num));

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
    Log::trace("A,");
    Log::traceln("B,");
    Log::debug("C,");
    Log::debugln("D,");
    Log::info("E,");
    Log::infoln("F,");
    Log::notice("G,");
    Log::noticeln("H,");
    Log::warn("I,");
    Log::warnln("J,");
    Log::error("K,");
    Log::errorln("L,");
    Log::fatal("M,");
    Log::fatalln("N!");

    // Suffix '_t'. TOML say a table. So-called map.
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

    // Wait for logging to complete. Time out 30 seconds.
    Log::wait_for_logging_to_complete(
        30, |elapsed_secs, rest_threads|
    {
        println!(
            "{} second(s). Wait for {} thread(s).",
            elapsed_secs, rest_threads
        );
    });
}
```

Output `./default-2020-07-13.log.toml` auto generated:  

```toml
["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=3"]
Info = "x is 100.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=24"]
Error = "(2)Error on (7)Trace.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=33"]
Notice = "(4)Notice on (5)Info.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=28"]
Notice = "(4)Notice on (6)debug.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=22"]
Notice = "(4)Notice on (7)Trace.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=27"]
Info = "(5)Info on (6)debug.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=23"]
Warn = "(3)Warn on (7)Trace.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=29"]
Warn = "(3)Warn on (6)debug.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=30"]
Error = "(2)Error on (6)debug.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=31"]
Fatal = "(1)Fatal on (6)debug.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=32"]
Info = "(5)Info on (5)Info.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=25"]
Fatal = "(1)Fatal on (7)Trace.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=34"]
Warn = "(3)Warn on (5)Info.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=11"]
Notice = "H,\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=1"]
Notice = "Remove 0 files.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=35"]
Error = "(2)Error on (5)Info.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=36"]
Fatal = "(1)Fatal on (5)Info.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=37"]
Notice = "(4)Notice on (4)Notice.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=5"]
Trace = "B,\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=38"]
Warn = "(3)Warn on (4)Notice.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=39"]
Error = "(2)Error on (4)Notice.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=40"]
Fatal = "(1)Fatal on (4)Notice.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=41"]
Warn = "(3)Warn on (3)Warn.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=42"]
Error = "(2)Error on (3)Warn.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=43"]
Fatal = "(1)Fatal on (3)Warn.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=45"]
Fatal = "(1)Fatal on (2)Error.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=44"]
Error = "(2)Error on (2)Error.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=46"]
Fatal = "(1)Fatal on (1)Fatal.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=2"]
Info = """
Hello, world!!
こんにちわ、世界！！\r\n
"""

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=12"]
Warn = "I,"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=15"]
Error = "L,\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=10"]
Notice = "G,"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=7"]
Debug = "D,\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=8"]
Info = "E,"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=6"]
Debug = "C,"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=14"]
Error = "K,"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=13"]
Warn = "J,\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=17"]
Fatal = "N!\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=9"]
Info = "F,\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=19"]
Trace = "(7)Trace on (7)Trace.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=4"]
Trace = "A,"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=20"]
Debug = "(6)Debug on (7)Trace.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=16"]
Fatal = "M,"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=26"]
Debug = "(6)Debug on (6)debug.\r\n"

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=18"]
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

["Now=2020-07-13 00:04:56&Pid=9012&Thr=ThreadId(1)&Seq=21"]
Info = "(5)Info on (7)Trace.\r\n"

```

## At third, Description

Code:  

```rust
use casual_logger::{Level, Log, Table, LOGGER};
```

At the timing of the first writing, a file with a  
time stamp in its name is automatically generated.  
For example: `./tic-tac-toe-2020-07-12.log.toml`  

Description:  

| Part          | Name      | Description       | Default   |
| ------------- | --------- | ----------------- | --------- |
| `./`          | file path | Working directory |           |
|               |           | only.             |           |
| `tic-tac-toe` | Prefix    | Editable.         | `default` |
| `-2020-07-12` | StartDate | Auto generated.   |           |
| `.log`        | Suffix    | Editable.         | `.log`    |
| `.toml`       | Extension | Editable.         | `.toml`   |


It is difficult to explain the **file path** for beginners.  
Therefore, it does not move.  

Excite yourself with a **prefix**.  

**StartDate** is basically today.  
If the rotation fails, it is the start date.

**Suffix** to be safe, include a word that  
clearly states that you can delete the file.  

If you don't like the .toml **extension**, leave  
the suffix empty and the .log extension.  

Set up, Code:  

```rust
fn main() {
    if let Ok(mut logger) = LOGGER.lock() {
        logger.set_file_name("tic-tac-toe", ".log", ".toml");
        logger.retention_days = 2;
        logger.level = Level::Trace;
    }

    // ...
}
```

Log rotation, Code:  

```rust
    let remove_num = if let Ok(mut logger) = LOGGER.lock() {
        logger.remove_old_logs()
    } else {
        0
    };
    Log::noticeln(&format!("Remove {} files.", remove_num));
```

### Logger Properties

| Name             | Description                | Default |
| ---------------- | -------------------------- | ------- |
| `retention_days` | After this number of days, | `7`     |
|                  | the file will be deleted.  |         |
| `level`          | Used to switch between     | `Trace` |
|                  | write and non-write.       |         |

Example of **retention_days**:  

* `retention_days` is 2.
* Today is 2020-07-12.
* The `./default-2020-07-09.log.toml` file will be deleted.
* The `./default-2020-07-10.log.toml` remains.
* Delete old files by date in filename.

Example of **level**:  

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
    Log::trace("A,");
    Log::traceln("B,");
    Log::debug("C,");
    Log::debugln("D,");
    Log::info("E,");
    Log::infoln("F,");
    Log::notice("G,");
    Log::noticeln("H,");
    Log::warn("I,");
    Log::warnln("J,");
    Log::error("K,");
    Log::errorln("L,");
    Log::fatal("M,");
    Log::fatalln("N!");
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
    // Suffix '_t'. TOML say a table. So-called map.
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

### Don't forget wait for logging to complete at end of program

Code:  

```rust
    // Wait for logging to complete. Time out 30 seconds.
    Log::wait_for_logging_to_complete(
        30, |elapsed_secs, rest_threads|
    {
        println!(
            "{} second(s). Wait for {} thread(s).",
            elapsed_secs, rest_threads
        );
    });
```

If you do not wait,  
the program will exit before writing all the logs.  

## TODO

* [ ] Output a stable log order.

## Tested environment

* OS: `Windows 10`.
* Editor: `Visual studio code`.

## Appendix

### Customize method

Code:  main.rs  

```rust
use casual_logger::Log;

pub trait LogExt {
    fn println(s: &str);
}
impl LogExt for Log {
    /// Info level logging and add print to stdout.
    fn println(s: &str) {
        println!("{}", s);
        Log::infoln(s);
    }
}
```

Usage:  other.rs

```rust
use crate::LogExt;

pub fn test() {
    Log::println("Hello, world!!");
}
```