use eosio_types::*;

use bytes::{ReadError, Readable, Writeable};
use core::marker::PhantomData;
use eosio_sys::ctypes::*;

pub trait TableRow: Readable + Writeable {
    fn primary_key(&self) -> u64;
}

#[derive(PartialEq, Copy, Clone)]
pub struct TableIter(i32);

impl From<i32> for TableIter {
    fn from(itr: i32) -> Self {
        TableIter(itr)
    }
}

pub struct Table<T>
where
    T: TableRow,
{
    code: AccountName,
    scope: AccountName,
    name: TableName,
    _row_type: PhantomData<T>,
}

impl<T> Table<T>
where
    T: TableRow,
{
    pub fn new(code: AccountName, scope: AccountName, name: TableName) -> Table<T> {
        Table {
            code,
            scope,
            name,
            _row_type: PhantomData,
        }
    }

    pub fn end(&self) -> TableIter {
        let itr = unsafe {
            ::eosio_sys::db::db_end_i64(self.code.as_u64(), self.scope.as_u64(), self.name.as_u64())
        };
        itr.into()
    }

    pub fn is_end(&self, itr: TableIter) -> bool {
        itr == self.end()
    }

    pub fn exists<Id>(&self, id: Id) -> bool
    where
        Id: Into<u64>,
    {
        let itr = self.find(id);
        !self.is_end(itr)
    }

    pub fn find<Id>(&self, id: Id) -> TableIter
    where
        Id: Into<u64>,
    {
        let itr = unsafe {
            ::eosio_sys::db::db_find_i64(
                self.code.as_u64(),
                self.scope.as_u64(),
                self.name.as_u64(),
                id.into(),
            )
        };
        itr.into()
    }

    pub fn get(&self, itr: TableIter) -> Result<T, ReadError> {
        let mut bytes = [0u8; 1000];
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_sys::db::db_get_i64(itr.0, ptr, 1000);
        }
        T::read(&bytes).map(|(t, _)| t)
    }

    pub fn emplace(&self, payer: AccountName, item: T) -> TableIter {
        let mut bytes = [0u8; 1000];
        let pos = item.write(&mut bytes).unwrap();
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        let itr = unsafe {
            ::eosio_sys::db::db_store_i64(
                self.scope.as_u64(),
                self.name.as_u64(),
                payer.as_u64(),
                item.primary_key(),
                ptr,
                pos as u32,
            )
        };
        itr.into()
    }

    pub fn modify(&self, itr: TableIter, payer: AccountName, item: T) {
        let mut bytes = [0u8; 1000];
        let pos = item.write(&mut bytes).unwrap();
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;

        unsafe { ::eosio_sys::db::db_update_i64(itr.0, payer.as_u64(), ptr, pos as u32) }
    }

    pub fn erase(&self, itr: TableIter) {
        unsafe {
            ::eosio_sys::db::db_remove_i64(itr.0);
        }
    }
}