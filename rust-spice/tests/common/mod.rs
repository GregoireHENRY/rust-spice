/*!
A self contained set of SPICE kernels for the test suite.

## Description

The kernels are generated in a temporary directory the first time a test asks for them: the text
ones are written from the strings below, the binary ones by CSPICE itself. Nothing outside the
repository is needed, so `cargo test` works on any machine that can build the crate.

Each file is written under a name unique to the process, then moved into place, so several test
processes generating the kernels at once -- `cargo nextest` gives every test one of its own --
cannot read a half written file.

The scenario is deliberately analytic, so that the expected values can be written down rather than
recorded:

+ body 399 (`EARTH`) moves on a circular orbit of [`ORBIT_RADIUS`] around body 10 (`SUN`), and body
  301 (`MOON`) on one of [`SATELLITE_RADIUS`] around body 399, both in the plane of `J2000` and both
  described by a type 9 SPK segment. Being coplanar, the three line up twice a lunar period, which
  is what makes an occultation search find anything;
+ the shape of body 399 is, in the DSK, a regular octahedron of "radius" [`SHAPE_RADIUS`];
+ spacecraft [`SPACECRAFT`] carries instrument [`INSTRUMENT`], whose orientation is the identity
  rotation with respect to `J2000` at every epoch of the CK;
+ frame `TEST_FIXED` is `J2000` rotated by 90 degrees about its third axis.
*/

#![allow(dead_code)]

use spice::c;
use std::ffi::{CStr, CString};
use std::fs;
use std::path::PathBuf;
use std::sync::Once;

/// The fixture drives CSPICE directly, so that a bug in the wrappers cannot quietly produce the
/// very kernels that are supposed to catch it. It also keeps the fixture usable with the `lock`
/// feature, which hides the unguarded API.
fn cstring(value: &str) -> CString {
    CString::new(value).expect("interior null byte")
}

/// NAIF ID of the test spacecraft.
pub const SPACECRAFT: i32 = -999;

/// NAIF ID of the test instrument, and of its CK frame.
pub const INSTRUMENT: i32 = -999000;

/// NAIF ID of the `TEST_FIXED` frame.
pub const FIXED_FRAME: i32 = -999001;

/// Frame class ID of the frame the binary PCK describes.
pub const PCK_FRAME: i32 = -999002;

/// Body the ephemeris describes.
pub const TARGET: i32 = 399;

/// Second body of the ephemeris, orbiting [`TARGET`].
pub const SATELLITE: i32 = 301;

/// Body the ephemeris is centred on.
pub const CENTER: i32 = 10;

/// Surface ID of the octahedron in the DSK.
pub const SURFACE: i32 = 1;

/// Radius of the circular orbit of [`TARGET`] about [`CENTER`], in km.
pub const ORBIT_RADIUS: f64 = 1.495_978_7e8;

/// Radius of the circular orbit of [`SATELLITE`] about [`TARGET`], in km.
pub const SATELLITE_RADIUS: f64 = 384_400.0;

/// Gravitational parameter of body 399, in km³/s².
pub const GM_TARGET: f64 = 3.986_004_354_360_96e5;

/// Gravitational parameter of body 10, in km³/s².
pub const GM_CENTER: f64 = 1.327_124_400_419_393_8e11;

/// Distance from the centre of the octahedron to each of its vertices, in km.
pub const SHAPE_RADIUS: f64 = 6378.0;

/// First epoch covered by the generated kernels, in TDB seconds past J2000.
pub const FIRST: f64 = -20.0 * 86400.0;

/// Last epoch covered by the generated kernels, in TDB seconds past J2000.
pub const LAST: f64 = 20.0 * 86400.0;

/// Step between the states of the SPK and the records of the CK, in seconds.
pub const STEP: f64 = 3600.0;

/// An epoch comfortably inside the coverage, used by most tests.
pub const EPOCH: f64 = 86400.0;

const LSK: &str = r#"KPL/LSK

\begindata
DELTET/DELTA_T_A = 32.184
DELTET/K         = 1.657D-3
DELTET/EB        = 1.671D-2
DELTET/M         = ( 6.239996D0 1.99096871D-7 )
DELTET/DELTA_AT  = ( 10, @1972-JAN-1
                     11, @1972-JUL-1
                     12, @1973-JAN-1
                     13, @1974-JAN-1
                     14, @1975-JAN-1
                     15, @1976-JAN-1
                     16, @1977-JAN-1
                     17, @1978-JAN-1
                     18, @1979-JAN-1
                     19, @1980-JAN-1
                     20, @1981-JUL-1
                     21, @1982-JUL-1
                     22, @1983-JUL-1
                     23, @1985-JUL-1
                     24, @1988-JAN-1
                     25, @1990-JAN-1
                     26, @1991-JAN-1
                     27, @1992-JUL-1
                     28, @1993-JUL-1
                     29, @1994-JUL-1
                     30, @1996-JAN-1
                     31, @1997-JUL-1
                     32, @1999-JAN-1
                     33, @2006-JAN-1
                     34, @2009-JAN-1
                     35, @2012-JUL-1
                     36, @2015-JUL-1
                     37, @2017-JAN-1 )
\begintext
"#;

const PCK: &str = r#"KPL/PCK

\begindata
BODY399_RADII    = ( 6378.1366 6378.1366 6356.7519 )
BODY399_POLE_RA  = (    0.      -0.641        0.   )
BODY399_POLE_DEC = (   90.      -0.557        0.   )
BODY399_PM       = (  190.147  360.9856235    0.   )
BODY399_GM       = ( 3.9860043543609598E+05 )

BODY301_RADII    = ( 1737.4 1737.4 1737.4 )
BODY301_POLE_RA  = (  269.9949    0.0031    0. )
BODY301_POLE_DEC = (   66.5392    0.0130    0. )
BODY301_PM       = (   38.3213   13.17635815 -1.4D-12 )

BODY10_RADII     = ( 696000. 696000. 696000. )
BODY10_POLE_RA   = (  286.13       0.         0. )
BODY10_POLE_DEC  = (   63.87       0.         0. )
BODY10_PM        = (   84.176     14.18440    0. )
BODY10_GM        = ( 1.3271244004193938E+11 )

NAIF_SURFACE_NAME += ( 'TEST OCTAHEDRON' )
NAIF_SURFACE_CODE += ( 1 )
NAIF_SURFACE_BODY += ( 399 )
\begintext
"#;

const FK: &str = r#"KPL/FK

\begindata
NAIF_BODY_NAME += ( 'TEST_SPACECRAFT' )
NAIF_BODY_CODE += ( -999 )
NAIF_BODY_NAME += ( 'TEST_INSTRUMENT' )
NAIF_BODY_CODE += ( -999000 )

FRAME_TEST_INSTRUMENT     = -999000
FRAME_-999000_NAME        = 'TEST_INSTRUMENT'
FRAME_-999000_CLASS       = 3
FRAME_-999000_CLASS_ID    = -999000
FRAME_-999000_CENTER      = -999
CK_-999000_SCLK           = -999
CK_-999000_SPK            = -999

FRAME_TEST_FIXED          = -999001
FRAME_-999001_NAME        = 'TEST_FIXED'
FRAME_-999001_CLASS       = 4
FRAME_-999001_CLASS_ID    = -999001
FRAME_-999001_CENTER      = -999
TKFRAME_-999001_SPEC      = 'ANGLES'
TKFRAME_-999001_RELATIVE  = 'J2000'
TKFRAME_-999001_ANGLES    = ( 0.0, 0.0, 90.0 )
TKFRAME_-999001_AXES      = ( 1,   2,   3    )
TKFRAME_-999001_UNITS     = 'DEGREES'
\begintext
"#;

const SCLK: &str = r#"KPL/SCLK

\begindata
SCLK_KERNEL_ID             = ( @2000-01-01/00:00:00 )
SCLK_DATA_TYPE_999         = ( 1 )
SCLK01_TIME_SYSTEM_999     = ( 1 )
SCLK01_N_FIELDS_999        = ( 2 )
SCLK01_MODULI_999          = ( 1000000000 1000 )
SCLK01_OFFSETS_999         = ( 0 0 )
SCLK01_OUTPUT_DELIM_999    = ( 2 )
SCLK_PARTITION_START_999   = ( 0.0000000000000E+00 )
SCLK_PARTITION_END_999     = ( 9.9900000000000E+11 )
SCLK01_COEFFICIENTS_999    = ( 0.0000000000000E+00
                               {epoch:.7E}
                               1.0000000000000E+00 )
\begintext
"#;

const IK: &str = r#"KPL/IK

\begindata
INS-999000_FOV_CLASS_SPEC  = 'ANGLES'
INS-999000_FOV_SHAPE       = 'RECTANGLE'
INS-999000_FOV_FRAME       = 'TEST_INSTRUMENT'
INS-999000_BORESIGHT       = ( 0.0, 0.0, 1.0 )
INS-999000_FOV_REF_VECTOR  = ( 1.0, 0.0, 0.0 )
INS-999000_FOV_REF_ANGLE   = ( 5.0 )
INS-999000_FOV_CROSS_ANGLE = ( 5.0 )
INS-999000_FOV_ANGLE_UNITS = 'DEGREES'
\begintext
"#;

/// One tick of the test spacecraft clock, in seconds; see `SCLK01_COEFFICIENTS_999`.
pub const TICK: f64 = 1.0e-3;

/// Epoch of tick zero of the test spacecraft clock: the clock cannot run negative.
pub const TICK_EPOCH: f64 = FIRST;

/// Encoded spacecraft clock ticks matching `et`.
pub fn ticks(et: f64) -> f64 {
    (et - TICK_EPOCH) / TICK
}

static BUILD: Once = Once::new();

/// Directory holding the generated kernels.
pub fn dir() -> PathBuf {
    std::env::temp_dir().join("rust-spice-tests")
}

/// Path of one of the generated kernels.
pub fn kernel(name: &str) -> String {
    dir()
        .join(name)
        .to_str()
        .expect("non UTF-8 path")
        .to_string()
}

/// Path of the meta-kernel loading every generated kernel.
pub fn meta() -> String {
    kernel("test.tm")
}

/**
Generate the kernels, once per process, then load the meta-kernel.

The kernel pool is cleared first, so every test starts from the same state whether or not the
previous one cleaned up after itself.
*/
pub fn load() {
    reset();
    let meta = cstring(&meta());
    unsafe { c::furnsh_c(meta.as_ptr() as *mut c::SpiceChar) };
    assert_ok("loading the test meta-kernel");
}

/// Unload everything the test loaded, and clear the CSPICE error state.
pub fn unload() {
    unsafe {
        c::kclear_c();
        c::reset_c();
    }
}

/// Build the kernels if needed, then clear the kernel pool and the CSPICE error state.
pub fn reset() {
    BUILD.call_once(build);
    unload();
}

/// Panic with the CSPICE error message if the toolkit is in an error state.
pub fn assert_ok(what: &str) {
    if unsafe { c::failed_c() } == 0 {
        return;
    }
    let mut trace = vec![0 as c::SpiceChar; 1841];
    let traceback = unsafe {
        c::qcktrc_c(trace.len() as c::SpiceInt, trace.as_mut_ptr());
        CStr::from_ptr(trace.as_ptr())
            .to_string_lossy()
            .into_owned()
    };
    let message = ["SHORT", "LONG"]
        .iter()
        .map(|option| {
            let option = cstring(option);
            let mut buffer = vec![0 as c::SpiceChar; 1841];
            unsafe {
                c::getmsg_c(
                    option.as_ptr() as *mut c::SpiceChar,
                    buffer.len() as c::SpiceInt,
                    buffer.as_mut_ptr(),
                );
                CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            }
        })
        .collect::<Vec<_>>()
        .join(": ");
    unsafe { c::reset_c() };
    panic!("CSPICE failed while {what}: {message} [{traceback}]");
}

/// The analytic state of [`TARGET`] relative to [`CENTER`] at `et`, in `J2000`.
pub fn analytic_state(et: f64) -> [f64; 6] {
    let rate = (GM_CENTER / ORBIT_RADIUS.powi(3)).sqrt();
    let (sin, cos) = (rate * et).sin_cos();
    [
        ORBIT_RADIUS * cos,
        ORBIT_RADIUS * sin,
        0.0,
        -ORBIT_RADIUS * rate * sin,
        ORBIT_RADIUS * rate * cos,
        0.0,
    ]
}

/// The analytic state of [`SATELLITE`] relative to [`TARGET`] at `et`, in `J2000`.
pub fn analytic_satellite_state(et: f64) -> [f64; 6] {
    let rate = (GM_TARGET / SATELLITE_RADIUS.powi(3)).sqrt();
    let (sin, cos) = (rate * et).sin_cos();
    [
        SATELLITE_RADIUS * cos,
        SATELLITE_RADIUS * sin,
        0.0,
        -SATELLITE_RADIUS * rate * sin,
        SATELLITE_RADIUS * rate * cos,
        0.0,
    ]
}

/// Vertices of the octahedron the DSK describes, in the order the plates index them.
pub fn octahedron_vertices() -> Vec<[f64; 3]> {
    let r = SHAPE_RADIUS;
    vec![
        [r, 0.0, 0.0],
        [0.0, r, 0.0],
        [-r, 0.0, 0.0],
        [0.0, -r, 0.0],
        [0.0, 0.0, r],
        [0.0, 0.0, -r],
    ]
}

/// Plates of the octahedron, as one-based vertex indices wound counter-clockwise from outside.
pub fn octahedron_plates() -> Vec<[i32; 3]> {
    vec![
        [1, 2, 5],
        [2, 3, 5],
        [3, 4, 5],
        [4, 1, 5],
        [2, 1, 6],
        [3, 2, 6],
        [4, 3, 6],
        [1, 4, 6],
    ]
}

/// Epochs of the states in the SPK, and of the records in the CK.
pub fn epochs() -> Vec<f64> {
    let count = ((LAST - FIRST) / STEP) as usize + 1;
    (0..count)
        .map(|index| FIRST + index as f64 * STEP)
        .collect()
}

fn build() {
    // CSPICE aborts the process on error by default, which would take the test runner with it.
    let (set, ret, null) = (cstring("SET"), cstring("RETURN"), cstring("NULL"));
    let mut action = ret
        .as_bytes_with_nul()
        .iter()
        .map(|&b| b as c::SpiceChar)
        .collect::<Vec<_>>();
    let mut device = null
        .as_bytes_with_nul()
        .iter()
        .map(|&b| b as c::SpiceChar)
        .collect::<Vec<_>>();
    unsafe {
        c::erract_c(
            set.as_ptr() as *mut c::SpiceChar,
            action.len() as c::SpiceInt,
            action.as_mut_ptr(),
        );
        c::errdev_c(
            set.as_ptr() as *mut c::SpiceChar,
            device.len() as c::SpiceInt,
            device.as_mut_ptr(),
        );
    }

    fs::create_dir_all(dir()).expect("cannot create the kernel directory");

    write("naif.tls", LSK);
    write("test.tpc", PCK);
    write("test.tf", FK);
    // Tick zero is the start of the coverage, so that the clock never runs negative.
    write(
        "test.tsc",
        &SCLK.replace("{epoch:.7E}", &format!("{:.7E}", TICK_EPOCH)),
    );
    write("test.ti", IK);
    // The kernels are named through a path symbol, so that a long temporary directory does not
    // push the entries themselves past what a pool string can hold.
    let names = [
        "naif.tls", "test.tpc", "test.tf", "test.tsc", "test.ti", "test.bsp", "test.bc", "test.bds",
    ]
    .iter()
    .map(|name| format!("    '$TESTS/{name}'\n"))
    .collect::<String>();
    write(
        "test.tm",
        &format!(
            "KPL/MK\n\n\\begindata\n\
             PATH_VALUES     = (\n    {}\n)\n\
             PATH_SYMBOLS    = ( 'TESTS' )\n\
             KERNELS_TO_LOAD = (\n{})\n\
             \\begintext\n",
            pool_string(dir().to_str().expect("non UTF-8 path")),
            names
        ),
    );

    build_spk();
    build_ck();
    build_dsk();
    build_pck();
}

/// Path a file is built under before it is moved into place.
fn staging(name: &str) -> String {
    kernel(&format!(".{name}.{}", std::process::id()))
}

/**
Build a kernel under a name of this process's own, then move it into place.

A rename is atomic, so a test process that is only reading either sees the previous file or the new
one, never a partial write.
*/
fn produce(name: &str, build: impl FnOnce(&str)) {
    let staging = staging(name);
    let _ = fs::remove_file(&staging);

    build(&staging);

    fs::rename(&staging, dir().join(name))
        .unwrap_or_else(|error| panic!("cannot move {name} into place: {error}"));
}

fn write(name: &str, content: &str) {
    produce(name, |path| {
        fs::write(path, content).unwrap_or_else(|error| panic!("cannot write {name}: {error}"))
    });
}

/**
Quote `value` as a kernel pool string.

A single pool string is capped at 80 characters, which a temporary directory can easily exceed, so
anything longer is split across the continuations `furnsh` understands.
*/
fn pool_string(value: &str) -> String {
    const CHUNK: usize = 70;

    let chunks = value
        .chars()
        .collect::<Vec<_>>()
        .chunks(CHUNK)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();

    let last = chunks.len() - 1;
    chunks
        .iter()
        .enumerate()
        .map(|(index, chunk)| {
            if index == last {
                format!("'{chunk}'")
            } else {
                format!("'{chunk}+'")
            }
        })
        .collect::<Vec<_>>()
        .join("\n    ")
}

/// A type 9 SPK segment sampling the analytic circular orbit.
fn build_spk() {
    produce("test.bsp", write_spk);
}

fn write_spk(path: &str) {
    let mut epochs = epochs();
    let mut states = epochs
        .iter()
        .map(|&et| analytic_state(et))
        .collect::<Vec<_>>();
    let mut satellite = epochs
        .iter()
        .map(|&et| analytic_satellite_state(et))
        .collect::<Vec<_>>();

    let (path, ifname) = (cstring(path), cstring("rust-spice test SPK"));
    let frame = cstring("J2000");
    let (orbit, satellite_id) = (cstring("TEST ORBIT"), cstring("TEST SATELLITE"));
    let barycentre = cstring("TEST BARYCENTRE");
    // Routines such as `sincpt` reach for the state of every object relative to the solar system
    // barycentre, whatever aberration correction they are asked for, so the centre needs one too.
    let mut centre = vec![[0.0; 6]; states.len()];
    let (first, last) = (epochs[0], epochs[epochs.len() - 1]);
    let mut handle = 0;

    unsafe {
        c::spkopn_c(
            path.as_ptr() as *mut c::SpiceChar,
            ifname.as_ptr() as *mut c::SpiceChar,
            0,
            &mut handle,
        );
        c::spkw09_c(
            handle,
            TARGET,
            CENTER,
            frame.as_ptr() as *mut c::SpiceChar,
            first,
            last,
            orbit.as_ptr() as *mut c::SpiceChar,
            7,
            states.len() as c::SpiceInt,
            states.as_mut_ptr(),
            epochs.as_mut_ptr(),
        );
        c::spkw09_c(
            handle,
            SATELLITE,
            TARGET,
            frame.as_ptr() as *mut c::SpiceChar,
            first,
            last,
            satellite_id.as_ptr() as *mut c::SpiceChar,
            7,
            satellite.len() as c::SpiceInt,
            satellite.as_mut_ptr(),
            epochs.as_mut_ptr(),
        );
        c::spkw09_c(
            handle,
            CENTER,
            0,
            frame.as_ptr() as *mut c::SpiceChar,
            first,
            last,
            barycentre.as_ptr() as *mut c::SpiceChar,
            7,
            centre.len() as c::SpiceInt,
            centre.as_mut_ptr(),
            epochs.as_mut_ptr(),
        );
        c::spkcls_c(handle);
    }
    assert_ok("writing the test SPK");
}

/// A type 3 CK holding the identity rotation from `J2000` to the instrument frame.
fn build_ck() {
    produce("test.bc", write_ck);
}

fn write_ck(path: &str) {
    let mut sclkdp = epochs().iter().map(|&et| ticks(et)).collect::<Vec<_>>();
    let mut quats = vec![[1.0, 0.0, 0.0, 0.0]; sclkdp.len()];
    let mut avvs = vec![[0.0; 3]; sclkdp.len()];
    let mut starts = vec![sclkdp[0]];

    let (path, ifname) = (cstring(path), cstring("rust-spice test CK"));
    let (frame, segid) = (cstring("J2000"), cstring("TEST POINTING"));
    let mut handle = 0;

    unsafe {
        c::ckopn_c(
            path.as_ptr() as *mut c::SpiceChar,
            ifname.as_ptr() as *mut c::SpiceChar,
            0,
            &mut handle,
        );
        c::ckw03_c(
            handle,
            sclkdp[0],
            sclkdp[sclkdp.len() - 1],
            INSTRUMENT,
            frame.as_ptr() as *mut c::SpiceChar,
            1,
            segid.as_ptr() as *mut c::SpiceChar,
            sclkdp.len() as c::SpiceInt,
            sclkdp.as_mut_ptr(),
            quats.as_mut_ptr(),
            avvs.as_mut_ptr(),
            1,
            starts.as_mut_ptr(),
        );
        c::ckcls_c(handle);
    }
    assert_ok("writing the test CK");
}

/// A type 2 binary PCK holding a constant orientation.
///
/// It is deliberately left out of the meta-kernel: the tests read it as a file, which is what
/// `pckcov` and `pckfrm` take.
fn build_pck() {
    produce("test.bpc", write_pck);
}

fn write_pck(path: &str) {
    // One interval a day, each holding the degree zero Chebyshev expansion of the three Euler
    // angles, so the orientation never changes.
    let intervals = ((LAST - FIRST) / 86400.0) as i32;
    let mut cdata = (0..intervals)
        .flat_map(|_| [0.1, 0.2, 0.3])
        .collect::<Vec<f64>>();

    let (path, ifname) = (cstring(path), cstring("rust-spice test PCK"));
    let (frame, segid) = (cstring("J2000"), cstring("TEST ORIENTATION"));
    let mut handle = 0;

    unsafe {
        c::pckopn_c(
            path.as_ptr() as *mut c::SpiceChar,
            ifname.as_ptr() as *mut c::SpiceChar,
            0,
            &mut handle,
        );
        c::pckw02_c(
            handle,
            PCK_FRAME,
            frame.as_ptr() as *mut c::SpiceChar,
            FIRST,
            LAST,
            segid.as_ptr() as *mut c::SpiceChar,
            86400.0,
            intervals,
            0,
            cdata.as_mut_ptr(),
            FIRST,
        );
        c::pckcls_c(handle);
    }
    assert_ok("writing the test PCK");
}

/// A type 2 DSK holding the octahedron, in the body-fixed frame of [`TARGET`].
fn build_dsk() {
    produce("test.bds", write_dsk);
}

fn write_dsk(path: &str) {
    let mut vertices = octahedron_vertices();
    let mut plates = octahedron_plates();

    // Sized well above what eight plates need; see the `dskmi2_c` documentation.
    let mut work = vec![[0 as c::SpiceInt; 2]; 100_000];
    let mut spaixd = vec![0.0; 10];
    let mut spaixi = vec![0 as c::SpiceInt; 200_000];
    let mut corpar = [0.0; 10];

    let (path, ifname) = (cstring(path), cstring("rust-spice test DSK"));
    let frame = cstring("IAU_EARTH");
    let mut handle = 0;

    unsafe {
        c::dskmi2_c(
            vertices.len() as c::SpiceInt,
            vertices.as_mut_ptr(),
            plates.len() as c::SpiceInt,
            plates.as_mut_ptr(),
            10.0,
            4,
            work.len() as c::SpiceInt,
            10_000,
            10_000,
            1,
            spaixi.len() as c::SpiceInt,
            work.as_mut_ptr(),
            spaixd.as_mut_ptr(),
            spaixi.as_mut_ptr(),
        );
        assert_ok("making the DSK spatial index");

        c::dskopn_c(
            path.as_ptr() as *mut c::SpiceChar,
            ifname.as_ptr() as *mut c::SpiceChar,
            0,
            &mut handle,
        );
        c::dskw02_c(
            handle,
            TARGET,
            SURFACE,
            // General surface, latitudinal coordinates.
            2,
            frame.as_ptr() as *mut c::SpiceChar,
            1,
            corpar.as_mut_ptr(),
            -std::f64::consts::PI,
            std::f64::consts::PI,
            -std::f64::consts::FRAC_PI_2,
            std::f64::consts::FRAC_PI_2,
            0.0,
            SHAPE_RADIUS,
            FIRST,
            LAST,
            vertices.len() as c::SpiceInt,
            vertices.as_mut_ptr(),
            plates.len() as c::SpiceInt,
            plates.as_mut_ptr(),
            spaixd.as_mut_ptr(),
            spaixi.as_mut_ptr(),
        );
        c::dskcls_c(handle, 1);
    }
    assert_ok("writing the test DSK");
}
