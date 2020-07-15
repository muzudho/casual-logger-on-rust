//! All features are described in one copy and paste.

use casual_logger::{Extension, Level, Log, Table};

fn main() {
    // Example of Log file name:
    //
    //      'tic-tac-toe-2020-07-11.log.toml'
    //       -----------
    //       Prefix     -----------
    //                  StartDate  ----
    //                             Suffix
    //                                 -----
    //                                 Extention
    //
    // - StartDate is automatically added.
    //
    // Set the prefix with 'set_file_name' method.
    Log::set_file_name("tic-tac-toe");
    // Log file extension.
    //
    // '.log.toml' or '.log'.
    // '.log' for safety, include a word that
    // clearly states that you can delete the file.
    // If you don't like the .toml extension, change.
    Log::set_file_ext(Extension::LogToml);
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

    // Set to true to allow Casual_logger to
    // output information to stdout and stderr.
    // By default it's set to false,
    // so you probably don't need to set it.
    Log::set_development(true);

    // Remove old log files. This is determined by the
    // StartDate in the filename.
    Log::remove_old_logs();

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

    // Wait for logging to complete or to timeout.
    Log::wait();
}
