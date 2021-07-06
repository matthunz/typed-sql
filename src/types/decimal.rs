use super::Primitive;
use rust_decimal::Decimal;

impl Primitive for Decimal {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(&self.to_string());
        sql.push('\'');
    }
}
