//! Untyped `Ipld` representation.

use crate::error::*;
use crate::ipld::*;
use core::convert::TryInto;

/// Untyped `Ipld` representation.
#[derive(Clone, Debug, PartialEq)]
pub enum Ipld {
    /// Represents the absence of a value or the value undefined.
    Null,
    /// Represents a boolean value.
    Bool(IpldBool),
    /// Represents an integer.
    Integer(IpldInteger),
    /// Represents a floating point value.
    Float(IpldFloat),
    /// Represents an UTF-8 string.
    String(IpldString),
    /// Represents a sequence of bytes.
    Bytes(IpldBytes),
    // /// Represents a list.
    // List(IpldList),
    // /// Represents a map.
    // Map(IpldMap),
    /// Represents a link to an Ipld node
    Link(IpldLink),
}

impl From<IpldString> for Ipld {
    fn from(string: IpldString) -> Ipld {
        Ipld::String(string)
    }
}

impl TryInto<IpldString> for Ipld {
    type Error = IpldTypeError;

    fn try_into(self) -> Result<IpldString, Self::Error> {
        match self {
            Ipld::String(string) => Ok(string),
            _ => Err(IpldTypeError::NotString),
        }
    }
}

impl From<String> for Ipld {
    fn from(string: String) -> Ipld {
        Ipld::from(IpldString::from(string))
    }
}

impl From<&str> for Ipld {
    fn from(string: &str) -> Ipld {
        Ipld::from(IpldString::from(string))
    }
}

impl From<IpldBool> for Ipld {
    fn from(boolean: IpldBool) -> Ipld {
        Ipld::Bool(boolean)
    }
}

impl TryInto<IpldBool> for Ipld {
    type Error = IpldTypeError;

    fn try_into(self) -> Result<IpldBool, Self::Error> {
        match self {
            Ipld::Bool(boolean) => Ok(boolean),
            _ => Err(IpldTypeError::NotBool),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipld_string_from() {
        Ipld::from("a string");
        Ipld::from("a string".to_string());
        Ipld::from(IpldString::from("a string"));
        Ipld::from(IpldString::from("a string".to_string()));
    }

    #[test]
    fn from_try_into_string() {
        let string = IpldString::from("a string".to_string());
        let ipld: Ipld = string.clone().into();
        let string2: IpldString = ipld.try_into().unwrap();
        assert_eq!(string, string2);
    }
}
