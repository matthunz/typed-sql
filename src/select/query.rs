use crate::field::Field;
use crate::Table;

pub trait Query {
    fn write_query(&self, sql: &mut String);
}

pub struct WildCard;

impl Query for WildCard {
    fn write_query(&self, sql: &mut String) {
        sql.push('*');
    }
}

pub struct Count<T> {
    column: T,
}

impl<T> Count<T> {
    pub(crate) fn new(column: T) -> Self {
        Self { column }
    }
}

impl Query for Count<()> {
    fn write_query(&self, sql: &mut String) {
        write_wildcard(sql);
    }
}

impl Query for Count<WildCard> {
    fn write_query(&self, sql: &mut String) {
        write_wildcard(sql);
    }
}

impl<T: Table, A> Query for Count<Field<T, A>> {
    fn write_query(&self, sql: &mut String) {
        sql.push_str("COUNT(");
        self.column.write_field(sql);
        sql.push(')');
    }
}

fn write_wildcard(sql: &mut String) {
    sql.push_str("COUNT(*)");
}
