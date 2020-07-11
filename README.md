# casual-logger-on-rust

A logger that can be easily installed to simplify the sample program.  
Ignore performance for ease of use.  
It only supports writing to files and deleting old log files.  

## How to use

```rust
use casual_logger::{Level, Log, LOGGER};

fn main() {
    let remove_file_count = if let Ok(mut logger) = LOGGER.lock() {
        // Do not call 'Log::xxxxx()' in this code block.
        //
        // Set file name.
        //
        // All: 'tic-tac-toe-2020-07-11.log.toml'
        // Prefix: 'tic-tac-toe'
        // StartDate: '-2020-07-11' automatically.
        // Suffix: '.log' - To be safe, include a word that clearly states that you can delete the file.
        // Extention: '.toml'
        //
        // If you don't like the .toml extension, leave the suffix empty and the .log extension.
        logger.set_file_name("tic-tac-toe", ".log", ".toml");

        logger.retention_days = 2;
        // The higher this level, the more will be omitted.
        //
        // |<-- Low Level ------------------------- High level -->|
        // |<-- High priority ------------------- Low priority -->|
        // | Fatal < Error < Warn < Notice < Info < Debug < Trace |
        logger.level = Level::Trace;
        // Remove old log files. This is determined by the StartDate in the filename.
        logger.remove_old_logs()
    } else {
        0
    };
    Log::noticeln(&format!("Remove file count={}", remove_file_count));

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
