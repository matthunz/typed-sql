pub trait CheckedSql {}

impl CheckedSql for i8 {}

impl CheckedSql for i16 {}

impl CheckedSql for i32 {}

impl CheckedSql for i64 {}

impl CheckedSql for u8 {}

impl CheckedSql for u16 {}

impl CheckedSql for u32 {}

impl CheckedSql for u64 {}

pub trait ToSql {
    fn write_sql_unchecked(&self, sql: &mut String);

    fn write_sql(&self, sql: &mut String)
    where
        Self: CheckedSql,
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
        Self: CheckedSql,
    {
        self.to_sql_unchecked()
    }
}
