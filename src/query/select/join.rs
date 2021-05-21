use super::SelectStatement;
use crate::query::queryable::WildCard;
use crate::query::Predicate;
use crate::Table;
use std::marker::PhantomData;

pub struct Inner;

pub trait JoinKind {
    const KIND: &'static str;
}

impl JoinKind for Inner {
    const KIND: &'static str = "INNER";
}

/// ```
/// use typed_sql::{Join, Query, Table, ToSql};
/// use typed_sql::query::Joined;
///
/// #[derive(Table)]
/// struct User {
///     id: i64   
/// }
///
/// #[derive(Table)]
/// struct Post {
///     id: i64,
///     user_id: i64
/// }
///
/// #[derive(Join)]
/// struct UserPost {
///    user: User,
///    post: Post
/// }
///
/// let join = UserPost::join(|join| UserPostJoin {
///     post: Joined::new(join.user.id.eq(join.post.user_id)),
/// });
///
/// assert_eq!(
///     join.select().to_sql(),
///     "SELECT * FROM users INNER JOIN posts ON users.id=posts.user_id;"
/// );
/// ```
pub trait Join<P> {
    type Table: Table;
    type Fields: Default;
    type Join: JoinSelect;

    fn join<F>(f: F) -> Self::Join
    where
        F: FnOnce(Self::Fields) -> Self::Join,
    {
        f(Default::default())
    }
}

pub trait JoinSelect {
    type Table: Table;
    type Fields: Default;

    fn write_join_select(&self, sql: &mut String);

    fn select(self) -> SelectStatement<Self, WildCard>
    where
        Self: Sized,
    {
        SelectStatement::new(self, WildCard)
    }
}

pub struct Joined<P, K, T> {
    predicate: P,
    _kind: PhantomData<K>,
    _table: PhantomData<T>,
}

impl<P, K, T> Joined<P, K, T>
where
    P: Predicate,
    K: JoinKind,
    T: Table,
{
    pub fn new(predicate: P) -> Self {
        Self {
            predicate,
            _kind: PhantomData,
            _table: PhantomData,
        }
    }

    pub fn write_join(&self, sql: &mut String) {
        sql.push(' ');
        sql.push_str(K::KIND);
        sql.push_str(" JOIN ");
        sql.push_str(T::NAME);
        sql.push_str(" ON ");
        self.predicate.write_predicate(sql);
    }
}
