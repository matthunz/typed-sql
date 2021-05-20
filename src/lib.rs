//! ```
//! use typed_sql::{Select, Table, ToSql};
//!
//! #[derive(Table)]
//! struct User {
//!     id: i64,
//!     name: String
//! }
//!
//! let stmt = User::table().select()
//!     .filter(|user| user.id.neq(6).and(user.id.gt(3)))
//!     .group_by(|user| user.name)
//!     .order_by(|user| user.name.then(user.id.ascending()))
//!     .limit(5);
//!
//! assert_eq!(
//!     stmt.to_sql(),
//!     "SELECT * FROM users \
//!     WHERE users.id != 6 AND users.id > 3 \
//!     GROUP BY users.name \
//!     ORDER BY users.name,users.id ASC \
//!     LIMIT 5;"
//! );
//! ```

#![feature(min_type_alias_impl_trait)]

pub mod bind;
use std::marker::PhantomData;

pub use bind::Binding;

pub mod field;

pub mod insert;
pub use insert::Insert;

pub mod join;
pub use join::Join;

pub mod select;
pub use select::Select;
use select::Selectable;

mod sql;
pub use sql::ToSql;

pub mod types;

pub use typed_sql_derive::*;

pub trait Table {
    const NAME: &'static str;

    type Fields: Default;

    fn table() -> SelectTable<Self> {
        SelectTable { table: PhantomData }
    }
}

pub struct SelectTable<T: ?Sized> {
    table: PhantomData<T>,
}

impl<T: Table + ?Sized> Selectable for SelectTable<T> {
    type Table = T;
    type Fields = T::Fields;

    fn write_join(&self, _sql: &mut String) {}
}

impl<T: ?Sized> Clone for SelectTable<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for SelectTable<T> {}
