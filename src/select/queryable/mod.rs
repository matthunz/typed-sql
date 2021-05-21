use std::fmt::Write;

use super::selectable::{Filter, SelectStatement, Selectable};
use super::{Predicate, Query};
use crate::ToSql;

pub mod group;
pub use group::{GroupBy, GroupOrder};

pub mod order;
pub use order::{Order, OrderBy};

pub trait Queryable: ToSql {
    type Select: Selectable;
    type Query: Query;
}

impl<S, Q> Queryable for SelectStatement<S, Q>
where
    S: Selectable,
    Q: Query,
{
    type Select = S;
    type Query = Q;
}

impl<S, Q, P> Queryable for Filter<S, Q, P>
where
    S: Selectable,
    Q: Query,
    P: Predicate,
{
    type Select = S;
    type Query = Q;
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

impl<Q: Queryable> ToSql for Limit<Q> {
    fn write_sql(&self, sql: &mut String) {
        self.queryable.write_sql(sql);
        sql.write_fmt(format_args!(" LIMIT {}", self.limit))
            .unwrap();
    }
}

impl<Q: Queryable> Queryable for Limit<Q> {
    type Select = Q::Select;
    type Query = Q::Query;
}
