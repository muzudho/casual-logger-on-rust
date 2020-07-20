//! You can copy and paste and use immediately.  
//! コピー＆ペーストしてすぐに使用できます。  

use casual_logger::Log;

fn main() {
    Log::remove_old_logs();

    Log::info("Hello, world!!");

    Log::flush();
}
