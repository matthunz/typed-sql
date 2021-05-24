use crate::types::Field;
use crate::Table;
use std::marker::PhantomData;

pub struct QueryableWriter<Q: ?Sized> {
    queryable: PhantomData<Q>,
}

impl<Q: Queryable + ?Sized> WriteQueryable for QueryableWriter<Q> {
    fn write_query(&self, sql: &mut String) {
        Q::write_queryable(sql);
    }
}

pub trait Queryable {
    fn write_queryable(sql: &mut String);

    fn queryable() -> QueryableWriter<Self> {
        QueryableWriter {
            queryable: PhantomData,
        }
    }
}

pub trait WriteQueryable {
    fn write_query(&self, sql: &mut String);
}

pub struct WildCard;

impl WriteQueryable for WildCard {
    fn write_query(&self, sql: &mut String) {
        sql.push('*');
    }
}

pub struct Count<T> {
    column: T,
}

impl<T> Count<T> {
    pub(crate) fn new(column: T) -> Self {
        Self { column }
    }
}

impl WriteQueryable for Count<()> {
    fn write_query(&self, sql: &mut String) {
        write_wildcard(sql);
    }
}

impl WriteQueryable for Count<WildCard> {
    fn write_query(&self, sql: &mut String) {
        write_wildcard(sql);
    }
}

impl<T: Table, A> WriteQueryable for Count<Field<T, A>> {
    fn write_query(&self, sql: &mut String) {
        sql.push_str("COUNT(");
        self.column.write_field(sql);
        sql.push(')');
    }
}

fn write_wildcard(sql: &mut String) {
    sql.push_str("COUNT(*)");
}
