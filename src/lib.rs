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
use select::{
    query::{Count, Queryable},
    SelectStatement, WildCard,
};

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
        SelectStatement::new(PhantomData, WildCard)
    }

    fn query<Q>(query: Q) -> SelectStatement<PhantomData<Self>, Q>
    where
        Self: Sized,
        Q: Queryable,
    {
        SelectStatement::new(PhantomData, query)
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {
    ///    content: Option<String>
    /// }
    ///
    /// let stmt = Post::count(|post| post.content);
    /// assert_eq!(stmt.to_sql(), "SELECT COUNT(posts.content) FROM posts;");
    /// ```
    /// ## Wildcard
    /// ```
    /// use typed_sql::{Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {}
    ///
    /// let stmt = Post::count(|_| {});
    /// assert_eq!(stmt.to_sql(), "SELECT COUNT(*) FROM posts;");
    /// ```
    fn count<F, T>(f: F) -> SelectStatement<PhantomData<Self>, Count<T>>
    where
        Self: Sized,
        F: FnOnce(Self::Fields) -> T,
    {
        SelectStatement::new(PhantomData, Count::new(f(Default::default())))
    }
}
