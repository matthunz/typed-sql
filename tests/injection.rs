use typed_sql::{Binding, Query, Table, ToSql};

#[derive(Table, Binding)]
struct User {
    name: String,
}

#[test]
fn unchecked_sql() {
    let stmt = User::table().select().filter(|user| user.name.neq("';--"));

    assert_eq!(
        stmt.to_sql_unchecked(),
        r#"SELECT * FROM users WHERE users.name != '';--';"#
    );
}

#[test]
fn binding_sql() {
    let stmt_plan = User::prepare("test_plan", |binds| {
        User::table()
            .select()
            .filter(|user| user.name.neq(binds.name))
    });

    assert_eq!(
        stmt_plan.to_sql(),
        "PREPARE test_plan AS SELECT * FROM users WHERE users.name != $1;"
    );

    let stmt = stmt_plan.execute(User {
        name: "';--".to_owned(),
    });

    assert_eq!(stmt.to_sql(), "EXECUTE test_plan('';--');");
    
    let stmt = stmt_plan.execute(User {
        name: "');--".to_owned(),
    });

    // https://www.xclusivetouch.co.uk/wp-content/uploads/2014/02/fry-futurama-not-sure-if-lying-meme.jpg
    assert_eq!(stmt.to_sql(), "EXECUTE test_plan('');--');");
}
