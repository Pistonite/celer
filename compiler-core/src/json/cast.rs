use serde_json::{Value, Map};

/// The Into version of the as_type methods for non-primitive types
///
/// On error, returns the original value with the Err variant
pub trait Cast: Sized{
    type Object;
    fn try_into_object(self) -> Result<Self::Object, Self>;
    fn try_into_array(self) -> Result<Vec<Self>, Self>;
    fn try_into_string(self) -> Result<String, Self>;
}

macro_rules! cast_match {
    ($self:ident, $variant:ident) => {
        match $self {
            Value::$variant(v) => Ok(v),
            x => Err(x),
        }
    };
}

impl Cast for Value {
    type Object = Map<String, Value>;
    fn try_into_object(self) -> Result<<Value as Cast>::Object, Self> {
        cast_match!(self, Object)
    }

    fn try_into_array(self) -> Result<Vec<Self>, Self> {
        cast_match!(self, Array)
    }

    fn try_into_string(self) -> Result<String, Self> {
        cast_match!(self, String)
    }
}
