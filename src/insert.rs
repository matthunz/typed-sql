use std::marker::PhantomData;

use crate::{Sql, Table, ToSql};

pub trait Insertable {
    fn write_columns(sql: &mut Sql);

    fn write_values(&self, sql: &mut Sql);
}

impl<I: Insertable> Insertable for &I {
    fn write_columns(sql: &mut Sql) {
        I::write_columns(sql);
    }

    fn write_values(&self, sql: &mut Sql) {
        (*self).write_values(sql);
    }
}

pub trait Insert: Table {
    fn insert<V>(values: V) -> InsertStatement<Self, V>
    where
        Self: Sized,
        V: IntoIterator,
    {
        InsertStatement {
            values,
            _table: PhantomData,
        }
    }
}

impl<T: Table> Insert for T {}

pub struct InsertStatement<T, V> {
    values: V,
    _table: PhantomData<T>,
}

impl<T, V> ToSql for InsertStatement<T, V>
where
    T: Table,
    V: IntoIterator + Clone,
    V::Item: Insertable,
{
    fn write_sql(&self, sql: &mut Sql) {
        sql.buf.push_str("INSERT INTO ");
        sql.buf.push_str(T::NAME);
        sql.buf.push('(');
        V::Item::write_columns(sql);
        sql.buf.push(')');
        sql.buf.push_str(" VALUES ");
        let mut values = self.values.clone().into_iter().peekable();
        loop {
            if let Some(value) = values.next() {
                sql.buf.push('(');
                value.write_values(sql);
                sql.buf.push(')');

                if values.peek().is_some() {
                    sql.buf.push(',');
                    continue;
                }
            }
            break;
        }
    }
}
