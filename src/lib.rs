#![deny(missing_docs)]

//! A dataset library.

/// Implemented by all data sets.
pub trait DataSet {
    /// Gets the table description of the data set.
    fn tables(&self) -> &[Table];
}

/// Implemented by datasets that has a table.
pub trait HasTable<T>: DataSet {
    /// Get access to the full table.
    /// Uses a raw pointer to access multiple tables at the same time.
    fn raw_table(&mut self) -> *mut Vec<T>;

    /// Gets an immutable view into table.
    fn get_table(&self) -> &[T];

    /// Adds a value.
    fn add(&mut self, val: T) -> usize {
        let mut table: &mut Vec<T> = unsafe { &mut *self.raw_table() };
        table.push(val);
        table.len() - 1
    }
}

/// Contains table information.
pub struct Table<'a> {
    /// The name of table.
    pub name: &'a str,
    /// The columns.
    pub columns: &'a [Column<'a>],
}

/// Contains column information.
pub struct Column<'a> {
    /// The name of column.
    pub name: &'a str,
    /// The type of column.
    pub column_type: ColumnType,
}

/// The type of column.
pub enum ColumnType {
    /// A pointer sized integer.
    Usize,
    /// The column is a string.
    String,
}
