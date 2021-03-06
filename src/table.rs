use crate::stringifier::Stringifier;
use crate::toml::auto_correct::AutoCorrect;
use crate::{ArrayOfTable, Level, Log, Opt, Table, NEW_LINE};

/// Kind of table.  
/// テーブルの種類。  
#[derive(Clone)]
pub enum KindOfTable {
    /// Sub table.  
    /// ただのサブ・テーブル。
    Table(Table),
    /// Array of table.  
    /// テーブルの配列。
    ArrayOfTable(ArrayOfTable),
}

#[derive(Clone)]
pub struct InternalTable {
    /// Base name.
    /// `B` in `[A.B]`.
    /// `a=1&b=2` in `["a=1&b=2"]`.
    pub base_name: String,
    /// Clone table.
    pub table: KindOfTable,
}
impl InternalTable {
    pub fn from_table(table: &Table) -> Self {
        InternalTable {
            base_name: table.base_name.to_string(),
            table: KindOfTable::Table(table.clone()),
        }
    }
    pub fn from_sub_table(name: &str, sub_table: &Table) -> Self {
        InternalTable {
            base_name: name.to_string(),
            table: KindOfTable::Table(sub_table.clone()),
        }
    }
    pub fn from_aot(name: &str, aot: &ArrayOfTable) -> Self {
        InternalTable {
            base_name: name.to_string(),
            table: KindOfTable::ArrayOfTable(aot.clone()),
        }
    }
    /// Example: `Info = "Message"`.
    pub fn create_log_level_kv_pair(table: &Table) -> String {
        let message = if table.message_trailing_newline {
            // There is a trailing newline.
            format!("{}{}", table.message, NEW_LINE)
        } else {
            table.message.to_string()
        };
        format!(
            "{} = {}
",
            table.level,
            Stringifier::format_str_value(&message)
        )
    }
    pub fn stringify(&self) -> String {
        let toml = &mut String::new();
        let indent_spaces = &mut String::new();
        // Write as TOML.
        // Recursive.
        InternalTable::stringify_sub_table(
            toml,
            indent_spaces,
            None,
            match &self.table {
                KindOfTable::Table(table) => Some(InternalTable::create_log_level_kv_pair(&table)),
                KindOfTable::ArrayOfTable(_) => None,
            },
            &self,
        );
        // End of recursive.
        // New line.
        toml.push_str(
            "
",
        );
        toml.to_string()
    }

    pub fn stringify_sub_table(
        toml: &mut String,
        indent_spaces: &mut String,
        parent: Option<&str>,
        log_level_kv_pair: Option<String>,
        i_table: &InternalTable,
    ) {
        // Table name.
        let path = &format!(
            "{}{}",
            if let Some(parent) = parent {
                format!("{}.", parent).to_string()
            } else {
                "".to_string()
            },
            i_table.base_name
        );
        // Table or Array of table.
        match &i_table.table {
            KindOfTable::Table(k_table) => {
                toml.push_str(&indent_spaces);
                toml.push_str(&format!(
                    "[{}]
",
                    path
                ));
                // Log level message.
                if let Some(log_level_kv_pair) = log_level_kv_pair {
                    toml.push_str(&log_level_kv_pair);
                }
                // Sorted map.
                if let Some(sorted_map) = &k_table.sorted_map {
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
                if let Some(sub_tables) = &k_table.sub_tables {
                    indent_spaces.push_str("  ");
                    for (_k1, sub_i_table) in sub_tables {
                        InternalTable::stringify_sub_table(
                            toml,
                            indent_spaces,
                            Some(path),
                            None,
                            sub_i_table,
                        );
                    }
                    indent_spaces.pop();
                    indent_spaces.pop();
                }
            }
            KindOfTable::ArrayOfTable(k_aot) => {
                for (i, sibling_table) in k_aot.tables.iter().enumerate() {
                    // Table header.
                    toml.push_str(&indent_spaces);
                    toml.push_str(&format!(
                        "[[{}]]
",
                        path
                    ));
                    // Sorted map.
                    if let Some(sorted_map) = &sibling_table.sorted_map {
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
                    if let Some(sub_tables) = &sibling_table.sub_tables {
                        indent_spaces.push_str("  ");
                        for (_k1, sub_i_table) in sub_tables {
                            InternalTable::stringify_sub_table(
                                toml,
                                indent_spaces,
                                Some(&format!("{}.{}", path, i)),
                                None,
                                sub_i_table,
                            );
                        }
                        indent_spaces.pop();
                        indent_spaces.pop();
                    }
                }
            }
        }
    }
}

impl Default for ArrayOfTable {
    fn default() -> Self {
        ArrayOfTable { tables: Vec::new() }
    }
}
impl ArrayOfTable {
    /// Push a table.  
    /// テーブルを追加します。
    pub fn table(&mut self, table: &Table) -> &mut Self {
        self.tables.push(table.clone());
        self
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
        let mut old = None;
        self.get_sorted_map(|sorted_map| {
            old = sorted_map.insert(
                AutoCorrect::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        if let Some(old) = old {
            Table::print_already_use(key, &old, &value.to_string());
        }

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
        let mut old = None;
        self.get_sorted_map(|sorted_map| {
            old = sorted_map.insert(
                // Log detail level.
                AutoCorrect::correct_key(key),
                // Message.
                Stringifier::format_str_value(&value.to_string()).to_string(),
            );
        });

        if let Some(old) = old {
            Table::print_already_use(key, &old, &value.to_string());
        }

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
        let mut old = None;
        self.get_sorted_map(|sorted_map| {
            old = sorted_map.insert(
                // Log detail level.
                AutoCorrect::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        if let Some(old) = old {
            Table::print_already_use(key, &old, &value.to_string());
        }

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
        let mut old = None;
        self.get_sorted_map(|sorted_map| {
            old = sorted_map.insert(
                // Log detail level.
                AutoCorrect::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        if let Some(old) = old {
            Table::print_already_use(key, &old, &value.to_string());
        }

        self
    }
    /// Insert pointer size integer value.  
    /// 符号付きポインター・サイズ整数を挿入します。  
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
    pub fn isize<'a>(&'a mut self, key: &'a str, value: isize) -> &'a mut Self {
        let mut old = None;
        self.get_sorted_map(|sorted_map| {
            old = sorted_map.insert(
                // Log detail level.
                AutoCorrect::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        if let Some(old) = old {
            Table::print_already_use(key, &old, &value.to_string());
        }

        self
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
        let mut old = None;
        self.get_sorted_map(|sorted_map| {
            old = sorted_map.insert(
                // Log detail level.
                AutoCorrect::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        if let Some(old) = old {
            Table::print_already_use(key, &old, &value.to_string());
        }

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
        let mut old = None;
        self.get_sorted_map(|sorted_map| {
            old = sorted_map.insert(
                // Log detail level.
                AutoCorrect::correct_key(key),
                // Message.
                Stringifier::format_str_value(value).to_string(),
            );
        });

        if let Some(old) = old {
            Table::print_already_use(key, &old, &value.to_string());
        }

        self
    }
    /// Insert table recursively.  
    /// テーブルを再帰的に挿入します。  
    ///
    /// # Arguments
    ///
    /// * `base_name` - Sub table name.  
    ///                 サブ・テーブル名。  
    /// * `table` - Sub table.  
    ///             サブ・テーブル。  
    ///
    /// # Returns
    ///
    /// Main table.  
    /// メインの方のテーブル。  
    pub fn sub_t<'a>(&'a mut self, base_name: &str, sub_table: &Table) -> &'a mut Self {
        let mut old = None;
        self.get_sub_tables(|sub_i_tables| {
            old = sub_i_tables.insert(
                // Base name.
                AutoCorrect::correct_key(base_name),
                // Message.
                InternalTable::from_sub_table(&AutoCorrect::correct_key(base_name), &sub_table),
            );
        });

        if let Some(_) = old {
            Table::print_already_use(
                &AutoCorrect::correct_key(base_name),
                &"...Omitted...",
                &"...Omitted...",
            );
        }

        self
    }
    /// Insert array of table recursively.  
    /// テーブルの配列を再帰的に挿入します。  
    ///
    /// # Arguments
    ///
    /// * `base_name` - Array of table name.  
    ///                 テーブルの配列名。  
    /// * `table` - Array of table.  
    ///             テーブルの配列。  
    ///
    /// # Returns
    ///
    /// Main table.  
    /// メインの方のテーブル。  
    pub fn sub_aot<'a>(&'a mut self, base_name: &str, aot: &ArrayOfTable) -> &'a mut Self {
        let mut old = None;
        self.get_sub_tables(|sub_i_tables| {
            old = sub_i_tables.insert(
                // Base name.
                AutoCorrect::correct_key(base_name),
                // Message.
                InternalTable::from_aot(&AutoCorrect::correct_key(base_name), &aot),
            );
        });

        if let Some(_) = old {
            Table::print_already_use(
                &AutoCorrect::correct_key(base_name),
                &"...Omitted...",
                &"...Omitted...",
            );
        }

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
        let mut old = None;
        self.get_sorted_map(|sorted_map| {
            old = sorted_map.insert(
                // Log detail level.
                AutoCorrect::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        if let Some(old) = old {
            Table::print_already_use(key, &old, &value.to_string());
        }

        self
    }
    /// Insert unsigned pointer size integer value.  
    /// 符号無しポインター・サイズ整数を挿入します。  
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
    pub fn usize<'a>(&'a mut self, key: &'a str, value: usize) -> &'a mut Self {
        let mut old = None;
        self.get_sorted_map(|sorted_map| {
            old = sorted_map.insert(
                // Log detail level.
                AutoCorrect::correct_key(key),
                // Message.
                value.to_string(),
            );
        });

        if let Some(old) = old {
            Table::print_already_use(key, &old, &value.to_string());
        }

        self
    }
    /// Key duplicate message.
    /// キーの重複メッセージ。
    fn print_already_use(key: &str, old: &str, value: &str) {
        if let Ok(opt) = Log::get_opt() {
            match opt {
                Opt::BeginnersSupport | Opt::Development => {
                    println!(
                        "casual_logger   | |{}| is already use. |{}| is is overwritten by |{}|.",
                        key, old, value
                    );
                }
                _ => {} // Ignored it.
            }
        }
    }
}
