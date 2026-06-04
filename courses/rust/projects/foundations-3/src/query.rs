use crate::Row;

/// A lazy query builder that chains operations on rows.
///
/// `Query` wraps any `Iterator<Item = Row>` and provides chainable methods
/// for filtering, mapping, selecting, and aggregating CSV data. All
/// non-terminal operations are lazy: they build up a pipeline that is only
/// executed when a consuming (terminal) method is called.
pub struct Query<I: Iterator<Item = Row>> {
    iter: I,
}

impl<I: Iterator<Item = Row>> Query<I> {
    /// Creates a new `Query` wrapping the given row iterator.
    pub fn new(iter: I) -> Self {
        todo!()
    }

    /// Lazily filters rows by a predicate closure.
    ///
    /// Only rows for which `predicate` returns `true` are yielded.
    pub fn filter_rows<F>(self, predicate: F) -> Query<impl Iterator<Item = Row>>
    where
        F: FnMut(&Row) -> bool,
    {
        let _ = predicate;
        todo!();
        // Unreachable, but gives the compiler a concrete iterator type.
        #[allow(unreachable_code)]
        Query { iter: self.iter.filter(|_: &Row| true) }
    }

    /// Lazily transforms the value of a single column using `transform`.
    ///
    /// For each row, the value in `column` is replaced with the result of
    /// calling `transform` on the old value. Rows that do not contain the
    /// column are passed through unchanged.
    pub fn map_column<F>(self, column: &str, transform: F) -> Query<impl Iterator<Item = Row>>
    where
        F: FnMut(&str) -> String,
    {
        let _ = (column, transform);
        todo!();
        #[allow(unreachable_code)]
        Query { iter: self.iter.map(|row: Row| row) }
    }

    /// Lazily projects only the specified columns from each row.
    pub fn select(self, columns: &[&str]) -> Query<impl Iterator<Item = Row>> {
        let _ = columns;
        todo!();
        #[allow(unreachable_code)]
        Query { iter: self.iter.map(|row: Row| row) }
    }

    // ----- Consuming (terminal) methods -----

    /// Consumes the iterator and returns the number of rows.
    pub fn count(self) -> usize {
        todo!()
    }

    /// Consumes the iterator and returns the sum of the given numeric column.
    ///
    /// Non-parseable values are treated as `0.0`.
    pub fn sum(self, column: &str) -> f64 {
        todo!()
    }

    /// Consumes the iterator and returns the average of the given numeric column.
    ///
    /// Returns `0.0` if the iterator is empty. Non-parseable values are treated as `0.0`.
    pub fn avg(self, column: &str) -> f64 {
        todo!()
    }

    /// Consumes the iterator and returns the minimum value in the given numeric column.
    ///
    /// Returns `None` if the iterator is empty. Non-parseable values are skipped.
    pub fn min(self, column: &str) -> Option<f64> {
        todo!()
    }

    /// Consumes the iterator and returns the maximum value in the given numeric column.
    ///
    /// Returns `None` if the iterator is empty. Non-parseable values are skipped.
    pub fn max(self, column: &str) -> Option<f64> {
        todo!()
    }

    /// Consumes the iterator and collects all rows into a `Vec<Row>`.
    pub fn collect_rows(self) -> Vec<Row> {
        todo!()
    }
}

/// Enable collecting `Row` iterators back into a CSV-formatted `String`.
///
/// The resulting string includes the header row followed by one line per data
/// row, with values separated by commas.
impl FromIterator<Row> for String {
    fn from_iter<T: IntoIterator<Item = Row>>(iter: T) -> Self {
        todo!()
    }
}
