#![deny(missing_docs)]

//! A dataset library.

use std::any::Any;

/// Implemented by data sets for runtime reflection.
/// A data set is a collection of tables, usually `Vec<T>`.
pub trait DataSet {
    /// Gets the table descriptions of the data set.
    fn tables(&self) -> &[Table];

    /// Read data from a column.
    /// The type T is the column type.
    /// Returns a `ReadData` which points directly inside the table.
    fn read<T: Any>(&self, table: &str, column: &str) -> Option<ReadData<T>>;
}

/// Implemented by datasets that has a table for generic programming.
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
    pub column_type: &'a str,
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

/// Generates an impl of `DataSet` for a type.
#[macro_export]
macro_rules! dataset_impl {
    ($dataset:ident { $($table_name:ident : $table_type:ident { $($n:ident : $t:ident),* })* }) => {

    impl DataSet for $dataset {

        fn tables(&self) -> &[Table] {
            static TABLES: &'static [Table<'static>] = &[
                $(
                Table { name: stringify!($table_type), columns: &[
                    $(
                    Column { name: stringify!($n), column_type: stringify!($t) },
                    )*
                ] },
                )*
            ];

            TABLES
        }

        fn read<T: Any>(&self, table: &str, column: &str) -> Option<ReadData<T>> {
            use std::mem::{ size_of, transmute };
            use std::ptr::null;

            match (table, column) {
                $($(
                (stringify!($table_type), stringify!($n)) => {
                    if TypeId::of::<T>() == TypeId::of::<$t>() {
                        if self.$table_name.len() == 0 {
                            Some(ReadData {
                                ptr: null(),
                                len: 0,
                                size: 0,
                            })
                        } else {
                            Some(unsafe {transmute(ReadData {
                                ptr: &self.$table_name[0].$n,
                                len: self.$table_name.len(),
                                size: size_of::<$table_type>()
                            })})
                        }
                    } else {
                        None
                    }
                }
                )*)*
                _ => None
            }
        }

    }

    }
}
