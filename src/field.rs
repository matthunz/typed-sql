use crate::select::predicate::{Eq, Gt, Lt, Neq, Op};
use crate::select::queryable::order::{Ascending, Descending, Ordered};
use crate::Table;

use std::marker::PhantomData;

pub struct Field<T, A> {
    name: &'static str,
    _table: PhantomData<T>,
    _type: PhantomData<A>,
}

impl<T, A> Field<T, A>
where
    T: Table,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            _table: PhantomData,
            _type: PhantomData,
        }
    }

    pub fn eq<U>(self, rhs: U) -> Op<T, A, U, Eq> {
        Op::new(self, rhs)
    }

    pub fn neq<U>(self, rhs: U) -> Op<T, A, U, Neq> {
        Op::new(self, rhs)
    }

    pub fn gt<U>(self, rhs: U) -> Op<T, A, U, Gt> {
        Op::new(self, rhs)
    }

    pub fn lt<U>(self, rhs: U) -> Op<T, A, U, Lt> {
        Op::new(self, rhs)
    }

    pub fn then<T2>(self, next: T2) -> Then<Self, T2> {
        Then {
            head: self,
            tail: next,
        }
    }

    pub fn ascending(self) -> Ordered<T, A, Ascending> {
        Ordered::new(self)
    }

    pub fn descending(self) -> Ordered<T, A, Descending> {
        Ordered::new(self)
    }

    pub(crate) fn write_field(&self, sql: &mut String) {
        sql.push_str(T::NAME);
        sql.push('.');
        sql.push_str(self.name);
    }
}

impl<T, A> Copy for Field<T, A> {}

impl<T, A> Clone for Field<T, A> {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct Then<H, T> {
    pub(crate) head: H,
    pub(crate) tail: T,
}

impl<H, T> Then<H, T> {
    pub fn then<T2>(self, next: T2) -> Then<Self, T2> {
        Then {
            head: self,
            tail: next,
        }
    }
}
