use std::marker::PhantomData;

use crate::{Sql, ToSql};

pub struct Bind {
    pub n: u8,
}

pub struct Binder {
    counter: u8,
}

impl Default for Binder {
    fn default() -> Self {
        Self { counter: 1 }
    }
}

impl Binder {
    pub fn bind(&mut self) -> Bind {
        let n = self.counter;
        self.counter += 1;
        Bind { n }
    }
}

pub trait Binding {
    type Bindings;

    fn bindings(binder: &mut Binder) -> Self::Bindings;

    fn write_types(sql: &mut Sql);

    fn write_values(&self, sql: &mut Sql);

    fn prepare<F, S>(name: &str, f: F) -> Prepare<Self, S>
    where
        Self: Sized,
        F: FnOnce(Self::Bindings) -> S,
        S: ToSql,
    {
        let bindings = Self::bindings(&mut Binder::default());
        Prepare {
            name,
            binding: PhantomData,
            stmt: f(bindings),
        }
    }
}

pub struct Prepare<'a, B, S> {
    name: &'a str,
    binding: PhantomData<B>,
    stmt: S,
}

impl<B: Binding, S: ToSql> Prepare<'_, B, S> {
    pub fn execute(&self, binding: B) -> Execute<B> {
        Execute {
            name: self.name,
            binding,
        }
    }
}

impl<B: Binding, S: ToSql> ToSql for Prepare<'_, B, S> {
    fn write_sql(&self, sql: &mut Sql) {
        sql.buf.push_str("PREPARE ");
        sql.buf.push_str(self.name);

        sql.buf.push('(');
        B::write_types(sql);
        sql.buf.push(')');

        sql.buf.push_str(" AS ");
        self.stmt.write_sql(sql);
    }
}

pub struct Execute<'a, B> {
    name: &'a str,
    binding: B,
}

impl<B: Binding> ToSql for Execute<'_, B> {
    fn write_sql(&self, sql: &mut Sql) {
        sql.buf.push_str("EXECUTE ");
        sql.buf.push_str(self.name);
        sql.buf.push('(');
        self.binding.write_values(sql);
        sql.buf.push(')');
    }
}
