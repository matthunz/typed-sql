#![feature(min_type_alias_impl_trait)]

pub mod bind;
use std::marker::PhantomData;

pub use bind::Binding;

pub mod field;

pub mod insert;
pub use insert::Insert;

pub mod join;
pub use join::{Join, JoinSelect};

pub mod select;
pub use select::QueryDsl;
use select::{SelectStatement, WildCard};

mod sql;
pub use sql::ToSql;

pub use typed_sql_derive::*;

pub trait Table {
    const NAME: &'static str;

    type Fields: Default;

    fn select() -> SelectStatement<PhantomData<Self>, WildCard>
    where
        Self: Sized,
    {
        SelectStatement::new(PhantomData)
    }
}
