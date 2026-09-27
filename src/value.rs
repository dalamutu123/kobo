// GIVEN: do not edit. Read it anyway; what you write has to fit it.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    Str(String),
    /// Named by its index in `Program::functions`. The name is carried too, so that
    /// `print f;` can say `<fn f>`.
    Function {
        index: usize,
        name: String,
    },
}

impl Value {
    /// `false` and `nil` are false; everything else, `0` and `""` included, is true.
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Nil | Value::Bool(false))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            // Rust's f64 Display is exactly spec section 2.2: shortest round-trip, no `.0`.
            Value::Number(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Function { name, .. } => write!(f, "<fn {}>", name),
        }
    }
}
