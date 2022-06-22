use crate::raw;
use crate::MAX_LEN_OUT;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SpiceError {
    short: String,
    long: String,
}

impl Error for SpiceError {}

impl fmt::Display for SpiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "short: {}\n\nlong: {}", self.short, self.long)
    }
}

pub fn furnsh_safe(name: &str) -> Result<(), SpiceError> {
    raw::erract("SET", MAX_LEN_OUT as i32, "RETURN");
    raw::furnsh(name);
    if raw::failed() {
        let short = raw::getmsg("SHORT", MAX_LEN_OUT as i32);
        let long = raw::getmsg("LONG", MAX_LEN_OUT as i32);
        let e = SpiceError { short, long };
        raw::reset();
        Err(e)
    } else {
        Ok(())
    }
}
