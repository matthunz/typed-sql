use super::selectable::{Filter, SelectStatement, Selectable};
use super::Predicate;
use crate::field::Field;
use crate::{Table, ToSql};

pub trait Query: ToSql {
    type Select: Selectable;
}

impl<S, Q> Query for SelectStatement<S, Q>
where
    S: Selectable,
    Q: Queryable,
{
    type Select = S;
}

impl<S, Q, P> Query for Filter<S, Q, P>
where
    S: Selectable,
    Q: Queryable,
    P: Predicate,
{
    type Select = S;
}

pub trait Queryable {
    fn write_query(&self, sql: &mut String);
}

pub struct WildCard;

impl Queryable for WildCard {
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

impl Queryable for Count<()> {
    fn write_query(&self, sql: &mut String) {
        write_wildcard(sql);
    }
}

impl Queryable for Count<WildCard> {
    fn write_query(&self, sql: &mut String) {
        write_wildcard(sql);
    }
}

impl<T: Table, A> Queryable for Count<Field<T, A>> {
    fn write_query(&self, sql: &mut String) {
        sql.push_str("COUNT(");
        self.column.write_field(sql);
        sql.push(')');
    }
}

fn write_wildcard(sql: &mut String) {
    sql.push_str("COUNT(*)");
}
