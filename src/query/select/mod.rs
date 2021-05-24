use std::fmt::Write;

use super::filter::Filter;
use super::Predicate;
use crate::sql::{CheckedSql, ToSql};

pub mod group;
pub use group::{GroupBy, GroupOrder};

pub mod join;
pub use join::{Join, Joined};

pub mod order;
pub use order::{Order, OrderBy};

pub mod queryable;
pub use queryable::{Queryable, WildCard, WriteQueryable};

mod selectable;
pub use selectable::{SelectStatement, Selectable};

pub trait Select: ToSql {
    type Selectable: Selectable;
    type Queryable: WriteQueryable;
}

impl<S, Q> Select for SelectStatement<S, Q>
where
    S: Selectable,
    Q: WriteQueryable,
{
    type Selectable = S;
    type Queryable = Q;
}

impl<S, Q, P> Select for Filter<SelectStatement<S, Q>, P>
where
    S: Selectable,
    Q: WriteQueryable,
    P: Predicate,
{
    type Selectable = S;
    type Queryable = Q;
}

#[derive(Debug, Clone, Copy)]
pub struct Limit<Q> {
    queryable: Q,
    limit: usize,
}

impl<Q> Limit<Q> {
    #[inline(always)]
    pub(crate) const fn new(queryable: Q, limit: usize) -> Self {
        Self { queryable, limit }
    }
}

impl<Q: Select> Select for Limit<Q> {
    type Selectable = Q::Selectable;
    type Queryable = Q::Queryable;
}

impl<Q: Select> ToSql for Limit<Q> {
    fn write_sql_unchecked(&self, sql: &mut String) {
        self.queryable.write_sql_unchecked(sql);
        sql.write_fmt(format_args!(" LIMIT {}", self.limit))
            .unwrap();
    }
}

impl<Q: CheckedSql> CheckedSql for Limit<Q> {}
