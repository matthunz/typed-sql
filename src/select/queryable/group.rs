use crate::field::{Field, Then};
use crate::select::Queryable;
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

impl<Q, O> ToSql for GroupBy<Q, O>
where
    Q: Queryable,
    O: GroupOrder,
{
    fn write_sql(&self, sql: &mut String) {
        self.stmt.write_sql(sql);
        sql.push_str(" GROUP BY ");
        self.order.write_columns(sql);
    }
}

impl<Q, O> Queryable for GroupBy<Q, O>
where
    Q: Queryable,
    O: GroupOrder,
{
    type Select = Q::Select;
}
