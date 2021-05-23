//! # Complex queries
//! See [Query] for available methods.
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
//! ## Injections
//! Queries with user input parameters are vulnerable to SQL injections.
//!
//! To avoid this use prepared statements with [Binding].
//! ```
//! use typed_sql::{Binding, Query, Table, ToSql};
//!
//! #[derive(Binding, Table)]
//! struct User {
//!     id: i64,
//! }
//!
//! let id_plan = User::prepare("idplan", |binds| {
//!     User::table()
//!         .select()
//!         .filter(|user| user.id.eq(binds.id))
//! });
//!
//! assert_eq!(id_plan.to_sql(), "PREPARE idplan AS SELECT * FROM users WHERE users.id=$1;");
//!
//! let stmt = id_plan.execute(User { id: 0 });
//! assert_eq!(stmt.to_sql(), "EXECUTE id_plan(0)");
//! ```

#![feature(associated_type_defaults)]
#![feature(min_type_alias_impl_trait)]

pub mod conn;

pub mod query;
pub use query::{Insertable, Join, Query};

mod sql;
pub use sql::ToSql;

pub mod table;
pub use table::Table;

pub mod types;
pub use types::Binding;

pub use typed_sql_derive::*;
