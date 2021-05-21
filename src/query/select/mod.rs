use std::fmt::Write;

use super::filter::Filter;
use super::{Predicate, Queryable};
use crate::ToSql;

pub mod group;
pub use group::{GroupBy, GroupOrder};

pub mod join;
pub use join::{Join, Joined};

pub mod order;
pub use order::{Order, OrderBy};

mod selectable;
pub use selectable::{SelectStatement, Selectable};

pub trait Select: ToSql {
    type Selectable: Selectable;
    type Queryable: Queryable;
}

impl<S, Q> Select for SelectStatement<S, Q>
where
    S: Selectable,
    Q: Queryable,
{
    type Selectable = S;
    type Queryable = Q;
}

impl<S, Q, P> Select for Filter<SelectStatement<S, Q>, P>
where
    S: Selectable,
    Q: Queryable,
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

impl<Q: Select> ToSql for Limit<Q> {
    fn write_sql(&self, sql: &mut String) {
        self.queryable.write_sql(sql);
        sql.write_fmt(format_args!(" LIMIT {}", self.limit))
            .unwrap();
    }
}

impl<Q: Select> Select for Limit<Q> {
    type Selectable = Q::Selectable;
    type Queryable = Q::Queryable;
}
