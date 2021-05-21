use crate::types::Field;
use crate::Table;

pub trait Queryable {
    fn write_query(&self, sql: &mut String);
}

pub struct WildCard;

impl Queryable for WildCard {
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

impl Queryable for Count<()> {
    fn write_query(&self, sql: &mut String) {
        write_wildcard(sql);
    }
}

impl Queryable for Count<WildCard> {
    fn write_query(&self, sql: &mut String) {
        write_wildcard(sql);
    }
}

impl<T: Table, A> Queryable for Count<Field<T, A>> {
    fn write_query(&self, sql: &mut String) {
        sql.push_str("COUNT(");
        self.column.write_field(sql);
        sql.push(')');
    }
}

fn write_wildcard(sql: &mut String) {
    sql.push_str("COUNT(*)");
}
