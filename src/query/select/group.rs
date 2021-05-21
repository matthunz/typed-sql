use super::Select;
use crate::types::field::{Field, Then};
use crate::{Table, ToSql};

pub trait GroupOrder {
    fn write_columns(&self, sql: &mut String);
}

impl<T, A> GroupOrder for Field<T, A>
where
    T: Table,
{
    fn write_columns(&self, sql: &mut String) {
        self.write_field(sql);
    }
}

impl<H, T> GroupOrder for Then<H, T>
where
    H: GroupOrder,
    T: GroupOrder,
{
    fn write_columns(&self, sql: &mut String) {
        self.head.write_columns(sql);
        sql.push(',');
        self.tail.write_columns(sql);
    }
}

pub struct GroupBy<Q, O> {
    stmt: Q,
    order: O,
}

impl<Q, O> GroupBy<Q, O> {
    pub(crate) fn new(stmt: Q, order: O) -> Self {
        Self { stmt, order }
    }
}

impl<S, O> ToSql for GroupBy<S, O>
where
    S: Select,
    O: GroupOrder,
{
    fn write_sql(&self, sql: &mut String) {
        self.stmt.write_sql(sql);
        sql.push_str(" GROUP BY ");
        self.order.write_columns(sql);
    }
}

impl<S, O> Select for GroupBy<S, O>
where
    S: Select,
    O: GroupOrder,
{
    type Selectable = S::Selectable;
    type Queryable = S::Queryable;
}
