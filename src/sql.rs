use crate::field::Predicate;
use crate::join::{Join, JoinKind};
use crate::Table;

pub trait WriteSql {
    fn write_join<J: Join, K: JoinKind, T: Table>(&mut self);
}

impl WriteSql for String {
    fn write_join<J: Join, K: JoinKind, T: Table>(&mut self) {
        self.push(' ');
        self.push_str(K::KIND);
        self.push_str(" JOIN ");
        self.push_str(T::NAME);
        self.push_str(" ON ");
        J::join(Default::default()).write_predicate(self);
    }
}

pub trait ToSql {
    fn write_sql(&self, sql: &mut String);

    fn to_sql(&self) -> String {
        let mut sql = String::new();
        self.write_sql(&mut sql);
        sql.push(';');
        sql
    }
}
