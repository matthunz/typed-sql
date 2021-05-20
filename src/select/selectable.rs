use super::{Predicate, Query};
use crate::join::JoinSelect;
use crate::{Table, ToSql};

pub trait Selectable {
    type Table: Table + ?Sized;
    type Fields: Default;

    fn write_join(&self, sql: &mut String);
}

impl<J: JoinSelect> Selectable for J {
    type Table = J::Table;
    type Fields = J::Fields;

    fn write_join(&self, sql: &mut String) {
        self.write_join_select(sql);
    }
}

pub struct SelectStatement<S, Q> {
    from: S,
    query: Q,
}

impl<S: Selectable, Q> SelectStatement<S, Q> {
    pub fn new(from: S, query: Q) -> Self {
        Self { from, query }
    }

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
    S: Selectable,
    Q: Query,
{
    fn write_sql(&self, sql: &mut String) {
        sql.push_str("SELECT ");
        self.query.write_query(sql);
        sql.push_str(" FROM ");
        sql.push_str(S::Table::NAME);
        self.from.write_join(sql);
    }
}

pub struct Filter<S, Q, P> {
    select: SelectStatement<S, Q>,
    predicate: P,
}

impl<S, Q, P> ToSql for Filter<S, Q, P>
where
    S: Selectable,
    Q: Query,
    P: Predicate,
{
    fn write_sql(&self, sql: &mut String) {
        self.select.write_sql(sql);
        sql.push_str(" WHERE ");
        self.predicate.write_predicate(sql);
    }
}
