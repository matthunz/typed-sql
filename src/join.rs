use crate::field::Predicate;
use crate::Table;

pub struct Inner;

pub trait JoinKind {
    const KIND: &'static str;
}

impl JoinKind for Inner {
    const KIND: &'static str = "INNER";
}

pub trait Join {
    type Table: Table;
    type Fields: Default;
    type Predicate: Predicate;

    fn join(joined: Self::Fields) -> Self::Predicate;

    fn write_join(sql: &mut String);
}

impl<T: Table> Join for T {
    type Table = T;
    type Fields = T::Fields;
    type Predicate = ();

    fn join(_joined: Self::Fields) -> Self::Predicate {}

    fn write_join(_sql: &mut String) {}
}
