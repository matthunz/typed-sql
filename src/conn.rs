use crate::{select::Queryable, ToSql};
use async_trait::async_trait;

pub trait Row {
    type Error;
}

#[async_trait(?Send)]
pub trait Connection {
    type Row: Row;
    type Error: From<<Self::Row as Row>::Error>;

    async fn query(&mut self, sql: impl ToSql) -> Result<Self::Row, Self::Error>;
}

pub trait FromRow<R: Row>: Sized {
    fn from_row(row: &R) -> Result<Self, R::Error>;
}

#[async_trait(?Send)]
pub trait Load<C: Connection>: ToSql + Sized {
    type Output: FromRow<C::Row>;

    async fn load(self, conn: &mut C) -> Result<Self::Output, C::Error> {
        conn.query(self)
            .await
            .and_then(|row| FromRow::from_row(&row).map_err(Into::into))
    }
}

impl<C, Q> Load<C> for Q
where
    C: Connection,
    Q: Queryable,
    Q::Query: FromRow<C::Row>,
{
    type Output = Q::Query;
}
