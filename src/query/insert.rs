use super::Select;
use crate::{CheckedSql, Table, ToSql};
use std::marker::PhantomData;

pub trait Insertable {
    fn write_columns(sql: &mut String);

    fn write_values(&self, sql: &mut String);
}

impl<I: Insertable> Insertable for &I {
    fn write_columns(sql: &mut String) {
        I::write_columns(sql);
    }

    fn write_values(&self, sql: &mut String) {
        (*self).write_values(sql);
    }
}

pub struct Values<I> {
    iter: I,
}

impl<I> Values<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

pub struct InsertSelect<S, I: ?Sized> {
    select: S,
    _insertable: PhantomData<I>,
}

impl<S, I: ?Sized> InsertSelect<S, I> {
    pub(crate) fn new(select: S) -> Self {
        Self {
            select,
            _insertable: PhantomData,
        }
    }
}

impl<S, I> Insertable for InsertSelect<S, I>
where
    S: Select,
    I: Insertable + ?Sized,
{
    fn write_columns(sql: &mut String) {
        I::write_columns(sql);
    }

    fn write_values(&self, sql: &mut String) {
        self.select.write_sql_unchecked(sql);
    }
}

pub struct InsertStatement<T: ?Sized, V> {
    values: V,
    _table: PhantomData<T>,
}

impl<T, I> InsertStatement<T, I>
where
    T: Table + ?Sized,
{
    pub(crate) fn new(values: I) -> Self {
        Self {
            values,
            _table: PhantomData,
        }
    }

    fn write_insert<I2: Insertable>(&self, sql: &mut String) {
        sql.push_str("INSERT INTO ");
        sql.push_str(T::NAME);
        sql.push('(');
        I2::write_columns(sql);
        sql.push(')');
        sql.push_str(" VALUES ");
    }
}

impl<T, I> ToSql for InsertStatement<T, I>
where
    T: Table + ?Sized,
    I: Insertable,
{
    fn write_sql_unchecked(&self, sql: &mut String) {
        self.write_insert::<I>(sql);

        sql.push('(');
        self.values.write_values(sql);
        sql.push(')');
    }
}

impl<T, I> ToSql for InsertStatement<T, Values<I>>
where
    T: Table,
    I: IntoIterator + Clone,
    I::Item: Insertable,
{
    fn write_sql_unchecked(&self, sql: &mut String) {
        self.write_insert::<I::Item>(sql);

        let mut values = self.values.iter.clone().into_iter().peekable();
        loop {
            if let Some(value) = values.next() {
                sql.push('(');
                value.write_values(sql);
                sql.push(')');

                if values.peek().is_some() {
                    sql.push(',');
                    continue;
                }
            }
            break;
        }
    }
}

impl<T, I: CheckedSql> CheckedSql for InsertSelect<T, I> {}
