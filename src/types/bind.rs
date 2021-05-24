use crate::sql::Prepared;

#[derive(Debug, Clone, Copy)]
pub struct Bind {
    pub n: u8,
}

impl Prepared for Bind {}

#[derive(Debug)]
pub struct Binder {
    counter: u8,
}

impl Default for Binder {
    fn default() -> Self {
        Self { counter: 1 }
    }
}

impl Binder {
    pub fn bind(&mut self) -> Bind {
        let n = self.counter;
        self.counter += 1;
        Bind { n }
    }
}

pub trait Binding {
    type Bindings;

    fn bindings(binder: &mut Binder) -> Self::Bindings;

    fn write_types(sql: &mut String);

    fn write_values(&self, sql: &mut String);
}
