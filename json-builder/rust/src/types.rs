
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Expression<T> {
    Value(T),
    Variable(String),
}

impl<T> Expression<T> {
    pub fn value(val: T) -> Self {
        Expression::Value(val)
    }
}
