use crate::ToSql;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct Bind {
    pub n: u8,
}

#[derive(Debug)]
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

    fn write_types(sql: &mut String);

    fn write_values(&self, sql: &mut String);

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
    fn write_sql(&self, sql: &mut String) {
        sql.push_str("PREPARE ");
        sql.push_str(self.name);

       
        B::write_types(sql);
    

        sql.push_str(" AS ");
        self.stmt.write_sql(sql);
    }
}

impl<B, S: Copy> Clone for Prepare<'_, B, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B, S: Copy> Copy for Prepare<'_, B, S> {}

#[derive(Debug, Clone, Copy)]
pub struct Execute<'a, B> {
    name: &'a str,
    binding: B,
}

impl<B: Binding> ToSql for Execute<'_, B> {
    fn write_sql(&self, sql: &mut String) {
        sql.push_str("EXECUTE ");
        sql.push_str(self.name);
        sql.push('(');
        self.binding.write_values(sql);
        sql.push(')');
    }
}
