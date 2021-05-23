use std::fmt::Write;

pub mod bind;
pub use bind::Bind;

pub mod field;
pub use field::Field;

pub trait Primative {
    fn write_primative(&self, sql: &mut String);
}

impl Primative for String {
    fn write_primative(&self, sql: &mut String) {
        SqlString::new(self).write_primative(sql);
    }
}

impl Primative for &'_ str {
    fn write_primative(&self, sql: &mut String) {
        SqlString::new(self).write_primative(sql);
    }
}

impl Primative for i64 {
    fn write_primative(&self, sql: &mut String) {
        sql.write_fmt(format_args!("{}", self)).unwrap();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SqlStr<'s> {
    checked: &'s str
}

impl<'s> SqlStr<'s> {
    pub fn new_unchecked(s: &'s str) -> Self {
        Self { checked: s}
    }
}

impl Primative for SqlStr<'_> {
    fn write_primative(&self, sql: &mut String) {
        sql.push('\'');
        sql.push_str(&self.checked);
        sql.push('\'');
    }
}

#[derive(Clone, Debug)]
pub struct SqlString {
    checked: String
}

impl SqlString {
    /// ```
    /// use typed_sql::types::SqlString;
    ///
    /// let s = SqlString::new("\'injection");
    /// assert_eq!(s, "\\\'injection");
    /// ```
    pub fn new(unchecked: impl AsRef<str>) -> Self {
        let checked = unchecked.as_ref().replace('\'', "\\\'");
        Self::new_unchecked(checked)
    }

    pub fn new_unchecked(checked: impl Into<String>) -> Self {
        Self { checked: checked.into() }
    }
}

impl AsRef<str> for SqlString {
    fn as_ref(&self) -> &str {
        &self.checked
    }
}

impl PartialEq<&'_ str> for SqlString {
    fn eq(&self, other: &&str) -> bool {
        self.as_ref() == *other
    }
}


impl Primative for SqlString {
    fn write_primative(&self, sql: &mut String) {
        SqlStr::new_unchecked(self.as_ref()).write_primative(sql);
    }
}
