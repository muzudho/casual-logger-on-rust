use casual_logger::Log;

fn main() {
    // If set to 1, it will remain until yesterday.
    // If set to 0, it will remain until today.
    // If set to -1, it will be deleted until today.
    // If set to -2, it will be deleted until tomorrow.
    // 1 にすると昨日の分まで残る。
    // 0 にすると今日の分まで残る。
    // -1 にすると今日の分まで消える。
    // -2 にすると明日の分まで消える。
    Log::set_retention_days(-1);

    // Execute the deletion.
    // 削除を実行します。
    Log::remove_old_logs();

    Log::info("Hooray!");

    Log::flush();
}
