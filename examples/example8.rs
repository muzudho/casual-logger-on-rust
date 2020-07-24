//! See how to override the settings.  
//! 設定を上書きする方法を確認してください。  

use casual_logger::{Extension, Level, Log, Opt, Table};

fn main() {
    // By specifying important, the setting will be
    // effective on a first come first serve basis.
    // You can always make it important,
    // so if you get lost, always omit important...
    // 重要を指定することで、設定は早い者勝ちで有効になります。
    // いつでも important にできるので、
    // 迷ったら常に important を省きましょう……
    Log::set_file_name_important("important-example"); // Ok.
    Log::set_file_name("mischief1"); // Ignore it.

    Log::set_file_ext_important(Extension::LogToml); // Ok.
    Log::set_file_ext(Extension::Log); // Ignore it.

    Log::set_retention_days_important(2); // Ok.
    Log::set_retention_days(31); // Ignore it.

    // Delete the old log after setting the file name
    // and extension.
    // ファイル名、拡張子を設定したあとで、古いログを
    // 削除しましょう。
    Log::remove_old_logs();

    Log::set_level_important(Level::Info); // Ok.
    Log::set_level(Level::Notice); // Ignore it.

    Log::set_opt_important(Opt::Release); // Ok.
    Log::set_opt(Opt::Development); // Ignore it.

    // Now for confirmation. Just use the log.
    // If there are more arguments, make a pre-judgment.
    // さあ確認です。ちょうどログが使えます。
    // 引数が増えたら前判定しましょう。
    if Log::enabled(Level::Info) {
        Log::info_t(
            "This is an Application.",
            Table::default()
                .str(
                    "FileNameStem",
                    &Log::get_file_name() //
                        .unwrap_or_else(|err| err),
                )
                .str(
                    "Extension",
                    &Log::get_file_ext_str() //
                        .unwrap_or_else(|err| err),
                )
                .int(
                    "RetentionDays",
                    Log::get_retention_days() //
                        .unwrap_or_else(|_| 0)
                        .into(),
                )
                .str(
                    "Level",
                    &match Log::get_level() {
                        Ok(level) => format!(
                            "{:?}", //
                            level
                        )
                        .to_string(),
                        Err(e) => e.to_string(),
                    },
                )
                .str(
                    "Optimization",
                    &match Log::get_opt() {
                        Ok(opt) => format!(
                            "{:?}", //
                            opt
                        )
                        .to_string(),
                        Err(e) => e.to_string(),
                    },
                ),
        );
    }

    Log::flush();
}
