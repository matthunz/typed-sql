use super::join::JoinSelect;
use super::Queryable;
use crate::table::{Table, TableQuery};
use crate::ToSql;

pub trait Selectable {
    type Table: Table + ?Sized;
    type Fields: Default;

    fn write_join(&self, sql: &mut String);
}

impl<T: Table + ?Sized> Selectable for TableQuery<T> {
    type Table = T;
    type Fields = T::Fields;

    fn write_join(&self, _sql: &mut String) {}
}

impl<J: JoinSelect> Selectable for J {
    type Table = J::Table;
    type Fields = J::Fields;

    fn write_join(&self, sql: &mut String) {
        self.write_join_select(sql);
    }
}

#[derive(Clone, Copy)]
pub struct SelectStatement<S, Q> {
    from: S,
    query: Q,
}

impl<S: Selectable, Q> SelectStatement<S, Q> {
    pub fn new(from: S, query: Q) -> Self {
        Self { from, query }
    }
}

impl<S, Q> ToSql for SelectStatement<S, Q>
where
    S: Selectable,
    Q: Queryable,
{
    fn write_sql(&self, sql: &mut String) {
        sql.push_str("SELECT ");
        self.query.write_query(sql);
        sql.push_str(" FROM ");
        sql.push_str(S::Table::NAME);
        self.from.write_join(sql);
    }
}
