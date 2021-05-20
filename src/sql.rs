pub trait ToSql {
    fn write_sql(&self, sql: &mut String);

    fn to_sql(&self) -> String {
        let mut sql = String::new();
        self.write_sql(&mut sql);
        sql.push(';');
        sql
    }
}
