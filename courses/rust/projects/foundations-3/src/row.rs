/// Represents a single CSV row with named columns.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    headers: Vec<String>,
    values: Vec<String>,
}

impl Row {
    /// Creates a new `Row` from parallel vectors of header names and values.
    pub fn new(headers: Vec<String>, values: Vec<String>) -> Self {
        todo!()
    }

    /// Returns the value for the given column name, or `None` if not found.
    pub fn get(&self, column: &str) -> Option<&str> {
        todo!()
    }

    /// Returns a slice of all header names in this row.
    pub fn headers(&self) -> &[String] {
        todo!()
    }

    /// Returns a slice of all values in this row.
    pub fn values(&self) -> &[String] {
        todo!()
    }

    /// Sets the value for the given column. Returns `true` if the column exists.
    pub fn set(&mut self, column: &str, value: String) -> bool {
        todo!()
    }

    /// Returns a new `Row` containing only the specified columns.
    pub fn select(&self, columns: &[&str]) -> Row {
        todo!()
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
