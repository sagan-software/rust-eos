use eosio::{AccountName, ReadError, ScopeName, Table, WriteError};

/// Table Cursor
pub trait TableCursor<T>: IntoIterator
where
    T: Table,
{
    /// Read and deserialize the current table row
    fn get(&self) -> Result<T::Row, ReadError>;
    /// Erase the current row
    fn erase(&self) -> Result<T::Row, ReadError>;
    /// Modify the current row
    fn modify(
        &self,
        payer: Option<AccountName>,
        item: &T::Row,
    ) -> Result<usize, WriteError>;
}

/// Table index
pub trait TableIndex<'a, K, T>
where
    T: Table + 'a,
{
    /// The kind of cursor this table index uses
    type Cursor: TableCursor<T> + 'a;
    /// Returns the account name of the smart contract
    fn code(&'a self) -> AccountName;
    /// Returns the table scope
    fn scope(&'a self) -> ScopeName;
    /// Returns a cursor pointing to the first row that matches a key
    fn lower_bound<N: Into<K>>(&'a self, key: N) -> Option<Self::Cursor>;
    /// Returns a cursor pointing to the last row that matches a key
    fn upper_bound<N: Into<K>>(&'a self, key: N) -> Option<Self::Cursor>;
    /// Inserts a new row into the table
    fn emplace(
        &'a self,
        payer: AccountName,
        item: &'a T::Row,
    ) -> Result<(), WriteError>;
}

/// Table iterator
pub trait TableIterator: DoubleEndedIterator {}
