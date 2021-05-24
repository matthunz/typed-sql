use crate::{Binding, Prepared, ToSql};
use std::marker::PhantomData;

pub struct Prepare<'a, B, S> {
    name: &'a str,
    binding: PhantomData<B>,
    stmt: S,
}

impl<'a, B, S> Prepare<'a, B, S> {
    pub(crate) fn new(name: &'a str, stmt: S) -> Self {
        Prepare {
            name,
            binding: PhantomData,
            stmt,
        }
    }
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
    fn write_sql_unchecked(&self, sql: &mut String) {
        sql.push_str("PREPARE ");
        sql.push_str(self.name);

        B::write_types(sql);

        sql.push_str(" AS ");
        self.stmt.write_sql_unchecked(sql);
    }
}

impl<B, S: Prepared> Prepared for Prepare<'_, B, S> {}

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
    fn write_sql_unchecked(&self, sql: &mut String) {
        sql.push_str("EXECUTE ");
        sql.push_str(self.name);
        sql.push('(');
        self.binding.write_values(sql);
        sql.push(')');
    }
}

impl<B> Prepared for Execute<'_, B> {}
