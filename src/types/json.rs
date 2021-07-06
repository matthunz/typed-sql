use super::Primitive;
use serde_json::Value;

impl Primitive for Value {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(&self.to_string());
        sql.push('\'');
    }
}
