use crate::bind::Bind;
use crate::field::Field;
use crate::Table;
use std::{
    fmt::{Display, Write},
    marker::PhantomData,
};

pub trait Predicate {
    fn write_predicate(&self, sql: &mut String);
}

pub struct And<H, T> {
    pub(crate) head: H,
    pub(crate) tail: T,
}

impl<H, T> Predicate for And<H, T>
where
    H: Predicate,
    T: Predicate,
{
    fn write_predicate(&self, sql: &mut String) {
        self.head.write_predicate(sql);
        sql.push_str(" AND ");
        self.tail.write_predicate(sql);
    }
}

pub trait Operator {
    fn write_operator(sql: &mut String);
}

pub struct Eq;

impl Operator for Eq {
    fn write_operator(sql: &mut String) {
        sql.push('=');
    }
}

pub struct Neq;

impl Operator for Neq {
    fn write_operator(sql: &mut String) {
        sql.push_str("!=");
    }
}

pub struct Gt;

impl Operator for Gt {
    fn write_operator(sql: &mut String) {
        sql.push('>');
    }
}

pub struct Lt;

impl Operator for Lt {
    fn write_operator(sql: &mut String) {
        sql.push('<');
    }
}

pub struct Op<T, A, U, O> {
    lhs: Field<T, A>,
    rhs: U,
    _operator: PhantomData<O>,
}

impl<T, A, U, O> Op<T, A, U, O> {
    pub(crate) fn new(lhs: Field<T, A>, rhs: U) -> Self {
        Self {
            lhs,
            rhs,
            _operator: PhantomData,
        }
    }
}

impl<T, A, U, O> Predicate for Op<T, A, U, O>
where
    T: Table,
    U: Display,
    O: Operator,
{
    fn write_predicate(&self, sql: &mut String) {
        self.lhs.write_field(sql);
        sql.push(' ');
        O::write_operator(sql);
        sql.push(' ');
        sql.write_fmt(format_args!("{}", self.rhs)).unwrap();
    }
}

impl<T, T2, A, O> Predicate for Op<T, A, Field<T2, A>, O>
where
    T: Table,
    T2: Table,
    O: Operator,
{
    fn write_predicate(&self, sql: &mut String) {
        self.lhs.write_field(sql);
        O::write_operator(sql);
        self.rhs.write_field(sql);
    }
}

impl<T, A, O> Predicate for Op<T, A, Bind, O>
where
    T: Table,
    O: Operator,
{
    fn write_predicate(&self, sql: &mut String) {
        self.lhs.write_field(sql);
        O::write_operator(sql);
        sql.write_fmt(format_args!("${}", self.rhs.n)).unwrap();
    }
}
