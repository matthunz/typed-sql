#![feature(min_type_alias_impl_trait)]

pub mod bind;
pub use bind::Binding;

pub mod field;

pub mod insert;
pub use insert::Insert;

pub mod join;
pub use join::Join;

pub mod select;
pub use select::{QueryDsl, Select};

mod sql;
pub use sql::{ToSql, WriteSql};

pub use typed_sql_derive::*;

pub trait Table {
    const NAME: &'static str;

    type Fields: Default;
}
