use std::marker::PhantomData;

pub trait Table {
    const NAME: &'static str;

    type Fields: Default;

    fn table() -> TableQuery<Self> {
        TableQuery { table: PhantomData }
    }
}

pub trait TableQueryable {
    type Table: Table + ?Sized;
}

pub struct TableQuery<T: ?Sized> {
    table: PhantomData<T>,
}

impl<T: Table + ?Sized> TableQueryable for TableQuery<T> {
    type Table = T;
}

impl<T: ?Sized> Clone for TableQuery<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for TableQuery<T> {}
