use std::fmt::Write;

pub mod bind;
pub use bind::Bind;

pub mod field;
pub use field::Field;

pub trait Primative {
    fn write_primative(&self, sql: &mut String);
}

impl Primative for String {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(&self);
        sql.push('\'');
    }
}

impl Primative for &'_ str {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(self);
        sql.push('\'');
    }
}

impl Primative for i64 {
    fn write_primative(&self, sql: &mut String) {
        sql.write_fmt(format_args!("{}", self)).unwrap();
    }
}