/*!
Improvement on the procedurally generated functions.

## Description

The idiomatic Rust bindings to CSPICE can be very hard to generate in a procedural macro in some
specific cases. You can find, in this module, fonctions wrapped from [`raw`] to better match
an idiomatic usage. The improvements consists in:

+ a function taking a string as input in C requires to also send the size of the pointer to a char
array. In Rust, you only send the string.
*/

use crate::raw;

/**
This routine converts an input epoch represented in TDB seconds past the TDB epoch of J2000 to a
character string formatted to the specifications of a user's format picture.
*/
pub fn timout<S>(et: f64, pictur: S) -> String
where
    S: Into<String>,
{
    let pictur_ = pictur.into();
    raw::timout(et, pictur_.clone(), pictur_.len())
}
