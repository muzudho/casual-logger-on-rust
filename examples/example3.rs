//! If someone used "casual_logger" in some library,  
//! see how to override the settings.  
//! もし他のライブラリで誰かが 'casual_logger' を使って  
//! いたなら、設定を上書きする方法を確認してください。  

use casual_logger::{Extension, Level, Log, Table};

fn main() {
    // By specifying important, the setting will be
    // effective on a first come first serve basis.
    // You can always make it important,
    // so if you get lost, always omit important...
    // 重要を指定することで、設定は早い者勝ちで有効になります。
    // いつでも important にできるので、
    // 迷ったら常に important を省きましょう……
    Log::set_file_name_important("lesson1"); // For app use.
    Log::set_file_name("mischief1"); // For library use.

    Log::set_file_ext_important(Extension::LogToml);
    Log::set_file_ext(Extension::Log);

    Log::set_retention_days_important(2);
    Log::set_retention_days(31);

    Log::set_level_important(Level::Info);
    Log::set_level(Level::Notice);

    Log::remove_old_logs();

    // If there are more arguments, make a pre-judgment.
    // 引数が増えたら前判定しましょう。
    if Log::enabled(Level::Info) {
        Log::info_t(
            "This is an Application.",
            Table::default()
                .str(
                    "FileName",
                    &Log::get_file_name() //
                        .unwrap_or_else(|err| err),
                )
                .str(
                    "Extension",
                    &Log::get_file_ext_str() //
                        .unwrap_or_else(|err| err),
                )
                .uint(
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
                ),
        );
    }

    Log::flush();
}
