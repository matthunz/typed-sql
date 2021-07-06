use super::Primitive;
use bigdecimal_::BigDecimal;

impl Primitive for BigDecimal {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(&self.to_string());
        sql.push('\'');
    }
}
