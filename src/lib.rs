#![feature(min_type_alias_impl_trait)]

pub mod field;

pub mod join;
pub use join::Join;

pub mod select;
pub use select::Select;

mod sql;
pub use sql::{Sql, ToSql};

pub trait Table {
    const NAME: &'static str;

    type Fields: Default;
}

#[cfg(test)]
mod tests {
    use super::field::*;
    use super::join::*;
    use super::*;

    struct User {}

    struct UserFields {
        id: Field<User, i64>,
    }

    impl Default for UserFields {
        fn default() -> Self {
            Self {
                id: Field::new("id"),
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

        fn write_join(sql: &mut Sql) {
            sql.write_join::<Self, Inner, Post>()
        }
    }

    #[derive(Default)]
    struct UserPostFields {
        user: UserFields,
        post: PostFields,
    }

    #[test]
    fn it_works() {
        dbg!(User::select().to_sql().buf);
        dbg!(UserPost::select().to_sql().buf);
    }
}
