use super::predicate::{And, Eq, Op, Predicate};
use crate::types::{Bind, Primitive};
use crate::{CheckedSql, Table, ToSql};
use std::marker::PhantomData;

pub trait UpdateSet {
    fn write_set(&self, sql: &mut String);
}

impl<T, A> UpdateSet for Op<T, A, Bind, Eq>
where
    T: Table,
{
    fn write_set(&self, sql: &mut String) {
        self.write_predicate(sql);
    }
}

impl<T, A, U> UpdateSet for Op<T, A, U, Eq>
where
    T: Table,
    U: Primitive,
{
    fn write_set(&self, sql: &mut String) {
        self.write_predicate(sql);
    }
}

impl<H, T> UpdateSet for And<H, T>
where
    H: UpdateSet,
    T: UpdateSet,
{
    fn write_set(&self, sql: &mut String) {
        self.head.write_set(sql);
        sql.push(',');
        self.tail.write_set(sql);
    }
}

pub struct Update<T: ?Sized, S> {
    _table: PhantomData<T>,
    set: S,
}

impl<T: ?Sized, S> Update<T, S> {
    pub(crate) const fn new(set: S) -> Self {
        Self {
            _table: PhantomData,
            set,
        }
    }
}

impl<T, S> ToSql for Update<T, S>
where
    T: Table + ?Sized,
    S: UpdateSet,
{
    fn write_sql_unchecked(&self, sql: &mut String) {
        sql.push_str("UPDATE ");
        sql.push_str(T::NAME);
        sql.push_str(" SET ");
        self.set.write_set(sql);
    }
}

impl<T: ?Sized, S: CheckedSql> CheckedSql for Update<T, S> {}
