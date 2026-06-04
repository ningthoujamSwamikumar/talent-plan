use crate::Row;

/// A lazy CSV reader that parses CSV text line-by-line.
///
/// `CsvReader` implements `Iterator`, yielding one `Row` per CSV data line.
/// The first line of the input is treated as the header row. Parsing is lazy:
/// no work is done until `next()` is called.
pub struct CsvReader<'a> {
    // TODO: fields for tracking position in the CSV text
    _data: std::marker::PhantomData<&'a str>,
}

impl<'a> CsvReader<'a> {
    /// Creates a new `CsvReader` from a string of CSV data.
    ///
    /// The first line is treated as the header row. Subsequent lines are
    /// parsed lazily as the iterator is consumed.
    pub fn new(data: &'a str) -> Self {
        todo!()
    }
}

impl<'a> Iterator for CsvReader<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
