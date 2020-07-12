//! Run:
//!
//! ```
//! cargo run --example example
//! ```
use casual_logger::{Level, Log, Table, LOGGER};

fn main() {
    let remove_num = if let Ok(mut logger) = LOGGER.lock() {
        // Do not call 'Log::xxxxx()' in this code block.
        //
        // Set file name.
        //
        // All: 'tic-tac-toe-2020-07-11.log.toml'
        // Prefix: 'tic-tac-toe'
        // StartDate: '-2020-07-11' automatically.
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
        //  StartDate in the filename.
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

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        logger.level = Level::Trace;
    }

    Log::traceln("(7)Trace on (7)Trace.");
    Log::debugln("(6)Debug on (7)Trace.");
    Log::infoln("(5)Info on (7)Trace.");
    Log::noticeln("(4)Notice on (7)Trace.");
    Log::warnln("(3)Warn on (7)Trace.");
    Log::errorln("(2)Error on (7)Trace.");
    Log::fatalln("(1)Fatal on (7)Trace.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        logger.level = Level::Debug;
    }

    Log::traceln("(7)Trace on (6)debug.");
    Log::debugln("(6)Debug on (6)debug.");
    Log::infoln("(5)Info on (6)debug.");
    Log::noticeln("(4)Notice on (6)debug.");
    Log::warnln("(3)Warn on (6)debug.");
    Log::errorln("(2)Error on (6)debug.");
    Log::fatalln("(1)Fatal on (6)debug.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        logger.level = Level::Info;
    }

    Log::traceln("(7)Trace on (5)Info.");
    Log::debugln("(6)Debug on (5)Info.");
    Log::infoln("(5)Info on (5)Info.");
    Log::noticeln("(4)Notice on (5)Info.");
    Log::warnln("(3)Warn on (5)Info.");
    Log::errorln("(2)Error on (5)Info.");
    Log::fatalln("(1)Fatal on (5)Info.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        logger.level = Level::Notice;
    }

    Log::traceln("(7)Trace on (4)Notice.");
    Log::debugln("(6)Debug on (4)Notice.");
    Log::infoln("(5)Info on (4)Notice.");
    Log::noticeln("(4)Notice on (4)Notice.");
    Log::warnln("(3)Warn on (4)Notice.");
    Log::errorln("(2)Error on (4)Notice.");
    Log::fatalln("(1)Fatal on (4)Notice.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        logger.level = Level::Warn;
    }

    Log::traceln("(7)Trace on (3)Warn.");
    Log::debugln("(6)Debug on (3)Warn.");
    Log::infoln("(5)Info on (3)Warn.");
    Log::noticeln("(4)Notice on (3)Warn.");
    Log::warnln("(3)Warn on (3)Warn.");
    Log::errorln("(2)Error on (3)Warn.");
    Log::fatalln("(1)Fatal on (3)Warn.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        logger.level = Level::Error;
    }

    Log::traceln("(7)Trace on (2)Error.");
    Log::debugln("(6)Debug on (2)Error.");
    Log::infoln("(5)Info on (2)Error.");
    Log::noticeln("(4)Notice on (2)Error.");
    Log::warnln("(3)Warn on (2)Error.");
    Log::errorln("(2)Error on (2)Error.");
    Log::fatalln("(1)Fatal on (2)Error.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        logger.level = Level::Fatal;
    }

    Log::traceln("(7)Trace on (1)Fatal.");
    Log::debugln("(6)Debug on (1)Fatal.");
    Log::infoln("(5)Info on (1)Fatal.");
    Log::noticeln("(4)Notice on (1)Fatal.");
    Log::warnln("(3)Warn on (1)Fatal.");
    Log::errorln("(2)Error on (1)Fatal.");
    Log::fatalln("(1)Fatal on (1)Fatal.");

    // Wait for logging to complete. Time out 30 seconds.
    Log::wait_for_logging_to_complete(30, |elapsed_secs, rest_threads| {
        println!(
            "{} second(s). Wait for {} thread(s).",
            elapsed_secs, rest_threads
        );
    });
}
