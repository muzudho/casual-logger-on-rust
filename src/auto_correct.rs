use crate::config::Logger;
use crate::stringifier::Stringifier;
use crate::Opt;
use regex::Regex;
use std::sync::Mutex;

lazy_static! {
    /// Without dot.
    static ref RE_TOML_KEY: Mutex<Regex> = Mutex::new(Regex::new(r"^[A-Za-z0-9_-]+$").unwrap());
    static ref RE_WHITE_SPACE: Mutex<Regex> = Mutex::new(Regex::new(r"\s").unwrap());
}

pub struct AutoCorrect {}
impl AutoCorrect {
    /// Correct the key automatically.  
    /// キーを補正します。  
    ///
    /// # Arguments
    ///
    /// * `key` - A key.  
    ///             キー。  
    ///
    /// # Returns
    ///
    /// Table.  
    /// テーブル。  
    pub fn correct_key(key: &str) -> String {
        match Logger::get_optimization() {
            Opt::Release => {
                return key.to_string();
            }
            _ => {}
        }

        // Check
        // TODO Dotted key support is difficult.
        if let Ok(re_toml_key) = RE_TOML_KEY.lock() {
            if re_toml_key.is_match(key) {
                // Ok.
                return key.to_string();
            }
        }

        if let Ok(re_white_space) = RE_WHITE_SPACE.lock() {
            // It will be corrected automatically.
            let bad = key;
            let better = format!(
                "\"{}\"",
                Stringifier::escape_double_quotation(&re_white_space.replace_all(key, " "))
            );
            let opt = Logger::get_optimization();
            match opt {
                Opt::BeginnersSupport | Opt::Development => {
                    println!(
                        "casual_logger   | Bad=|{}|
                | Not too bad=|{}|",
                        bad, better,
                    );
                }
                Opt::Release => {}
            }

            better
        } else {
            // TODO Error
            key.to_string()
        }
    }
}
