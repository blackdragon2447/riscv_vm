use std::fmt::Debug;

pub struct Address(u64);

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#8x}", self.0)
    }
}

impl<T: Into<u64>> From<T> for Address {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
