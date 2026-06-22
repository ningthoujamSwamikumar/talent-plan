use std::collections::HashSet;

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
        Self { iter }
    }

    /// Lazily filters rows by a predicate closure.
    ///
    /// Only rows for which `predicate` returns `true` are yielded.
    pub fn filter_rows<F>(self, predicate: F) -> Query<impl Iterator<Item = Row>>
    where
        F: FnMut(&Row) -> bool,
    {
        let iter = self.iter.filter(predicate);
        Query { iter }
    }

    /// Lazily transforms the value of a single column using `transform`.
    ///
    /// For each row, the value in `column` is replaced with the result of
    /// calling `transform` on the old value. Rows that do not contain the
    /// column are passed through unchanged.
    pub fn map_column<'a, F>(
        self,
        column: &'a str,
        mut transform: F,
    ) -> Query<impl Iterator<Item = Row> + 'a>
    where
        F: FnMut(&str) -> String + 'a,
        I: 'a,
    {
        let iter = self.iter.map(move |mut r| {
            if let Some(col_pos) = r.headers().iter().position(|h| h == column) {
                let new_value = transform(r.values()[col_pos].as_str());
                r.set(column, new_value);
            };
            r
        });

        Query { iter }
    }

    /// Alternate implementation of the above method
    /// without lifetime annotations
    ///
    pub fn map_column_alternative<F>(
        self,
        column: &str,
        mut transform: F,
    ) -> Query<impl Iterator<Item = Row>>
    where
        F: FnMut(&str) -> String,
    {
        let owned_column = column.to_string();

        let iter = self.iter.map(move |mut r| {
            if let Some(col_pos) = r.headers().iter().position(|h| h == &owned_column) {
                let new_value = transform(r.values()[col_pos].as_str());
                r.set(&owned_column, new_value);
            };
            r
        });

        Query { iter }
    }

    /// Lazily projects only the specified columns from each row.
    pub fn select(self, columns: &[&str]) -> Query<impl Iterator<Item = Row>> {
        let column_set = columns
            .into_iter()
            // this prevents the need to give lifetime annotation, when captured by closure
            .map(|&c| c.to_string())
            .collect::<HashSet<_>>();

        Query {
            iter: self.iter.map(move |row: Row| {
                let (selected_headers, selected_values): (Vec<String>, Vec<String>) = row
                    .headers()
                    .iter()
                    .zip(row.values())
                    .filter(|(header, _value)| column_set.contains(header.as_str()))
                    .map(|(header, value)| (header.to_owned(), value.to_owned()))
                    .unzip();

                Row::new(selected_headers, selected_values)
            }),
        }
    }

    // ----- Consuming (terminal) methods -----

    /// Consumes the iterator and returns the number of rows.
    pub fn count(self) -> usize {
        self.iter.count()
    }

    /// Consumes the iterator and returns the sum of the given numeric column.
    ///
    /// Non-parseable values are treated as `0.0`.
    pub fn sum(self, column: &str) -> f64 {
        let sum: f64 = self
            .iter
            .map(|row| {
                row.get(column)
                    .map(|v| v.parse::<f64>().unwrap_or(0.0))
                    .unwrap_or(0.0)
            })
            .sum();

        // handle nan
        if sum.is_nan() {
            0.0
        } else {
            sum
        }
    }

    /// Consumes the iterator and returns the average of the given numeric column.
    ///
    /// Returns `0.0` if the iterator is empty. Non-parseable values are treated as `0.0`.
    pub fn avg(self, column: &str) -> f64 {
        let mut count = 0;
        let sum: f64 = self
            .iter
            .map(|row| {
                count += 1;
                row.get(column)
                    .map(|v| v.parse::<f64>().unwrap_or(0.0))
                    .unwrap_or(0.0)
            })
            .sum();

        // handle nan
        if sum.is_nan() || count == 0 {
            0.0
        } else {
            sum / count as f64
        }
    }

    /// Consumes the iterator and returns the minimum value in the given numeric column.
    ///
    /// Returns `None` if the iterator is empty. Non-parseable values are skipped.
    pub fn min(self, column: &str) -> Option<f64> {
        self.iter
            .filter_map(|row| row.get(column).map(|v| v.parse::<f64>().ok()))
            .flat_map(|v| v)
            .filter(|v| !v.is_nan())
            .reduce(f64::min)
    }

    /// Consumes the iterator and returns the maximum value in the given numeric column.
    ///
    /// Returns `None` if the iterator is empty. Non-parseable values are skipped.
    pub fn max(self, column: &str) -> Option<f64> {
        self.iter
            .filter_map(|row| row.get(column).map(|v| v.parse::<f64>().ok()))
            .flat_map(|v| v)
            .filter(|v| !v.is_nan())
            .reduce(f64::max)
    }

    /// Consumes the iterator and collects all rows into a `Vec<Row>`.
    pub fn collect_rows(self) -> Vec<Row> {
        self.iter.collect()
    }
}

/// Enable collecting `Row` iterators back into a CSV-formatted `String`.
///
/// The resulting string includes the header row followed by one line per data
/// row, with values separated by commas.
impl FromIterator<Row> for String {
    fn from_iter<T: IntoIterator<Item = Row>>(iter: T) -> Self {
        iter.into_iter().fold(String::new(), |mut acc, r| {
            if acc.is_empty() {
                acc.push_str(format!("{}", r.headers().join(",")).as_str());
            };

            acc.push_str(format!("\n{}", r.values().join(",")).as_str());

            acc
        })
    }
}
