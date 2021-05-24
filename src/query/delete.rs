use crate::table::Table;
use crate::ToSql;
use std::marker::PhantomData;

pub struct Delete<T: ?Sized> {
    _table: PhantomData<T>,
}

impl<T: ?Sized> Delete<T> {
    pub(crate) const fn new() -> Self {
        Self {
            _table: PhantomData,
        }
    }
}

impl<T: Table + ?Sized> ToSql for Delete<T> {
    fn write_sql_unchecked(&self, sql: &mut String) {
        sql.push_str("DELETE FROM ");
        sql.push_str(T::NAME);
    }
}

impl<T: ?Sized> Clone for Delete<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Delete<T> {}
