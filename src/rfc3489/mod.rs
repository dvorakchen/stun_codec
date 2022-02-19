//! [RFC 3489] specific components.
//!
//! [RFC 3489]: https://datatracker.ietf.org/doc/html/rfc3489
use self::attributes::*;

pub mod attributes;

define_attribute_enums!(
    Attribute,
    AttributeDecoder,
    AttributeEncoder,
    [ChangedAddress]
);
