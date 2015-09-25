#![deny(missing_docs)]

//! A dataset library.

/// Implemented by all data sets.
pub trait DataSet {
    /// Gets the table description of the data set.
    fn tables(&self) -> &[Table];

    /// Read usize data from a column.
    fn read_usize(&self, table: &str, column: &str) -> Option<ReadData<usize>>;

    /// Read string data from a column.
    fn read_string(&self, table: &str, column: &str) -> Option<ReadData<String>>;
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

/// Reads data.
pub struct ReadData<T> {
    /// The current pointer.
    pub ptr: *const T,
    /// The number of data left.
    pub len: usize,
    /// The number of bytes to jump to next pointer.
    pub size: usize,
}

impl<T> ReadData<T> {
    /// Gets pointer at index location.
    pub fn get(&self, index: usize) -> Option<*const T> {
        if index >= self.len { None }
        else {
            Some(unsafe {
                (self.ptr as *const u8)
                    .offset((self.size * index) as isize) as *const T
            })
        }
    }
}

impl<T> Iterator for ReadData<T> {
    type Item = *const T;

    fn next(&mut self) -> Option<*const T> {
        if self.len == 0 { None }
        else {
            self.len -= 1;
            let res = self.ptr;
            self.ptr = unsafe {
                (self.ptr as *const u8)
                    .offset(self.size as isize) as *const T
            };
            Some(res)
        }
    }
}

#[macro_export]
macro_rules! has_table_impls {
    ($x:path { $($n:ident : $t:path),* }) => {

        $(
        impl HasTable<$t> for $x {
            fn raw_table(&mut self) -> *mut Vec<$t> {
                &mut self.$n as *mut _
            }

            fn get_table(&self) -> &[$t] {
                &self.$n[0..]
            }
        }
        )*

    }
}
