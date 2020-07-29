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
#[cfg(not(windows))]
const NEW_LINE_SEQUENCE: &'static str = "\\n";
const CARRIAGE_RETURN: &'static char = &'\r';
const LINE_FEED: &'static char = &'\n';
enum NewLineType {
    /// `\r\n`.
    CarriageReturnLineFeed,
    /// `\n`
    LineFeed,
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
        let multi_line = if 1 < value.lines().count() {
            // Multi-line.
            true
        } else {
            // Single line.
            false
        };
        if multi_line {
            // Multi-line string.
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
                    Stringifier::escape_double_quotation(&Stringifier::escape_back_slash(
                        &escaped_trailng_newline_string
                    ))
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
    /// Which new line type?  
    /// 改行はどれですか？  
    ///
    /// Note: Even on Windows, there may be mixed '\n' line breaks.  
    /// 注意: Windows でも、'\n' 改行が混じっていることがあります。  
    fn which_new_line_type(ch_vec: &Vec<char>) -> Option<NewLineType> {
        // If the last '\n'.
        // 最後が '\n' なら。
        if 0 < ch_vec.len() && &ch_vec[ch_vec.len() - 1] == LINE_FEED {
            // if the second from the last is '\r'.
            // 最後から二番目が '\r' なら。
            if 1 < ch_vec.len() && &ch_vec[ch_vec.len() - 2] == CARRIAGE_RETURN {
                // It's "\r\n".
                // "\r\n" です。
                return Some(NewLineType::CarriageReturnLineFeed);
            }
            // It's '\n'.
            // '\n' です。
            Some(NewLineType::LineFeed)
        } else {
            // No trailing new line.
            // 末尾に改行はありません。
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
                NewLineType::LineFeed => {
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
                NewLineType::CarriageReturnLineFeed => {
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
