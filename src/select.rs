use crate::{Join, Sql, Table, ToSql};
use std::marker::PhantomData;

pub struct WildCard;

pub trait Query {
    fn write_query(sql: &mut String);
}

impl Query for WildCard {
    fn write_query(sql: &mut String) {
        sql.push('*');
    }
}

pub trait Select: Join {
    fn select() -> SelectStatement<Self, WildCard>
    where
        Self: Sized,
    {
        SelectStatement {
            from: PhantomData,
            query: PhantomData,
        }
    }
}

impl<J: Join> Select for J {}

pub struct SelectStatement<F, Q> {
    from: PhantomData<F>,
    query: PhantomData<Q>,
}

impl<F, Q> ToSql for SelectStatement<F, Q>
where
    F: Select,
    Q: Query,
{
    fn write_sql(&self, sql: &mut Sql) {
        sql.buf.push_str("SELECT ");
        Q::write_query(&mut sql.buf);
        sql.buf.push_str(" FROM ");
        sql.buf.push_str(F::Table::NAME);
        F::write_join(sql);
    }
}
