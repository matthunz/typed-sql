use crate::field::Predicate;
use crate::{Join, Table, ToSql};
use std::marker::PhantomData;

pub mod group;
use group::{GroupBy, GroupOrder};

pub mod order;
use order::{Order, OrderBy};

pub struct WildCard;

pub trait Queryable {
    fn write_query(sql: &mut String);
}

impl Queryable for WildCard {
    fn write_query(sql: &mut String) {
        sql.push('*');
    }
}

pub trait Select: Join {
    /// ```
    /// use typed_sql::{Table, Select, ToSql};
    ///
    /// #[derive(Table)]
    /// struct User {
    ///     id: i64
    /// }
    ///
    /// let stmt = User::select();
    /// assert_eq!(stmt.to_sql(), "SELECT * FROM users;");
    /// ```
    fn select() -> SelectStatement<Self, WildCard>
    where
        Self: Sized,
    {
        SelectStatement {
            from: PhantomData,
            query: PhantomData,
        }
    }
}

impl<J: Join> Select for J {}

pub struct SelectStatement<S, Q> {
    from: PhantomData<S>,
    query: PhantomData<Q>,
}

impl<S: Select, Q> SelectStatement<S, Q> {
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
    S: Select,
    Q: Queryable,
{
    fn write_sql(&self, sql: &mut String) {
        sql.push_str("SELECT ");
        Q::write_query(sql);
        sql.push_str(" FROM ");
        sql.push_str(S::Table::NAME);
        S::write_join(sql);
    }
}

pub struct Filter<S, Q, P> {
    select: SelectStatement<S, Q>,
    predicate: P,
}

impl<S, Q, P> ToSql for Filter<S, Q, P>
where
    S: Select,
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
    type Select: Select;

    /// # Examples
    /// ```
    /// use typed_sql::{Select, Table, ToSql, QueryDsl};
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
    /// use typed_sql::{Select, Table, ToSql, QueryDsl};
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
        F: FnOnce(<Self::Select as Join>::Fields) -> O,
        O: GroupOrder,
    {
        GroupBy::new(self, f(Default::default()))
    }

    /// # Examples
    /// ```
    /// use typed_sql::{Select, Table, ToSql, QueryDsl};
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
    /// use typed_sql::{Select, Table, ToSql, QueryDsl};
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
    /// use typed_sql::{Select, Table, ToSql, QueryDsl};
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
        F: FnOnce(<Self::Select as Join>::Fields) -> O,
        O: Order,
    {
        OrderBy::new(self, f(Default::default()))
    }
}

impl<S, Q> QueryDsl for SelectStatement<S, Q>
where
    S: Select,
    Q: Queryable,
{
    type Select = S;
}

impl<S, Q, P> QueryDsl for Filter<S, Q, P>
where
    S: Select,
    Q: Queryable,
    P: Predicate,
{
    type Select = S;
}
