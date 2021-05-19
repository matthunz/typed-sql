#![feature(min_type_alias_impl_trait)]

pub mod bind;
pub use bind::Binding;

pub mod field;

pub mod insert;
pub use insert::Insert;

pub mod join;
pub use join::Join;

pub mod select;
pub use select::{QueryDsl, Select};

mod sql;
pub use sql::{ToSql, WriteSql};

pub trait Table {
    const NAME: &'static str;

    type Fields: Default;
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use crate::insert::Insertable;

    use super::bind::{Bind, Binder};
    use super::field::*;
    use super::join::*;
    use super::*;

    struct User {
        id: i64,
        name: String,
    }

    impl Insertable for User {
        fn write_columns(sql: &mut String) {
            sql.push_str("id");
        }

        fn write_values(&self, sql: &mut String) {
            sql.write_fmt(format_args!("{}", self.id)).unwrap();
        }
    }

    struct UserFields {
        id: Field<User, i64>,
        name: Field<User, String>,
        email: Field<User, String>,
    }

    impl Default for UserFields {
        fn default() -> Self {
            Self {
                id: Field::new("id"),
                name: Field::new("name"),
                email: Field::new("email"),
            }
        }
    }

    impl Table for User {
        const NAME: &'static str = "users";

        type Fields = UserFields;
    }

    struct Post {}

    struct PostFields {
        id: Field<Post, i64>,
    }

    impl Default for PostFields {
        fn default() -> Self {
            Self {
                id: Field::new("id"),
            }
        }
    }

    impl Table for Post {
        const NAME: &'static str = "posts";

        type Fields = PostFields;
    }

    struct UserPost {
        user: User,
        post: Post,
    }

    impl Join for UserPost {
        type Table = User;
        type Fields = UserPostFields;
        type Predicate = impl Predicate;

        fn join(joined: Self::Fields) -> Self::Predicate {
            joined.user.id.eq(joined.post.id)
        }

        fn write_join(sql: &mut String) {
            sql.write_join::<Self, Inner, Post>()
        }
    }

    #[derive(Default)]
    struct UserPostFields {
        user: UserFields,
        post: PostFields,
    }

    struct UserBindings {
        id: Bind,
    }

    impl Binding for User {
        type Bindings = UserBindings;

        fn write_types(sql: &mut String) {
            sql.push_str("int");
        }

        fn write_values(&self, sql: &mut String) {
            sql.write_fmt(format_args!("{}", self.id)).unwrap();
        }

        fn bindings(binder: &mut Binder) -> Self::Bindings {
            UserBindings { id: binder.bind() }
        }
    }

    #[test]
    fn it_works() {
        dbg!(User::select().to_sql());
        dbg!(UserPost::select().to_sql());

        let stmt = User::prepare("idplan", |binds| {
            User::select()
                .filter(|user| user.id.eq(binds.id))
                .group_by(|user| user.id.then(user.name).then(user.email))
                .order_by(|user| {
                    user.id
                        .then(user.name.ascending())
                        .then(user.email.descending())
                })
        });
        dbg!(stmt.to_sql());
    }
}
