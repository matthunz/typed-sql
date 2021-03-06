use super::Select;
use crate::{
    sql::CheckedSql,
    types::field::{Field, Then},
};
use crate::{Table, ToSql};
use std::marker::PhantomData;

pub struct OrderBy<Q, O> {
    stmt: Q,
    order: O,
}

impl<Q, O> OrderBy<Q, O> {
    pub(crate) fn new(stmt: Q, order: O) -> Self {
        Self { stmt, order }
    }
}

impl<S, O> Select for OrderBy<S, O>
where
    S: Select,
    O: Order,
{
    type Selectable = S::Selectable;
    type Queryable = S::Queryable;
}

impl<Q, O> ToSql for OrderBy<Q, O>
where
    Q: Select,
    O: Order,
{
    fn write_sql_unchecked(&self, sql: &mut String) {
        self.stmt.write_sql_unchecked(sql);
        sql.push_str(" ORDER BY ");
        self.order.write_order(sql);
    }
}

impl<Q: CheckedSql, O> CheckedSql for OrderBy<Q, O> {}

pub trait Direction {
    const DIRECTION: &'static str;
}

pub struct Ascending;

impl Direction for Ascending {
    const DIRECTION: &'static str = "ASC";
}

pub struct Descending;

impl Direction for Descending {
    const DIRECTION: &'static str = "DESC";
}

pub struct Ordered<T, A, D> {
    pub(crate) field: Field<T, A>,
    _direction: PhantomData<D>,
}

impl<T, A, D> Ordered<T, A, D> {
    pub(crate) fn new(field: Field<T, A>) -> Self {
        Self {
            field,
            _direction: PhantomData,
        }
    }

    pub fn then<T2>(self, next: T2) -> Then<Self, T2> {
        Then {
            head: self,
            tail: next,
        }
    }
}

pub trait Order {
    fn write_order(&self, sql: &mut String);
}

impl<T: Table, A, D: Direction> Order for Ordered<T, A, D> {
    fn write_order(&self, sql: &mut String) {
        self.field.write_field(sql);
        sql.push(' ');
        sql.push_str(D::DIRECTION);
    }
}

impl<H: Order, T: Order> Order for Then<H, T> {
    fn write_order(&self, sql: &mut String) {
        self.head.write_order(sql);
        sql.push(',');
        self.tail.write_order(sql);
    }
}

impl<T: Table, A> Order for Field<T, A> {
    fn write_order(&self, sql: &mut String) {
        self.write_field(sql);
    }
}
