use super::Primitive;
use uuid::Uuid;

impl Primitive for Uuid {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(&self.to_string());
        sql.push('\'');
    }
}
