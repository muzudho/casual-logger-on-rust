//! All features are described in one copy and paste.

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

        // The higher this level, the more will be omitted.
        //
        // |<-- Low Level --------------------- High level -->|
        // |<-- High priority --------------- Low priority -->|
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        logger.level = Level::Trace;

        // Wait for seconds logging to complete when fatal.
        // By default it's set to 30 seconds,
        // so you probably don't need to set it.
        logger.fatal_timeout_secs = 30;

        // Remove old log files. This is determined by the
        // StartDate in the filename.
        logger.retention_days = 2;
        logger.remove_old_logs()
    } else {
        // Setup failed. Continue with the default settings.
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
        // |                                             *****|
        logger.level = Level::Trace;
    }

    Log::trace("(7)Trace on (7)Trace.");
    Log::debug("(6)Debug on (7)Trace.");
    Log::info("(5)Info on (7)Trace.");
    Log::notice("(4)Notice on (7)Trace.");
    Log::warn("(3)Warn on (7)Trace.");
    Log::error("(2)Error on (7)Trace.");
    Log::fatal("(1)Fatal on (7)Trace.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        // |                                      *****       |
        logger.level = Level::Debug;
    }

    Log::trace("(7)Trace on (6)debug. Skip!");
    Log::debug("(6)Debug on (6)debug.");
    Log::info("(5)Info on (6)debug.");
    Log::notice("(4)Notice on (6)debug.");
    Log::warn("(3)Warn on (6)debug.");
    Log::error("(2)Error on (6)debug.");
    Log::fatal("(1)Fatal on (6)debug.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        // |                               *****              |
        logger.level = Level::Info;
    }

    Log::trace("(7)Trace on (5)Info. Skip!");
    Log::debug("(6)Debug on (5)Info. Skip!");
    Log::info("(5)Info on (5)Info.");
    Log::notice("(4)Notice on (5)Info.");
    Log::warn("(3)Warn on (5)Info.");
    Log::error("(2)Error on (5)Info.");
    Log::fatal("(1)Fatal on (5)Info.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        // |                      ******                      |
        logger.level = Level::Notice;
    }

    Log::trace("(7)Trace on (4)Notice. Skip!");
    Log::debug("(6)Debug on (4)Notice. Skip!");
    Log::info("(5)Info on (4)Notice. Skip!");
    Log::notice("(4)Notice on (4)Notice.");
    Log::warn("(3)Warn on (4)Notice.");
    Log::error("(2)Error on (4)Notice.");
    Log::fatal("(1)Fatal on (4)Notice.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        // |               ****                               |
        logger.level = Level::Warn;
    }

    Log::trace("(7)Trace on (3)Warn. Skip!");
    Log::debug("(6)Debug on (3)Warn. Skip!");
    Log::info("(5)Info on (3)Warn. Skip!");
    Log::notice("(4)Notice on (3)Warn. Skip!");
    Log::warn("(3)Warn on (3)Warn.");
    Log::error("(2)Error on (3)Warn.");
    Log::fatal("(1)Fatal on (3)Warn.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        // |       *****                                      |
        logger.level = Level::Error;
    }

    Log::trace("(7)Trace on (2)Error. Skip!");
    Log::debug("(6)Debug on (2)Error. Skip!");
    Log::info("(5)Info on (2)Error. Skip!");
    Log::notice("(4)Notice on (2)Error. Skip!");
    Log::warn("(3)Warn on (2)Error. Skip!");
    Log::error("(2)Error on (2)Error.");
    Log::fatal("(1)Fatal on (2)Error.");

    if let Ok(mut logger) = LOGGER.lock() {
        // |Fatal< Error < Warn < Notice < Info < Debug <Trace|
        // |*****                                             |
        logger.level = Level::Fatal;
    }

    Log::trace("(7)Trace on (1)Fatal. Skip!");
    Log::debug("(6)Debug on (1)Fatal. Skip!");
    Log::info("(5)Info on (1)Fatal. Skip!");
    Log::notice("(4)Notice on (1)Fatal. Skip!");
    Log::warn("(3)Warn on (1)Fatal. Skip!");
    Log::error("(2)Error on (1)Fatal. Skip!");
    Log::fatal("(1)Fatal on (1)Fatal.");

    // Wait for logging to complete. Time out 30 seconds.
    Log::wait_for_logging_to_complete(30, |secs, message| {
        // Do not call 'Log::xxxxx()' in this code block.
        println!("{} sec(s). {}", secs, message);
    });
}
