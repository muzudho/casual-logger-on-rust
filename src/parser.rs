//! Escape control characters.

use regex::Regex;
use std::sync::Mutex;

// For multi-platform. Windows, or not.
#[cfg(windows)]
const NEW_LINE_SEQUENCE: &'static str = "\\r\\n";
#[cfg(windows)]
const NEW_LINE_CHARS: &'static [char; 2] = &['\r', '\n'];
#[cfg(not(windows))]
const NEW_LINE_SEQUENCE: &'static str = "\\n";
#[cfg(not(windows))]
const NEW_LINE_CHARS: &'static [char; 1] = &['\n'];

lazy_static! {
    /// Triple single quotation.
    static ref RE_TRIPLE_SINGLE_QUOTE: Mutex<Regex> = Mutex::new(Regex::new(r"'''").unwrap());
}

/// Escape control characters.
pub struct Parser {}
impl Parser {
    /// Escape double quotation.
    pub fn escape_double_quotation(text: &str) -> String {
        text.replace("\"", "\\\"")
    }

    /// Escape back slash.
    fn escape_back_slash(text: &str) -> String {
        text.replace("\\", "\\\\")
    }

    #[deprecated(since = "0.4.1", note = "This is private method")]
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
        if 1 < value.lines().count() {
            // Multi-line string.
            // if let Ok(re) = RE_TRIPLE_SINGLE_QUOTE.lock() {
            // if re.is_match(value) {
            if value.contains("'''") {
                let escaped_string = if let Some(escaped_trailing_newline_string) =
                    Parser::escape_trailing_newline(value)
                {
                    Parser::escape_double_quotation(&escaped_trailing_newline_string)
                } else {
                    Parser::escape_double_quotation(value)
                };
                return format!(
                    "\"\"\"
{}
\"\"\"",
                    escaped_string
                );
            }
            format!(
                "'''
{}
'''",
                value
            )
        } else {
            // One liner.
            let escaped_trailng_newline_string = Parser::escape_trailing_newline(value);
            if let Some(escaped_trailng_newline_string) = escaped_trailng_newline_string {
                return format!(
                    "\"{}\"",
                    Parser::escape_double_quotation(&escaped_trailng_newline_string)
                );
            }
            if value.contains("'") {
                return format!(
                    "\"{}\"",
                    Parser::escape_double_quotation(&Parser::escape_back_slash(value))
                );
            }

            format!("'{}'", value)
        }
    }

    /// Escape trailing newline.
    ///
    /// # Returns
    ///
    /// Escaped string or None.
    fn escape_trailing_newline(value: &str) -> Option<String> {
        let ch_vec: Vec<char> = value.chars().collect();
        if NEW_LINE_CHARS.len() == 2 && 1 < ch_vec.len() {
            if ch_vec[ch_vec.len() - 2] == NEW_LINE_CHARS[0]
                && ch_vec[ch_vec.len() - 1] == NEW_LINE_CHARS[1]
            {
                // TODO For windows.
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
            } else {
                // Don't.
                None
            }
        } else if NEW_LINE_CHARS.len() == 1 && 0 < ch_vec.len() {
            if ch_vec[ch_vec.len() - 1] == NEW_LINE_CHARS[0] {
                // TODO For linux OS.
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
            } else {
                // Don't.
                None
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
