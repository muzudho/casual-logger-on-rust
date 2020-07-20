//! You can copy and paste and use immediately.

use casual_logger::Log;

fn main() {
    Log::remove_old_logs();

    panic!(Log::fatal("Critical message!"));
}
