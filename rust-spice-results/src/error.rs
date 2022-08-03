use std::fmt;

#[derive(Debug)]
pub struct SpiceError {
    pub kind: Kind,
    pub long: String,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    EmptyString,
    NoSuchFile,
    UnknownFrame,
    IdCodeNotFound,
    UnknownError,
}

impl From<String> for Kind {
    fn from(short_err: String) -> Self {
        match short_err.as_str() {
            "SPICE(NOSUCHFILE)" => Kind::NoSuchFile,
            "SPICE(EMPTYSTRING)" => Kind::EmptyString,
            "SPICE(UNKNOWNFRAME)" => Kind::UnknownFrame,
            "SPICE(IDCODENOTFOUND)" => Kind::IdCodeNotFound,
            _ => Kind::UnknownError,
        }
    }
}

impl std::error::Error for SpiceError {}

impl fmt::Display for SpiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long)
    }
}
