pub mod predicate;
use predicate::And;
pub use predicate::Predicate;

pub mod query;
pub use query::Query;
use query::{Count, WildCard};

pub mod queryable;
pub use queryable::Queryable;
use queryable::{GroupBy, GroupOrder, Order, OrderBy};

pub mod selectable;
use selectable::SelectStatement;
pub use selectable::Selectable;

use self::queryable::Limit;

pub trait Select: Sized {
    /// # Examples
    /// ```
    /// use typed_sql::{Select, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct Post {
    ///     content: String
    /// }
    ///
    /// let stmt = Post::table().select().filter(|p| p.content.eq("foo"));
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM posts WHERE posts.content = 'foo';");
    /// ```
    fn select(self) -> SelectStatement<Self, WildCard>
    where
        Self: Selectable,
    {
        self.query(WildCard)
    }

    fn query<Q>(self, query: Q) -> SelectStatement<Self, Q>
    where
        Self: Selectable,
        Q: Query,
    {
        SelectStatement::new(self, query)
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Select, Table, ToSql};
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
    /// use typed_sql::{Select, Table, ToSql};
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
        Count<T>: Query,
    {
        self.query(Count::new(f(Default::default())))
    }

    /// ```
    /// use typed_sql::{Select, Table, ToSql};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64   
    /// }
    ///
    /// let stmt = User::table().select().filter(|user| user.id.neq(2).and(user.id.lt(5)));
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users WHERE users.id != 2 AND users.id < 5;");
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

    /// # Examples
    /// ```
    /// use typed_sql::{Table, ToSql, Select};
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
    /// use typed_sql::{Select, Table, ToSql};
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
        Self: Queryable,
        F: FnOnce(<Self::Select as Selectable>::Fields) -> O,
        O: GroupOrder,
    {
        GroupBy::new(self, f(Default::default()))
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Select, Table, ToSql};
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
    /// use typed_sql::{Select, Table, ToSql};
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
    /// use typed_sql::{Select, Table, ToSql};
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
        Self: Queryable,
        F: FnOnce(<Self::Select as Selectable>::Fields) -> O,
        O: Order,
    {
        OrderBy::new(self, f(Default::default()))
    }

    fn limit(self, limit: usize) -> Limit<Self>
    where
        Self: Queryable,
    {
        Limit::new(self, limit)
    }
}

impl<T> Select for T {}
