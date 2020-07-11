# casual_logger

A logger used when practicing the sample programs.  
Only write to file, rotate by date.  
Not for hard users.

## At first, Disclaim

* It differs from the standard Rust log interface.
* Ignore performance for ease of use.

## At second, Overall view

Your code:  

```rust
use casual_logger::{Level, Log, LOGGER};

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
        // |<-- Low Level ----------------------- High level -->|
        // |<-- High priority ----------------- Low priority -->|
        // |Fatal < Error < Warn < Notice < Info < Debug < Trace|
        logger.level = Level::Trace;
        // Remove old log files. This is determined by the
        // StartDate in the filename.
        logger.remove_old_logs()
    } else {
        0
    };
    Log::noticeln(&format!("Remove file count={}", remove_num));

    Log::infoln(
        "Hello, world!!
こんにちわ、世界！！",
    );

    if Log::enabled(Level::Info) {
        let x = 100; // Time-consuming preparation, here.
        Log::infoln(&format!("x is {}.", x));
    }

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
}
```

Output `./default-2020-07-12.log.toml` auto generated:  

```toml
["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=1"]
Notice = "Remove file count=0\r\n"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=2"]
Info = """
Hello, world!!
こんにちわ、世界！！\r\n
"""

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=3"]
Info = "x is 100.\r\n"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=4"]
Trace = "A,"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=5"]
Trace = "B,\r\n"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=6"]
Debug = "C,"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=7"]
Debug = "D,\r\n"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=8"]
Info = "E,"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=9"]
Info = "F,\r\n"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=10"]
Notice = "G,"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=11"]
Notice = "H,\r\n"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=12"]
Warn = "I,"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=13"]
Warn = "J,\r\n"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=14"]
Error = "K,"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=15"]
Error = "L,\r\n"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=16"]
Fatal = "M,"

["Now=2020-07-12 00:49:17&Pid=19548&Thr=ThreadId(1)&Seq=17"]
Fatal = "N!\r\n"

```

## At third, Description

Code:  

```rust
use casual_logger::{Level, Log, LOGGER};
```

At the timing of the first writing, a file with a  
time stamp in its name is automatically generated.  
For example: `./tic-tac-toe-2020-07-12.log.toml`  

Description:  

* `./` - Working directory only.
* `tic-tac-toe` - Prefix. Editable. Default: `default`.
* `-2020-07-12` - StartDate. Auto generated.
* `.log` - Suffix. Editable. Default: `.log`.

Suffix to be safe, include a word that  
clearly states that you can delete the file.  

* `.toml` - Extension. Editable. Default: `.toml`.

If you don't like the .toml extension, leave  
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

Description of **retention_days**:  

* For example, `retention_days` is 2. Default: `7`.
* Today is 2020-07-12.
* The `./default-2020-07-09.log.toml` file will be deleted.
* The `./default-2020-07-10.log.toml` remains.
* Delete old files by date in filename.

Description of **level**:  

* There are 7 log levels. Default: `Trace`.
  * `Fatal < Error < Warn < Notice < Info < Debug < Trace`.
* Example:
  * `Log::info("Hello, world!!");`
  * `Log::infoln("Hello, world!!");`
  * `if Log::enabled(Level::Info) {Log::infoln("Hello!");}`

Code:  

```rust
    Log::infoln(
        "Hello, world!!
こんにちわ、世界！！",
    );

    if Log::enabled(Level::Info) {
        let x = 100; // Time-consuming preparation, here.
        Log::infoln(&format!("x is {}.", x));
    }

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

## TODO

* [ ] Adding table items as toml.

## Tested environment

* OS: `Windows 10`.
* Editor: `Visual studio code`.
