use crate::query::select::{SelectStatement, Selectable, WildCard};
use crate::{CheckedSql, ToSql};
use sqlx::{Database, Error, Executor, FromRow};
use std::future::Future;
use std::pin::Pin;

pub trait Fetch<'c, 'out, E>: ToSql + CheckedSql
where
    E: Executor<'c> + 'out,
{
    type Output: for<'r> FromRow<'r, <E::Database as Database>::Row>;

    fn fetch_one(
        &self,
        exec: E,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output, Error>> + Send + 'out>> {
        let sql = self.to_sql();
        Box::pin(async move {
            exec.fetch_one(sql.as_ref())
                .await
                .and_then(|row| FromRow::from_row(&row))
        })
    }

    fn fetch_optional(
        &self,
        exec: E,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Self::Output>, Error>> + Send + 'out>> {
        let sql = self.to_sql();
        Box::pin(async move {
            exec.fetch_optional(sql.as_ref())
                .await
                .and_then(|row| row.as_ref().map(FromRow::from_row).transpose())
        })
    }
}

impl<'c, 'out, E, S> Fetch<'c, 'out, E> for SelectStatement<S, WildCard>
where
    E: Executor<'c> + 'out,
    S: Selectable,
    S::Table: for<'r> FromRow<'r, <E::Database as Database>::Row>,
{
    type Output = S::Table;
}
