#![deny(missing_docs)]

//! A dataset library.

use std::any::Any;

/// Implemented by all data sets.
pub unsafe trait DataSet {
    /// Get access to the full table.
    /// Uses a raw pointer to access multiple tables at the same time.
    fn raw_table<T: Any>(&mut self) -> Option<*mut Vec<T>>;

    /// Gets an immutable view into table.
    fn get_table<T: Any>(&self) -> Option<&[T]>;
}
