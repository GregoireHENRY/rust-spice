pub mod error;
use error::Kind;

use std::fmt;

#[derive(Debug)]
pub struct SpiceError {
    kind: Kind,
    short: String,
    long: String,
}

impl SpiceError {
    pub fn new(short: String, long: String) -> SpiceError {
        SpiceError {
            kind: short.as_str().into(),
            short,
            long,
        }
    }

    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }

    pub fn short(&self) -> &str {
        &self.short
    }

    pub fn long(&self) -> &str {
        &self.long
    }
}

impl std::error::Error for SpiceError {}

impl fmt::Display for SpiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const LINE: &str =
            "================================================================================";
        write!(
            f,
            "{}\n\n{}\n\n{}\n\n{}",
            LINE,
            self.short(),
            self.long(),
            LINE
        )
    }
}

impl PartialEq for SpiceError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
