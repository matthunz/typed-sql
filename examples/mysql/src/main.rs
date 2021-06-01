use sqlx::FromRow;
use sqlx::mysql::MySqlPoolOptions;
use typed_sql::{Fetch, Query, Table, Insertable, ToSql};

#[derive(Table, Insertable, FromRow, Debug)]
struct User {
    id: Option<i64>,
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let pool = MySqlPoolOptions::new()
        .connect("mysql://root:123456@127.0.0.1:3306/test")
        .await
        .unwrap();

    let user = User::table().select().fetch_one(&pool).await.unwrap();

    println!("{:?}", user);

    assert_eq!(user.id, Some(1));

    let user1 = User {
        id: None,
        name: Some(String::from("test")),
    };
    let sql = User::table().insert(user1).to_sql_unchecked();
    assert_eq!("INSERT INTO users(id,name) VALUES (NULL,'test');", sql);
}
