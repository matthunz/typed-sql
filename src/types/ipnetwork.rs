use super::Primitive;
use ipnetwork::IpNetwork;

impl Primitive for IpNetwork {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(&self.to_string());
        sql.push('\'');
    }
}
