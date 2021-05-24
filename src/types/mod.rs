use std::fmt::Write;

pub mod bind;
pub use bind::{Bind, Binding};

pub mod field;
pub use field::Field;

pub trait Primitive {
    fn write_primative(&self, sql: &mut String);
}

impl Primitive for String {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(&self);
        sql.push('\'');
    }
}

impl Primitive for &'_ str {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(self);
        sql.push('\'');
    }
}

impl Primitive for i64 {
    fn write_primative(&self, sql: &mut String) {
        sql.write_fmt(format_args!("{}", self)).unwrap();
    }
}

impl<P: Primitive> Primitive for Option<P> {
    fn write_primative(&self, sql: &mut String) {
        if let Some(primative) = self {
            primative.write_primative(sql);
        } else {
            sql.push_str("NULL");
        }
    }
}
