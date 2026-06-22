use crate::Row;

/// A lazy CSV reader that parses CSV text line-by-line.
///
/// `CsvReader` implements `Iterator`, yielding one `Row` per CSV data line.
/// The first line of the input is treated as the header row. Parsing is lazy:
/// no work is done until `next()` is called.
pub struct CsvReader<'a> {
    // TODO: fields for tracking position in the CSV text
    lines: Vec<&'a str>,
    i: usize,
}

impl<'a> CsvReader<'a> {
    /// Creates a new `CsvReader` from a string of CSV data.
    ///
    /// The first line is treated as the header row. Subsequent lines are
    /// parsed lazily as the iterator is consumed.
    pub fn new(data: &'a str) -> Self {
        CsvReader {
            lines: data.lines().collect(),
            i: 0,
        }
    }

    fn header(&self) -> Vec<String> {
        if self.lines.is_empty() {
            return Vec::new();
        };

        self.lines[0]
            .split(',')
            .map(|h| h.trim().to_string())
            .collect()
    }
}

impl<'a> Iterator for CsvReader<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;

        if self.i >= self.lines.len() {
            return None;
        };

        let headers = self.header();
        let values = self.lines[self.i]
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        Some(Row::new(headers, values))
    }
}
