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
