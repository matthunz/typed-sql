use crate::Table;
use std::fmt::{Display, Write};
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

    pub fn eq<U>(self, rhs: U) -> Eq<T, A, U> {
        Eq { lhs: self, rhs }
    }

    fn write_field(&self, sql: &mut String) {
        sql.push_str(T::NAME);
        sql.push('.');
        sql.push_str(self.name);
    }
}

pub struct Eq<T, A, U> {
    lhs: Field<T, A>,
    rhs: U,
}

pub trait Predicate {
    fn write_predicate(&self, sql: &mut String);
}

impl Predicate for () {
    fn write_predicate(&self, _sql: &mut String) {}
}

impl<T, A, U> Predicate for Eq<T, A, U>
where
    T: Table,
    U: Display,
{
    fn write_predicate(&self, sql: &mut String) {
        self.lhs.write_field(sql);
        sql.push('=');
        sql.write_fmt(format_args!("{}", self.rhs)).unwrap();
    }
}

impl<T, T2, A> Predicate for Eq<T, A, Field<T2, A>>
where
    T: Table,
    T2: Table,
{
    fn write_predicate(&self, sql: &mut String) {
        self.lhs.write_field(sql);
        sql.push('=');
        self.rhs.write_field(sql);
    }
}
