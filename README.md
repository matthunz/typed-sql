# typed-sql

[![Latest Version](https://img.shields.io/crates/v/typed-sql.svg)](https://crates.io/crates/typed-sql)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/typed-sql)
![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)

```rust
use typed_sql::{Query, Table, ToSql};

#[derive(Table)]
struct User {
    id: i64,
    name: String
}

let stmt = User::table()
    .select()
    .filter(|user| user.id.neq(6).and(user.id.gt(3)))
    .group_by(|user| user.name)
    .order_by(|user| user.name.then(user.id.ascending()))
    .limit(5);

assert_eq!(
    stmt.to_sql(),
    "SELECT * FROM users \
    WHERE users.id != 6 AND users.id > 3 \
    GROUP BY users.name \
    ORDER BY users.name,users.id ASC \
    LIMIT 5;"
);
```
