/*!
# A Rust idiomatic CSPICE wrapper built with procedural macros.
*/

spice_derive::cspice_proc!(
    /**
    Load one or more SPICE kernels into a program.
    */
    pub fn furnsh<S: Into<String>>(name: S) {}
);

spice_derive::cspice_proc!(
    /**
    Return the matrix that transforms position vectors from one specified frame to another at a
    specified epoch.
    */
    pub fn pxform<S: Into<String>>(from: S, to: S, et: f64) -> [[f64; 3]; 3] {}
);

spice_derive::cspice_proc!(
    /**
    Return the 3x3 matrix that transforms position vectors from one specified frame at a specified
    epoch to another specified frame at another specified epoch.
    */
    pub fn pxfrm2<S: Into<String>>(from: S, to: S, etfrom: f64, etto: f64) -> [[f64; 3]; 3] {}
);

spice_derive::cspice_proc!(
    /**
    Return the position of a target body relative to an observing body, optionally corrected for
    light time (planetary aberration) and stellar aberration.
    */
    pub fn spkpos<S: Into<String>>(
        targ: S,
        et: f64,
        frame: S,
        abcorr: S,
        obs: S,
    ) -> ([f64; 3], f64) {
    }
);

spice_derive::cspice_proc!(
    /**
    Convert a string representing an epoch to a double precision value representing the number of
    TDB seconds past the J2000 epoch corresponding to the input epoch.
    */
    pub fn str2et<S: Into<String>>(targ: S) -> f64 {}
);

spice_derive::cspice_proc!(
    /**
    This routine converts an input epoch represented in TDB seconds past the TDB epoch of J2000 to a
    character string formatted to the specifications of a user's format picture.
    This function has a [neat version][crate::neat::timout].
    */
    pub fn timout<S: Into<String>>(et: f64, pictur: S, lenout: usize) -> String {}
);

spice_derive::cspice_proc!(
    /**
    Unload a SPICE kernel.
    */
    pub fn unload<S: Into<String>>(name: S) {}
);
