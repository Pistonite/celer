use serde_json::{Map, Value};

/// Convert a JSON value into non-primitive types (array or object)
pub trait Cast: Sized {
    type Array;
    type Object;
    type AsArray<'a>: 'a where Self: 'a;
    type AsObject<'a>: 'a where Self: 'a;
    fn try_into_array(self) -> Result<Self::Array, Self>;
    fn try_into_object(self) -> Result<Self::Object, Self>;
    fn as_array(&self) -> Option<Self::AsArray<'_>>;
    fn as_object(&self) -> Option<Self::AsObject<'_>>;
    fn as_str(&self) -> Option<&str>;
    fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
    fn is_object(&self) -> bool {
        self.as_object().is_some()
    }
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
    type Array = Vec<Self>;
    type Object = Map<String, Self>;
    type AsArray<'a> = &'a <Self as Cast>::Array;
    type AsObject<'a> = &'a <Self as Cast>::Object;
    fn try_into_object(self) -> Result<<Self as Cast>::Object, Self> {
        cast_match!(self, Object)
    }

    fn try_into_array(self) -> Result<<Self as Cast>::Array, Self> {
        cast_match!(self, Array)
    }

    fn as_array(&self) -> Option<Self::AsArray<'_>> {
        Value::as_array(self)
    }

    fn as_object(&self) -> Option<Self::AsObject<'_>> {
        Value::as_object(self)
    }

    fn as_str(&self) -> Option<&str> {
        Value::as_str(self)
    }
}
