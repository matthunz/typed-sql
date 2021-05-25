use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use typed_sql::{Fetch, Query, Table};

#[derive(Table, FromRow)]
struct User {
    id: i64,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test")
        .await
        .unwrap();

    let user = User::table().select().fetch_one(&pool).await.unwrap();

    assert_eq!(user.id, 2);
}
