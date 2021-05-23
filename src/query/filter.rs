use super::delete::Delete;
use super::select::{Select, Selectable};
use super::update::Update;
use super::Predicate;
use crate::{Table, ToSql};

pub trait Filterable {
    type Fields: Default;
}

impl<S: Select> Filterable for S {
    type Fields = <S::Selectable as Selectable>::Fields;
}

impl<T: Table + ?Sized> Filterable for Delete<T> {
    type Fields = T::Fields;
}

impl<T: Table + ?Sized, S> Filterable for Update<T, S> {
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
