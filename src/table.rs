use crate::stringifier::Stringifier;
use crate::Level;
use crate::Logger;
use crate::Opt;
use crate::Table;
use crate::LOGGER;
use crate::NEW_LINE;
use regex::Regex;
use std::collections::BTreeMap;
use std::sync::Mutex;

lazy_static! {
    /// Without dot.
    static ref RE_TOML_KEY: Mutex<Regex> = Mutex::new(Regex::new(r"^[A-Za-z0-9_-]+$").unwrap());
    static ref RE_WHITE_SPACE: Mutex<Regex> = Mutex::new(Regex::new(r"\s").unwrap());
}

#[derive(Clone)]
pub struct InternalTable {
    /// Base name.
    /// `B` in `[A.B]`.
    /// `a=1&b=2` in `["a=1&b=2"]`.
    pub base_name: String,
    /// Clone.
    pub table: Table,
}
impl InternalTable {
    pub fn new(base_name: &str, table: &Table) -> Self {
        InternalTable {
            base_name: base_name.to_string(),
            table: table.clone(),
        }
    }
    pub fn stringify(&self) -> String {
        // Write as TOML.
        // Table name.
        let mut toml = format!(
            "[{}]
",
            self.base_name
        );
        // Log level message.
        let message = if self.table.message_trailing_newline {
            // There is a trailing newline.
            format!("{}{}", self.table.message, NEW_LINE)
        } else {
            self.table.message.to_string()
        };
        toml.push_str(&format!(
            "{} = {}
",
            self.table.level,
            Stringifier::format_str_value(&message)
        ));
        // Sorted map.
        if let Some(sorted_map) = &self.table.sorted_map {
            for (k, formatted_v) in sorted_map {
                toml.push_str(&format!(
                    "{} = {}
",
                    k, formatted_v
                ));
            }
        }
        // Sub tables.
        // TODO Recursive.
        let mut indent_spaces = String::new();
        if let Some(sub_tables) = &self.table.sub_tables {
            indent_spaces.push_str("  ");
            for (_k1, i_table) in sub_tables {
                InternalTable::stringify_sub_table(
                    &mut toml,
                    &mut indent_spaces,
                    &self.base_name,
                    i_table,
                );
            }
            indent_spaces.pop();
            indent_spaces.pop();
        }
        // New line.
        toml.push_str(
            "
",
        );
        toml
    }

    pub fn stringify_sub_table(
        toml: &mut String,
        indent_spaces: &mut String,
        path: &str,
        i_table: &InternalTable,
    ) {
        toml.push_str(&indent_spaces);
        toml.push_str(&format!(
            "[{}.{}]
",
            path, i_table.base_name
        ));
        // Sorted map.
        if let Some(sorted_map) = &i_table.table.sorted_map {
            for (k2, formatted_v) in sorted_map {
                toml.push_str(&indent_spaces);
                toml.push_str(&format!(
                    "{} = {}
",
                    k2, formatted_v
                ));
            }
        }
        // Sub tables.
        if let Some(sub_tables) = &i_table.table.sub_tables {
            indent_spaces.push_str("  ");
            for (_k1, i_table) in sub_tables {
                InternalTable::stringify_sub_table(toml, indent_spaces, path, i_table);
            }
            indent_spaces.pop();
            indent_spaces.pop();
        }
    }
}
impl Default for Table {
    fn default() -> Self {
        Table {
            // The base name is added when writing the log.
            // ログを書くときにベース名が付きます。
            base_name: "".to_string(),
            level: Level::Trace,
            message: "".to_string(),
            message_trailing_newline: false,
            sorted_map: None,
            sub_tables: None,
        }
    }
}
impl Table {
    /*
    pub fn convert_multi_byte_string(value: &str) -> String {
        let bytes: &[u8] = value.as_bytes();
        // convert bytes => str
        // let res = bytes.iter().map(|&s| s as char).collect::<String>();
        let converted: String = if let Ok(converted) = String::from_utf8(bytes.to_vec()) {
            converted
        } else {
            value.to_string()
        };
        println!(
            "Value=|{}|{}| Converted=|{}|{}|",
            value,
            value.len(),
            converted,
            converted.len()
        );
        converted
    }
    */
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
    fn correct_key(key: &str) -> String {
        if let Ok(logger) = LOGGER.lock() {
            match Logger::get_optimization(&logger) {
                Opt::Release => {
                    return key.to_string();
                }
                _ => {}
            }
        };

        // Check
        // TODO Dotted key support is difficult.
        if let Ok(re_toml_key) = RE_TOML_KEY.lock() {
            if re_toml_key.is_match(key) {
                // Ok.
                return key.to_string();
            }
        }

        // TODO Auto correct
        if let Ok(re_white_space) = RE_WHITE_SPACE.lock() {
            format!(
                "\"{}\"",
                Stringifier::escape_double_quotation(&re_white_space.replace_all(key, " "))
            )
        } else {
            // TODO Error
            key.to_string()
        }
    }
    /// Insert literal string value. Do not put in quotes.  
    /// リテラル文字列を挿入します。引用符で挟みません。  
    ///
    /// # Arguments
    ///
    /// * `key` - A key.  
    ///             キー。  
    /// * `value` - A value.  
    ///             値。  
    ///
    /// # Returns
    ///
    /// Table.  
    /// テーブル。  
    pub fn literal<'a>(&'a mut self, key: &'a str, value: &str) -> &'a mut Self {
        self.get_sorted_map(|sorted_map| {
            sorted_map.insert(
                // Log detail level.
                Table::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        self
    }
    /// Insert string value.  
    /// 文字列を挿入します。  
    ///
    /// # Arguments
    ///
    /// * `key` - A key.  
    ///             キー。  
    /// * `value` - A value.  
    ///             値。  
    ///
    /// # Returns
    ///
    /// Table.  
    /// テーブル。  
    pub fn str<'a>(&'a mut self, key: &'a str, value: &str) -> &'a mut Self {
        self.get_sorted_map(|sorted_map| {
            sorted_map.insert(
                // Log detail level.
                Table::correct_key(key),
                // Message.
                Stringifier::format_str_value(value).to_string(),
            );
        });

        self
    }
    /// Insert character value.  
    /// 文字を挿入します。  
    ///
    /// # Arguments
    ///
    /// * `key` - A key.  
    ///             キー。  
    /// * `value` - A value.  
    ///             値。  
    ///
    /// # Returns
    ///
    /// Table.  
    /// テーブル。  
    pub fn char<'a>(&'a mut self, key: &'a str, value: char) -> &'a mut Self {
        self.get_sorted_map(|sorted_map| {
            sorted_map.insert(
                // Log detail level.
                Table::correct_key(key),
                // Message.
                Stringifier::format_str_value(&value.to_string()).to_string(),
            );
        });

        self
    }
    /// Insert integer value.  
    /// 符号付き整数を挿入します。  
    ///
    /// # Arguments
    ///
    /// * `key` - A key.  
    ///             キー。  
    /// * `value` - A value.  
    ///             値。  
    ///
    /// # Returns
    ///
    /// Table.  
    /// テーブル。  
    pub fn int<'a>(&'a mut self, key: &'a str, value: i128) -> &'a mut Self {
        self.get_sorted_map(|sorted_map| {
            sorted_map.insert(
                // Log detail level.
                Table::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        self
    }
    /// Insert unsigned integer value.  
    /// 符号無し整数を挿入します。  
    ///
    /// # Arguments
    ///
    /// * `key` - A key.  
    ///             キー。  
    /// * `value` - A value.  
    ///             値。  
    ///
    /// # Returns
    ///
    /// Table.  
    /// テーブル。  
    pub fn uint<'a>(&'a mut self, key: &'a str, value: u128) -> &'a mut Self {
        self.get_sorted_map(|sorted_map| {
            sorted_map.insert(
                // Log detail level.
                Table::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        self
    }
    /// Insert float value.  
    /// 浮動小数点数を挿入します。  
    ///
    /// # Arguments
    ///
    /// * `key` - A key.  
    ///             キー。  
    /// * `value` - A value.  
    ///             値。  
    ///
    /// # Returns
    ///
    /// Table.  
    /// テーブル。  
    pub fn float<'a>(&'a mut self, key: &'a str, value: f64) -> &'a mut Self {
        self.get_sorted_map(|sorted_map| {
            sorted_map.insert(
                // Log detail level.
                Table::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        self
    }
    /// Insert boolean value.  
    /// 真理値を挿入します。  
    ///
    /// # Arguments
    ///
    /// * `key` - A key.  
    ///             キー。  
    /// * `value` - A value.  
    ///             値。  
    ///
    /// # Returns
    ///
    /// Table.  
    /// テーブル。  
    pub fn bool<'a>(&'a mut self, key: &'a str, value: bool) -> &'a mut Self {
        self.get_sorted_map(|sorted_map| {
            sorted_map.insert(
                Table::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        self
    }
    /// TODO WIP.
    pub fn sub_t<'a>(&'a mut self, base_name: &str, table: &Table) -> &'a mut Self {
        self.get_sub_tables(|sub_i_tables| {
            sub_i_tables.insert(
                // Base name.
                Table::correct_key(base_name),
                // Message.
                InternalTable::new(&Table::correct_key(base_name), &table),
            );
        });

        self
    }
}
