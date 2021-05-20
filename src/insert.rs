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

pub trait Insert<I>: Table {
    /// ```
    /// use typed_sql::{Insert, Table}
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64,
    ///     name: String
    /// }
    ///
    /// struct UserInsert {}
    /// ```
    fn insert(value: I) -> InsertStatement<Self, I>
    where
        I: Insertable,
    {
        InsertStatement::new(value)
    }

    fn insert_values(values: I) -> InsertStatement<Self, Values<I>>
    where
        I: IntoIterator + Clone,
        I::Item: Insertable,
    {
        InsertStatement::new(Values { iter: values })
    }
}

impl<I, T: Table> Insert<I> for T {}

pub struct Values<I> {
    iter: I,
}

pub struct InsertStatement<T: ?Sized, V> {
    values: V,
    _table: PhantomData<T>,
}

impl<T, I> InsertStatement<T, I>
where
    T: Table + ?Sized,
{
    fn new(values: I) -> Self {
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
    fn write_sql(&self, sql: &mut String) {
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
    fn write_sql(&self, sql: &mut String) {
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
