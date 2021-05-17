use crate::field::Predicate;
use crate::join::{Join, JoinKind};
use crate::Table;

pub struct Sql {
    pub(crate) buf: String,
}

impl Sql {
    pub fn write_join<J: Join, K: JoinKind, T: Table>(&mut self) {
        self.buf.push(' ');
        self.buf.push_str(K::KIND);
        self.buf.push_str(" JOIN ");
        self.buf.push_str(T::NAME);
        self.buf.push_str(" ON ");
        J::join(Default::default()).write_predicate(&mut self.buf);
    }
}

pub trait ToSql {
    fn write_sql(&self, sql: &mut Sql);

    fn to_sql(&self) -> Sql {
        let mut sql = Sql { buf: String::new() };
        self.write_sql(&mut sql);
        sql
    }
}
