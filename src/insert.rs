use crate::{Table, ToSql};
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
    fn write_sql(&self, sql: &mut String) {
        sql.push_str("INSERT INTO ");
        sql.push_str(T::NAME);
        sql.push('(');
        V::Item::write_columns(sql);
        sql.push(')');
        sql.push_str(" VALUES ");
        let mut values = self.values.clone().into_iter().peekable();
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
