use std::fmt::write;

/// Represents a single CSV row with named columns.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    headers: Vec<String>,
    values: Vec<String>,
}

impl Row {
    /// Creates a new `Row` from parallel vectors of header names and values.
    pub fn new(headers: Vec<String>, values: Vec<String>) -> Self {
        Self { headers, values }
    }

    /// Returns the value for the given column name, or `None` if not found.
    pub fn get(&self, column: &str) -> Option<&str> {
        if let Some(pos) = self.headers.iter().position(|c| c == column) {
            self.values.get(pos).map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Returns a slice of all header names in this row.
    pub fn headers(&self) -> &[String] {
        self.headers.as_ref()
    }

    /// Returns a slice of all values in this row.
    pub fn values(&self) -> &[String] {
        self.values.as_ref()
    }

    /// Sets the value for the given column. Returns `true` if the column exists.
    pub fn set(&mut self, column: &str, value: String) -> bool {
        if let Some(pos) = self.headers.iter().position(|c| c == column) {
            if let Some(val) = self.values.get_mut(pos) {
                *val = value;
                return true;
            } else {
                self.values.insert(pos, value);
            }
        } else {
            // assuming they have consistently equal length
            self.headers.push(column.to_string());
            self.values.push(value);
        }

        false
    }

    /// Returns a new `Row` containing only the specified columns.
    pub fn select(&self, columns: &[&str]) -> Row {
        let mut values = Vec::new();
        columns.iter().for_each(|col| {
            if let Some(pos) = self.headers.iter().position(|c| &c == col) {
                values.push(self.values[pos].clone());
            } else {
                values.push("".to_string());
            }
        });

        Row {
            headers: columns.iter().map(|c| c.to_string()).collect(),
            values,
        }
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(
            f,
            format_args!("Headers: {:#?}\nValues: {:#?}\n", self.headers, self.values),
        )
    }
}
