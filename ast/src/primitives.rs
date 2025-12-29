use crate::identifier_value::IdentifierValue;

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Primitives {
    Number,
    String,
    Element,
    Boolean,
}

impl ToString for Primitives {
    fn to_string(&self) -> String {
        match self {
            Primitives::Number => String::from("Number"),
            Primitives::Element => String::from("Element"),
            Primitives::String => String::from("String"),
            Primitives::Boolean => String::from("Boolean"),
        }
    }
}

impl Primitives {
    pub fn to_identifier_value(&self) -> IdentifierValue {
        match self {
            Primitives::Number => IdentifierValue::Number(None),
            Primitives::Element => IdentifierValue::Element(None),
            Primitives::String => IdentifierValue::String(None),
            Primitives::Boolean => IdentifierValue::Boolean(None),
        }
    }
}
