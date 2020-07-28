//! Escape control characters.  
//! 制御文字をエスケープします。  

use chrono::Local;
use regex::Regex;
use std::process;
use std::sync::Mutex;
use std::thread;

lazy_static! {
    /// Triple single quotation.
    /// ３連シングル・クォーテーション。
    static ref RE_TRIPLE_SINGLE_QUOTE: Mutex<Regex> = Mutex::new(Regex::new(r"'''").unwrap());
}

// For multi-platform. Windows, or not.
// マルチプラットフォーム対応。Windowsか、それ以外か。
#[cfg(windows)]
const NEW_LINE_SEQUENCE: &'static str = "\\r\\n";
#[cfg(windows)]
const NEW_LINE_CHARS: &'static [char; 2] = &['\r', '\n'];
#[cfg(not(windows))]
const NEW_LINE_SEQUENCE: &'static str = "\\n";
#[cfg(not(windows))]
const NEW_LINE_CHARS: &'static [char; 1] = &['\n'];

enum NewLineType {
    /// `\r\n`.
    Windows,
    /// `\n`
    NotWindows,
}

/// Unstable.  
/// Escape control characters.  
/// 仕様は変わることがあります。  
/// 制御文字をエスケープします。  
pub struct Stringifier {}
impl Stringifier {
    /// Table name to keep for ordering.  
    /// You can parse it easily by writing the table name like a GET query.  
    /// テーブル名は順を保ってください。  
    /// GETクエリのようにテーブル名を記述することで、簡単に解析できます。  
    pub fn create_identify_table_name(seq: u128) -> String {
        format!(
            "\"Now={}&Pid={}&Thr={}&Seq={}\"",
            Local::now().format("%Y-%m-%dT%H:%M:%S%z"),
            // Process ID.
            process::id(),
            Stringifier::thread_id().to_string(),
            seq
        )
        .to_string()
    }
    /// Automatic. Thread ID. However, Note that you are not limited to numbers.  
    /// 自動。スレッドID。ただし、数値に限定されないことに注意してください。  
    pub fn thread_id() -> String {
        format!("{:?}", thread::current().id())
    }

    /// Parse a string.  
    /// 文字列をパースします。  
    pub fn format_str_value(value: &str) -> String {
        // let value = Table::convert_multi_byte_string(slice);
        // Divide by A, B, C, E or F.
        // A) You must use multi-line ["""].
        //  * Multi-line string.
        // B) You must use one-line ["""].
        // C) You must use multi-line ['''].
        // D) You must use one-line ['''].
        // E) You must use ['].
        // F) Use ["].
        let multi_line = if 1 < value.lines().count() {
            true
        } else {
            /*
            // "'xxx'\r\n" Supports missing cases.
            // "'xxx'\r\n" ケースの取り逃しに対応。
            let ch_vec: Vec<char> = value.chars().collect();
            if let Some(_) = Stringifier::which_new_line_type(&ch_vec) {
                true
            } else {
            */
            false
            //}
        };
        if multi_line {
            // Multi-line string.
            // if let Ok(re) = RE_TRIPLE_SINGLE_QUOTE.lock() {
            // if re.is_match(value) {
            if value.contains("'''") {
                let escaped_string = if let Some(escaped_trailing_newline_string) =
                    Stringifier::escape_trailing_newline(value)
                {
                    Stringifier::escape_double_quotation(&escaped_trailing_newline_string)
                } else {
                    Stringifier::escape_double_quotation(value)
                };
                // (Result 3) Triple double quoted, Multi-line.
                return format!(
                    "\"\"\"
{}
\"\"\"",
                    escaped_string
                );
            }
            // (Result 6) Triple single quoted, Multi-line.
            format!(
                "'''
{}
'''",
                value
            )
        } else {
            // One liner.
            let escaped_trailng_newline_string = Stringifier::escape_trailing_newline(value);
            if let Some(escaped_trailng_newline_string) = escaped_trailng_newline_string {
                // (Result 1) Double quoted, Single-line.
                return format!(
                    "\"{}\"",
                    Stringifier::escape_double_quotation(&escaped_trailng_newline_string)
                );
            }
            if value.contains("'") {
                // (Result 1) Double quoted, Single-line.
                return format!(
                    "\"{}\"",
                    Stringifier::escape_double_quotation(&Stringifier::escape_back_slash(value))
                );
            }

            // (Result 4) Single quoted, Single-line.
            format!("'{}'", value)
        }
    }

    /// Escape back slash.  
    /// バック・スラッシュをエスケープします。  
    pub fn escape_back_slash(text: &str) -> String {
        text.replace("\\", "\\\\")
    }
    /// Escape double quotation.  
    /// 二重引用符をエスケープします。  
    pub fn escape_double_quotation(text: &str) -> String {
        text.replace("\"", "\\\"")
    }
    fn which_new_line_type(ch_vec: &Vec<char>) -> Option<NewLineType> {
        if NEW_LINE_CHARS.len() == 2 && 1 < ch_vec.len() {
            if ch_vec[ch_vec.len() - 2] == NEW_LINE_CHARS[0]
                && ch_vec[ch_vec.len() - 1] == NEW_LINE_CHARS[1]
            {
                // For windows.
                Some(NewLineType::Windows)
            } else {
                // No trailing new line.
                None
            }
        } else if NEW_LINE_CHARS.len() == 1 && 0 < ch_vec.len() {
            if ch_vec[ch_vec.len() - 1] == NEW_LINE_CHARS[0] {
                // TODO For linux OS.
                Some(NewLineType::NotWindows)
            } else {
                // No trailing new line.
                None
            }
        } else {
            // No trailing new line.
            None
        }
    }
    /// Escape trailing newline.  
    /// 末尾の改行をエスケープします。  
    ///
    /// # Returns
    ///
    /// Escaped string or None.  
    /// エスケープした文字列か、あるいは None です。  
    pub fn escape_trailing_newline(value: &str) -> Option<String> {
        let ch_vec: Vec<char> = value.chars().collect();
        if let Some(t) = Stringifier::which_new_line_type(&ch_vec) {
            match t {
                NewLineType::NotWindows => {
                    // For Unix OS.
                    //*
                    Some(format!("{}{}", value.trim_end(), NEW_LINE_SEQUENCE).to_string())
                    // */
                    /*
                        // Remove new line code.
                        ch_vec.pop();
                        // Append escaped new line.
                        ch_vec.push(NEW_LINE_ESCAPED_CHARS[0]);
                        ch_vec.push(NEW_LINE_ESCAPED_CHARS[1]);
                        // From vector to string.
                        ch_vec.iter().map(|&s| s as char).collect::<String>()
                    // */
                    /*
                    value.to_string()
                    // */
                }
                NewLineType::Windows => {
                    // For windows.
                    //*
                    Some(format!("{}{}", value.trim_end(), NEW_LINE_SEQUENCE).to_string())
                    // */
                    /*
                        // Remove new line code.
                        ch_vec.pop();
                        ch_vec.pop();
                        // Append escaped new line.
                        ch_vec.push(NEW_LINE_ESCAPED_CHARS[0]);
                        ch_vec.push(NEW_LINE_ESCAPED_CHARS[1]);
                        ch_vec.push(NEW_LINE_ESCAPED_CHARS[2]);
                        ch_vec.push(NEW_LINE_ESCAPED_CHARS[3]);
                        // From vector to string.
                        ch_vec.iter().map(|&s| s as char).collect::<String>()
                    // */
                    /*
                    value.to_string()
                    // */
                }
            }
        } else {
            // No trailing new line.
            None
        }
        /*
        let mut body = if value[value.len() - NEW_LINE.len()..] == *NEW_LINE {
            // Do.
            format!("{}{}", value.trim_end(), NEW_LINE_SEQUENCE)
        } else {
            // Don't.
            value.to_string()
        };
        */
    }
}
