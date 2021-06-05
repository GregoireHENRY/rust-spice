/*!
Improvement on the procedurally generated functions.

## Description

The idiomatic Rust bindings to CSPICE can be very hard to generate in a procedural macro in some
specific cases. You can find, in this module, fonctions wrapped from [`raw`] to better match
an idiomatic usage. The improvements consists in functions taking:

+ a string as input in C requires to also send the size of the pointer to a char array. In Rust, you
  only send the string.
+ taking input for array size and outputing size whereas a vector can be used
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

/**
Fetch triangular plates from a type 2 DSK segment.
*/
pub fn dskp02(handle: i32, dladsc: raw::DLADSC) -> Vec<[i32; 3]> {
    let (_, np) = raw::dskz02(handle, dladsc);
    raw::dskp02(handle, dladsc, 1, np).1
}
