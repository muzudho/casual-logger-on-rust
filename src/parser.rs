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
    #[deprecated(since = "0.4.1", note = "This is private method")]
    pub fn format_str_value(value: &str) -> String {
        // let value = Table::convert_multi_byte_string(slice);
        // Escape the trailing newline at last.
        let ch_vec: Vec<char> = value.chars().collect();
        let mut body = if NEW_LINE_CHARS.len() == 2 && 1 < ch_vec.len() {
            if ch_vec[ch_vec.len() - 2] == NEW_LINE_CHARS[0]
                && ch_vec[ch_vec.len() - 1] == NEW_LINE_CHARS[1]
            {
                // TODO For windows.
                //*
                format!("{}{}", value.trim_end(), NEW_LINE_SEQUENCE)
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
                value.to_string()
            }
        } else if NEW_LINE_CHARS.len() == 1 && 0 < ch_vec.len() {
            if ch_vec[ch_vec.len() - 1] == NEW_LINE_CHARS[0] {
                // TODO For linux OS.
                //*
                format!("{}{}", value.trim_end(), NEW_LINE_SEQUENCE)
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
                value.to_string()
            }
        } else {
            // No trailing new line.
            value.to_string()
        };
        /*
        let mut body = if value[value.len() - NEW_LINE.len()..] == *NEW_LINE {
            // Do.
            format!("{}{}", value.trim_end(), NEW_LINE_SEQUENCE)
        } else {
            // Don't.
            value.to_string()
        };
        */
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
                body = Parser::escape(&body);
                return format!(
                    "\"\"\"
{}
\"\"\"",
                    body
                );
            }
            format!(
                "'''
{}
'''",
                body
            )
        } else {
            // One liner.
            if value.contains("'") {
                body = Parser::escape(&body);
                return format!("\"{}\"", body);
            }

            format!("'{}'", body)
        }
    }

    /// Escape string.
    pub fn escape(text: &str) -> String {
        // Escape the double quotation.
        text.replace("\"", "\\\"")
    }
}
