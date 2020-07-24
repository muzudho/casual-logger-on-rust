use casual_logger::{Log, Table};

fn main() {
    Log::remove_old_logs();

    let key = "YourWeight";
    let value = 97.0;

    Log::info_t("", Table::default().str("key", key).float("value", value));

    Log::flush();
}
