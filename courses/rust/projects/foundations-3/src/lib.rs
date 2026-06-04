#![deny(missing_docs)]
//! A lazy CSV query engine demonstrating Rust's iterator system.

mod query;
mod reader;
mod row;

pub use query::Query;
pub use reader::CsvReader;
pub use row::Row;
