//! You can copy and paste and use immediately.

use casual_logger::Log;

fn main() {
    Log::remove_old_logs();

    Log::info("Hello, world!!");

    Log::flush();
}
