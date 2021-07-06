use super::Primitive;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

impl Primitive for BitVec {
    fn write_primative(&self, sql: &mut String) {
        let btcvec = serde_json::to_string(self).unwrap();

        sql.push('\'');
        sql.push_str(&btcvec);
        sql.push('\'');
    }
}
