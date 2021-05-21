use super::delete::Delete;
use super::select::{Select, Selectable};
use super::Predicate;
use crate::{Table, ToSql};

pub trait Filterable {
    type Fields: Default;
}

impl<Q: Select> Filterable for Q {
    type Fields = <Q::Selectable as Selectable>::Fields;
}

impl<T: Table + ?Sized> Filterable for Delete<T> {
    type Fields = T::Fields;
}

#[derive(Clone, Copy, Debug)]
pub struct Filter<S, P> {
    stmt: S,
    predicate: P,
}

impl<S, P> Filter<S, P> {
    pub(crate) const fn new(stmt: S, predicate: P) -> Self {
        Self { stmt, predicate }
    }
}

impl<S, P> ToSql for Filter<S, P>
where
    S: ToSql,
    P: Predicate,
{
    fn write_sql(&self, sql: &mut String) {
        self.stmt.write_sql(sql);
        sql.push_str(" WHERE ");
        self.predicate.write_predicate(sql);
    }
}
