pub trait Prepared {}

pub trait ToSql {
    fn write_sql_unchecked(&self, sql: &mut String);

    fn write_sql(&self, sql: &mut String)
    where
        Self: Prepared,
    {
        self.write_sql_unchecked(sql);
    }

    fn to_sql_unchecked(&self) -> String {
        let mut sql = String::new();
        self.write_sql_unchecked(&mut sql);
        sql.push(';');
        sql
    }

    fn to_sql(&self) -> String
    where
        Self: Prepared,
    {
        self.to_sql_unchecked()
    }
}
