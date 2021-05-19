use crate::{field::Predicate, Join, Sql, Table, ToSql};
use std::marker::PhantomData;

pub struct WildCard;

pub trait Queryable {
    fn write_query(sql: &mut String);
}

impl Queryable for WildCard {
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

pub struct SelectStatement<S, Q> {
    from: PhantomData<S>,
    query: PhantomData<Q>,
}

impl<S: Select, Q> SelectStatement<S, Q> {
    pub fn filter<F, P>(self, f: F) -> Filter<S, Q, P>
    where
        F: FnOnce(S::Fields) -> P,
    {
        Filter {
            select: self,
            predicate: f(Default::default()),
        }
    }
}

impl<S, Q> ToSql for SelectStatement<S, Q>
where
    S: Select,
    Q: Queryable,
{
    fn write_sql(&self, sql: &mut Sql) {
        sql.buf.push_str("SELECT ");
        Q::write_query(&mut sql.buf);
        sql.buf.push_str(" FROM ");
        sql.buf.push_str(S::Table::NAME);
        S::write_join(sql);
    }
}

pub struct Filter<S, Q, P> {
    select: SelectStatement<S, Q>,
    predicate: P,
}

impl<S, Q, P> ToSql for Filter<S, Q, P>
where
    S: Select,
    Q: Queryable,
    P: Predicate,
{
    fn write_sql(&self, sql: &mut Sql) {
        self.select.write_sql(sql);
        sql.buf.push_str(" WHERE ");
        self.predicate.write_predicate(&mut sql.buf);
    }
}
