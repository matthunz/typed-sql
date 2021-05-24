use crate::table::{Table, TableQueryable};
use crate::types::bind::{Binder, Binding};
use crate::ToSql;

pub mod delete;
use delete::Delete;

pub mod filter;
use filter::Filter;
pub use filter::Filterable;

pub mod insert;
pub use insert::Insertable;
use insert::{InsertSelect, InsertStatement, Values};

pub mod predicate;
pub use predicate::Predicate;
use predicate::{And, Or};

pub mod prepare;
use prepare::Prepare;

pub mod select;
use select::queryable::{Count, WildCard, WriteQueryable};
use select::{GroupBy, GroupOrder, Limit, Order, OrderBy, SelectStatement, Selectable};
pub use select::{Join, Joined, Queryable, Select};

pub mod update;
use update::{Update, UpdateSet};

pub trait Query: Sized {
    /// # Examples
    /// ```
    /// use typed_sql::{Binding, Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {
    ///     id: i64,
    ///     content: String
    /// }
    ///
    /// #[derive(Binding)]
    /// struct PostBinding {
    ///     id: i64
    /// }
    ///
    /// let stmt = PostBinding::prepare("postplan", |binds| {
    ///     Post::table()
    ///         .select()
    ///         .filter(|post| post.id.eq(binds.id))
    /// });
    ///
    /// assert_eq!(
    ///     stmt.to_sql(),
    ///     "PREPARE postplan AS SELECT * FROM posts WHERE posts.id=$1;"
    /// );
    /// ```
    fn prepare<F, S>(name: &str, f: F) -> Prepare<Self, S>
    where
        Self: Binding,
        F: FnOnce(Self::Bindings) -> S,
        S: ToSql,
    {
        let bindings = Self::bindings(&mut Binder::default());
        Prepare::new(name, f(bindings))
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {
    ///     content: String
    /// }
    ///
    /// let stmt = Post::table().select().filter(|p| p.content.eq("foo"));
    ///
    /// assert_eq!(
    ///     stmt.to_sql_unchecked(),
    ///     "SELECT * FROM posts WHERE posts.content = 'foo';"
    /// );
    /// ```
    fn select(self) -> SelectStatement<Self, WildCard>
    where
        Self: Selectable,
    {
        self.query(WildCard)
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Query, Queryable, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {
    ///     id: i64,
    ///     content: String
    /// }
    ///
    /// #[derive(Queryable)]
    /// struct PostQuery {
    ///     content: String
    /// }
    ///
    /// let stmt = Post::table().query(PostQuery::queryable());
    ///
    /// assert_eq!(
    ///     stmt.to_sql_unchecked(),
    ///     "SELECT content FROM posts;"
    /// );
    /// ```
    fn query<Q>(self, query: Q) -> SelectStatement<Self, Q>
    where
        Self: Selectable,
        Q: WriteQueryable,
    {
        SelectStatement::new(self, query)
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {
    ///    content: Option<String>
    /// }
    ///
    /// let stmt = Post::table().count(|post| post.content);
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT COUNT(posts.content) FROM posts;");
    /// ```
    /// ## Wildcard
    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {}
    ///
    /// let stmt = Post::table().count(|_| {});
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT COUNT(*) FROM posts;");
    /// ```
    fn count<F, T>(self, f: F) -> SelectStatement<Self, Count<T>>
    where
        Self: Selectable,
        F: FnOnce(Self::Fields) -> T,
        Count<T>: WriteQueryable,
    {
        self.query(Count::new(f(Default::default())))
    }

    /// ```
    /// use typed_sql::{Insertable, Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64,
    ///     name: String
    /// }
    ///
    /// #[derive(Insertable)]
    /// struct UserInsert {
    ///     name: &'static str
    /// }
    ///
    /// let stmt = User::table().insert(UserInsert { name: "Matt" });
    ///
    /// assert_eq!(
    ///     stmt.to_sql_unchecked(),
    ///     "INSERT INTO users(name) VALUES ('Matt');"
    /// );
    /// ```
    fn insert<I>(self, value: I) -> InsertStatement<Self::Table, I>
    where
        Self: TableQueryable,
        I: Insertable,
    {
        InsertStatement::new(value)
    }

    fn insert_values<I>(self, values: I) -> InsertStatement<Self::Table, Values<I>>
    where
        Self: TableQueryable,
        I: IntoIterator + Clone,
        I::Item: Insertable,
    {
        InsertStatement::new(Values::new(values))
    }

    fn insert_select<S, I>(self, select: S) -> InsertStatement<Self::Table, InsertSelect<S, I>>
    where
        Self: TableQueryable,
        S: Select,
        I: Insertable,
    {
        InsertStatement::new(InsertSelect::new(select))
    }

    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {
    ///     id: i64,
    ///     name: String
    /// }
    ///
    /// let stmt = Post::table()
    ///     .update(|p| p.id.eq(2).and(p.name.eq("foo")))
    ///     .filter(|p| p.id.eq(1));
    ///
    /// assert_eq!(
    ///     stmt.to_sql_unchecked(),
    ///     "UPDATE posts \
    ///     SET posts.id = 2,posts.name = 'foo' \
    ///     WHERE posts.id = 1;"
    /// );
    /// ```
    fn update<F, S>(self, f: F) -> Update<Self::Table, S>
    where
        Self: TableQueryable,
        F: FnOnce(<Self::Table as Table>::Fields) -> S,
        S: UpdateSet,
    {
        Update::new(f(Default::default()))
    }

    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {
    ///     id: i64
    /// }
    ///
    /// let stmt = Post::table().delete().filter(|p| p.id.eq(2));
    ///
    /// assert_eq!(stmt.to_sql_unchecked(), "DELETE FROM posts WHERE posts.id = 2;");
    /// ```
    fn delete(self) -> Delete<Self::Table>
    where
        Self: TableQueryable,
    {
        Delete::new()
    }

    fn filter<F, P>(self, f: F) -> Filter<Self, P>
    where
        Self: Filterable,
        F: FnOnce(Self::Fields) -> P,
    {
        Filter::new(self, f(Default::default()))
    }

    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64   
    /// }
    ///
    /// let stmt = User::table().select().filter(|user| user.id.neq(2).and(user.id.lt(5)));
    ///
    /// assert_eq!(
    ///     stmt.to_sql_unchecked(),
    ///     "SELECT * FROM users WHERE users.id != 2 AND users.id < 5;"
    /// );
    /// ```
    fn and<P>(self, predicate: P) -> And<Self, P>
    where
        Self: Predicate,
        P: Predicate,
    {
        And {
            head: self,
            tail: predicate,
        }
    }

    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64   
    /// }
    ///
    /// let stmt = User::table()
    ///     .select()
    ///     .filter(|user| user.id.eq(1).or(user.id.eq(3)));
    ///
    /// assert_eq!(
    ///     stmt.to_sql_unchecked(),
    ///     "SELECT * FROM users WHERE users.id = 1 OR users.id = 3;"
    /// );
    fn or<P>(self, predicate: P) -> Or<Self, P>
    where
        Self: Predicate,
        P: Predicate,
    {
        Or {
            head: self,
            tail: predicate,
        }
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Table, ToSql, Query};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64
    /// }
    ///
    /// let stmt = User::table().select().group_by(|user| user.id);
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users GROUP BY users.id;");
    /// ```
    /// ## Multiple columns
    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64,
    ///     name: String
    /// }
    ///
    /// let stmt = User::table().select().group_by(|user| user.id.then(user.name));
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users GROUP BY users.id,users.name;");
    /// ```
    fn group_by<F, O>(self, f: F) -> GroupBy<Self, O>
    where
        Self: Select,
        F: FnOnce(<Self::Selectable as Selectable>::Fields) -> O,
        O: GroupOrder,
    {
        GroupBy::new(self, f(Default::default()))
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64,
    ///     name: String
    /// }
    ///
    /// let stmt = User::table().select().order_by(|user| user.id);
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users ORDER BY users.id;");
    /// ```
    /// ## Direction
    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64
    /// }
    ///
    /// let stmt = User::table().select().order_by(|user| user.id.ascending());
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users ORDER BY users.id ASC;");
    /// ```
    /// ## Multiple columns
    /// ```
    /// use typed_sql::{Query, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64,
    ///     name: String
    /// }
    ///
    /// let stmt = User::table().select()
    ///     .order_by(|user| user.id.ascending().then(user.name.descending()));
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users ORDER BY users.id ASC,users.name DESC;");
    /// ```
    fn order_by<F, O>(self, f: F) -> OrderBy<Self, O>
    where
        Self: Select,
        F: FnOnce(<Self::Selectable as Selectable>::Fields) -> O,
        O: Order,
    {
        OrderBy::new(self, f(Default::default()))
    }

    fn limit(self, limit: usize) -> Limit<Self>
    where
        Self: Select,
    {
        Limit::new(self, limit)
    }
}

impl<T> Query for T {}
