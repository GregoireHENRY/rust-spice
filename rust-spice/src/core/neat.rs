/*!
Improvement on the procedurally generated functions.
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
