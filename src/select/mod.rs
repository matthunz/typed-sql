use crate::join::JoinSelect;
use crate::{Table, ToSql};
use std::marker::PhantomData;

pub mod group;
use group::{GroupBy, GroupOrder};

pub mod order;
use order::{Order, OrderBy};

pub mod predicate;
pub use predicate::Predicate;

pub mod query;
use query::Queryable;
pub use query::WildCard;

pub trait Selectable {
    type Table: Table;
    type Fields: Default;

    fn write_join(&self, sql: &mut String);
}

impl<T> Selectable for PhantomData<T>
where
    T: Table,
{
    type Table = T;
    type Fields = T::Fields;

    fn write_join(&self, _sql: &mut String) {}
}

impl<J: JoinSelect> Selectable for J {
    type Table = J::Table;
    type Fields = J::Fields;

    fn write_join(&self, sql: &mut String) {
        self.write_join_select(sql);
    }
}

pub struct SelectStatement<S, Q> {
    from: S,
    query: Q,
}

impl<S: Selectable, Q> SelectStatement<S, Q> {
    pub fn new(from: S, query: Q) -> Self {
        Self { from, query }
    }

    pub fn filter<F, P>(self, f: F) -> Filter<S, Q, P>
    where
        F: FnOnce(S::Fields) -> P,
    {
        Filter {
            select: self,
            predicate: f(Default::default()),
        }
    }
}

impl<S, Q> ToSql for SelectStatement<S, Q>
where
    S: Selectable,
    Q: Queryable,
{
    fn write_sql(&self, sql: &mut String) {
        sql.push_str("SELECT ");
        self.query.write_query(sql);
        sql.push_str(" FROM ");
        sql.push_str(S::Table::NAME);
        self.from.write_join(sql);
    }
}

pub struct Filter<S, Q, P> {
    select: SelectStatement<S, Q>,
    predicate: P,
}

impl<S, Q, P> ToSql for Filter<S, Q, P>
where
    S: Selectable,
    Q: Queryable,
    P: Predicate,
{
    fn write_sql(&self, sql: &mut String) {
        self.select.write_sql(sql);
        sql.push_str(" WHERE ");
        self.predicate.write_predicate(sql);
    }
}

pub trait QueryDsl: ToSql {
    type Select: Selectable;

    /// # Examples
    /// ```
    /// use typed_sql::{Table, ToSql, QueryDsl};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64
    /// }
    ///
    /// let stmt = User::select().group_by(|user| user.id);
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users GROUP BY users.id;");
    /// ```
    /// ## Multiple columns
    /// ```
    /// use typed_sql::{Table, ToSql, QueryDsl};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64,
    ///     name: String
    /// }
    ///
    /// let stmt = User::select().group_by(|user| user.id.then(user.name));
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users GROUP BY users.id,users.name;");
    /// ```
    fn group_by<F, O>(self, f: F) -> GroupBy<Self, O>
    where
        Self: Sized,
        F: FnOnce(<Self::Select as Selectable>::Fields) -> O,
        O: GroupOrder,
    {
        GroupBy::new(self, f(Default::default()))
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Table, ToSql, QueryDsl};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64,
    ///     name: String
    /// }
    ///
    /// let stmt = User::select().order_by(|user| user.id);
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users ORDER BY users.id;");
    /// ```
    /// ## Direction
    /// ```
    /// use typed_sql::{Table, ToSql, QueryDsl};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64
    /// }
    ///
    /// let stmt = User::select().order_by(|user| user.id.ascending());
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users ORDER BY users.id ASC;");
    /// ```
    /// ## Multiple columns
    /// ```
    /// use typed_sql::{Table, ToSql, QueryDsl};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64,
    ///     name: String
    /// }
    ///
    /// let stmt = User::select()
    ///     .order_by(|user| user.id.ascending().then(user.name.descending()));
    ///
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users ORDER BY users.id ASC,users.name DESC;");
    /// ```
    fn order_by<F, O>(self, f: F) -> OrderBy<Self, O>
    where
        Self: Sized,
        F: FnOnce(<Self::Select as Selectable>::Fields) -> O,
        O: Order,
    {
        OrderBy::new(self, f(Default::default()))
    }
}

impl<S, Q> QueryDsl for SelectStatement<S, Q>
where
    S: Selectable,
    Q: Queryable,
{
    type Select = S;
}

impl<S, Q, P> QueryDsl for Filter<S, Q, P>
where
    S: Selectable,
    Q: Queryable,
    P: Predicate,
{
    type Select = S;
}
