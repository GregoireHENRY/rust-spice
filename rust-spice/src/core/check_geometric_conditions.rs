/*!
# SPICE functions for checking geometric conditions

## Checking for in Field-Of-View (FOV) conditions

CSpice | `rust-spice` | Description
-------|--------------|------------
[fovray_c][fovray_c link] | TODO |
[fovtrg_c][fovtrg_c link] | TODO |

## Checking for occultation conditions

CSpice | `rust-spice` | Description
-------|--------------|------------
[occult_c][occult_c link] | [`occult`] | determines the occultation condition (not occulted, partially, etc.) of one target relative to another target as seen by an observer at a given time, with targets modeled as points, ellipsoids, or digital shapes (DSK)

[fovray_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/fovray_c.html
[fovtrg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/fovtrg_c.html
[occult_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/occult_c.html
*/
use crate::cstr;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Enumeration of the possible occultation values between two bodies and an observer.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum Occultation {
    /// Total occultation of first target by second.
    FirstTotal = -3,
    /// Annular occultation of first target by second. The second target does not block the limb of
    /// the first.
    FirstAnnular,
    /// Partial occultation of first target by second target.
    FirstPartial,
    /// No occultation or transit: both objects are completely visible to the observer.
    None,
    /// Partial occultation of second target by first target.
    SecondPartial,
    /// Annular occultation of second target by first.
    SecondAnnular,
    /// Total occultation of second target by first.
    SecondTotal,
}

impl Occultation {
    fn from_i32(id: i32) -> Occultation {
        match id {
            -3 => Occultation::FirstTotal,
            -2 => Occultation::FirstAnnular,
            -1 => Occultation::FirstPartial,
            1 => Occultation::SecondPartial,
            2 => Occultation::SecondAnnular,
            3 => Occultation::SecondTotal,
            _ => Occultation::None,
        }
    }
}

/// Determines the occultation condition (not occulted, partially, etc.) of one target relative to
/// another target as seen by an observer at a given time, with targets modeled as points,
/// ellipsoids, or digital shapes (DSK)
#[allow(clippy::too_many_arguments)]
pub fn occult<S: Into<String>>(
    targ1: S,
    shape1: S,
    frame1: S,
    targ2: S,
    shape2: S,
    frame2: S,
    abcorr: S,
    obsrvr: S,
    et: f64,
) -> Occultation {
    let mut ocltid: i32 = 0;
    unsafe {
        crate::c::occult_c(
            cstr!(targ1.into()),
            cstr!(shape1.into()),
            cstr!(frame1.into()),
            cstr!(targ2.into()),
            cstr!(shape2.into()),
            cstr!(frame2.into()),
            cstr!(abcorr.into()),
            cstr!(obsrvr.into()),
            et,
            &mut ocltid,
        );
    }
    Occultation::from_i32(ocltid)
}
