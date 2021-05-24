//! # Complex queries
//! See [`Query`] for available methods.
//!
//! ```
//! use typed_sql::{Query, Table, ToSql};
//!
//! #[derive(Table)]
//! struct User {
//!     id: i64,
//!     name: String
//! }
//!
//! let stmt = User::table()
//!     .select()
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
//! ## Injections
//! Queries with user input strings are vulnerable to SQL injections
//! and therefore must be serialized with [`ToSql::to_sql_unchecked`].
//!
//! ```
//! use typed_sql::{Query, Table, ToSql};
//!
//! #[derive(Table)]
//! struct User {
//!     name: String
//! }
//!
//! let stmt = User::table()
//!         .select()
//!         .filter(|user| user.name.eq("foo"));
//!
//! assert_eq!(
//!     stmt.to_sql_unchecked(),
//!     "SELECT * FROM users WHERE users.name = 'foo';"
//! );
//! ```
//!
//! To avoid this use prepared statements with [`Binding`].
//! ```
//! use typed_sql::{Binding, Query, Table, ToSql};
//!
//! #[derive(Binding, Table)]
//! struct User {
//!     name: String
//! }
//!
//! let id_plan = User::prepare("idplan", |binds| {
//!     User::table().update(|user| user.name.eq(binds.name))
//! });
//!
//! assert_eq!(
//!     id_plan.to_sql(),
//!     "PREPARE idplan AS UPDATE users SET users.name = $1;"
//! );
//!
//! let stmt = id_plan.execute(User { name: String::from("foo") });
//! assert_eq!(stmt.to_sql(), "EXECUTE idplan('foo');");
//! ```

#![feature(min_type_alias_impl_trait)]

pub mod conn;

pub mod query;
pub use query::{Insertable, Join, Query, Queryable};

mod sql;
pub use sql::{CheckedSql, ToSql};

pub mod table;
pub use table::Table;

pub mod types;
pub use types::Binding;

pub use typed_sql_derive::*;
