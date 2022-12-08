use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),

    Nil,
    ArithmeticError,
    NumsOrStringsError,
}

impl From<bool> for Object {
    fn from(b: bool) -> Self {
        Object::Bool(b)
    }
}

impl From<String> for Object {
    fn from(s: String) -> Self {
        Object::Str(s)
    }
}

impl From<f64> for Object {
    fn from(f: f64) -> Self {
        Object::Num(f)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Bool(x) => {
                if *x {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Object::Nil => write!(f, "nil"),
            _ => panic!("Should not be trying to print this"),
        }
    }
}
