/*!
The wrappers, checked against the C routines they call.

Where the tests elsewhere assert *what* a routine computes, these assert that the Rust wrapper hands
CSPICE exactly what the C caller would, and reads exactly what it wrote back: same arguments, same
buffers, same conventions. Every shape the marshalling layer knows about is covered — scalars,
booleans, strings in and out, fixed size vectors and matrices, slices, descriptors, cells, and the
routines that return their value instead of writing it through a pointer.
*/

use crate::common;
use crate::cptr;
use crate::{assert_slice_eq, cs};
use spice::c;

#[test]
#[serial]
fn scalars_and_strings() {
    common::load();

    let date = "2000-JAN-02 04:05:06.789";

    // str2et: a string in, a double out.
    let mut expected = 0.0;
    let input = cs(date);
    unsafe { c::str2et_c(cptr!(input), &mut expected) };
    assert_eq!(spice::str2et(date), expected);

    // timout: a double and a string in, a string out.
    let picture = cs(spice::TIME_FORMAT);
    let mut buffer = vec![0 as c::SpiceChar; 64];
    unsafe {
        c::timout_c(
            expected,
            cptr!(picture),
            buffer.len() as c::SpiceInt,
            buffer.as_mut_ptr(),
        )
    };
    let expected_string = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    assert_eq!(
        spice::raw::timout(expected, spice::TIME_FORMAT, 64),
        expected_string
    );

    common::unload();
}

#[test]
#[serial]
fn strings_and_flags_out() {
    common::load();

    // bodc2n: an int in, a string and a flag out.
    let mut buffer = vec![0 as c::SpiceChar; spice::MAX_LEN_OUT];
    let mut found = 0;
    unsafe {
        c::bodc2n_c(
            399,
            buffer.len() as c::SpiceInt,
            buffer.as_mut_ptr(),
            &mut found,
        )
    };
    let expected = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    assert_eq!(spice::bodc2n(399), (expected, found != 0));

    // bodn2c: a string in, an int and a flag out.
    let name = cs("EARTH");
    let (mut code, mut found) = (0, 0);
    unsafe { c::bodn2c_c(cptr!(name), &mut code, &mut found) };
    assert_eq!(spice::bodn2c("EARTH"), (code, found != 0));

    common::unload();
}

#[test]
#[serial]
fn direct_returns() {
    common::load();

    // A double the C routine returns rather than writes.
    let (mut a, mut b) = ([1.0, 2.0, 3.0], [-4.0, 5.0, -6.0]);
    let expected = unsafe { c::vdot_c(a.as_mut_ptr(), b.as_mut_ptr()) };
    assert_eq!(spice::vdot(a, b), expected);
    assert_eq!(spice::vsep(a, b), unsafe {
        c::vsep_c(a.as_mut_ptr(), b.as_mut_ptr())
    });
    assert_eq!(spice::vnorm(a), unsafe { c::vnorm_c(a.as_mut_ptr()) });

    // A boolean the C routine returns as a `SpiceBoolean`.
    let item = cs("RADII");
    assert_eq!(spice::bodfnd(399, "RADII"), unsafe {
        c::bodfnd_c(399, cptr!(item)) != 0
    });
    let missing = cs("NO_SUCH_ITEM");
    assert_eq!(spice::bodfnd(399, "NO_SUCH_ITEM"), unsafe {
        c::bodfnd_c(399, cptr!(missing)) != 0
    });

    // A string the C routine returns as a pointer to its own storage.
    let item = cs("TOOLKIT");
    let expected = unsafe {
        std::ffi::CStr::from_ptr(c::tkvrsn_c(cptr!(item)))
            .to_string_lossy()
            .into_owned()
    };
    assert_eq!(spice::tkvrsn("TOOLKIT"), expected);

    // And a constant, taking no argument at all.
    assert_eq!(spice::dpr(), unsafe { c::dpr_c() });

    common::unload();
}

#[test]
#[serial]
fn vectors_and_matrices() {
    common::load();

    let et = common::EPOCH;

    // spkpos: a vector and a double out.
    let (target, frame, abcorr, observer) = (cs("EARTH"), cs("J2000"), cs("NONE"), cs("SUN"));
    let mut position = [0.0; 3];
    let mut light_time = 0.0;
    unsafe {
        c::spkpos_c(
            cptr!(target),
            et,
            cptr!(frame),
            cptr!(abcorr),
            cptr!(observer),
            position.as_mut_ptr(),
            &mut light_time,
        )
    };
    let (wrapped_position, wrapped_light_time) = spice::spkpos("EARTH", et, "J2000", "NONE", "SUN");
    assert_eq!(wrapped_position, position);
    assert_eq!(wrapped_light_time, light_time);

    // pxform: a 3x3 matrix out.
    let (from, to) = (cs("J2000"), cs("IAU_EARTH"));
    let mut matrix = [[0.0; 3]; 3];
    unsafe { c::pxform_c(cptr!(from), cptr!(to), et, matrix.as_mut_ptr()) };
    assert_eq!(spice::pxform("J2000", "IAU_EARTH", et), matrix);

    // sxform: a 6x6 matrix out, to check the row stride of the larger case.
    let mut state = [[0.0; 6]; 6];
    unsafe { c::sxform_c(cptr!(from), cptr!(to), et, state.as_mut_ptr()) };
    assert_eq!(spice::sxform("J2000", "IAU_EARTH", et), state);

    // mxv: a matrix and a vector in, a vector out.
    let mut vector = [1.0, 2.0, 3.0];
    let mut product = [0.0; 3];
    unsafe {
        c::mxv_c(
            matrix.as_mut_ptr(),
            vector.as_mut_ptr(),
            product.as_mut_ptr(),
        )
    };
    assert_eq!(spice::mxv(matrix, vector), product);

    // conics: an eight element array in, a six element state out.
    let mut elements = [1.0e8, 0.1, 0.2, 0.3, 0.4, 0.5, 0.0, common::GM_CENTER];
    let mut propagated = [0.0; 6];
    unsafe { c::conics_c(elements.as_mut_ptr(), et, propagated.as_mut_ptr()) };
    assert_eq!(spice::conics(elements, et), propagated);

    common::unload();
}

#[test]
#[serial]
fn descriptors() {
    common::load();

    let path = cs(&common::kernel("test.bds"));
    let mut handle = 0;
    let mut dladsc = std::mem::MaybeUninit::<c::SpiceDLADescr>::zeroed();
    let mut found = 0;
    let mut dskdsc = std::mem::MaybeUninit::<c::SpiceDSKDescr>::zeroed();

    let (wrapped_handle, wrapped_dladsc, wrapped_found, wrapped_dskdsc) = {
        let handle = spice::dasopr(&common::kernel("test.bds"));
        let (dladsc, found) = spice::dlabfs(handle);
        let dskdsc = spice::dskgd(handle, dladsc);
        spice::dascls(handle);
        (handle, dladsc, found, dskdsc)
    };

    unsafe {
        c::dasopr_c(cptr!(path), &mut handle);
        c::dlabfs_c(handle, dladsc.as_mut_ptr(), &mut found);
        c::dskgd_c(handle, dladsc.as_mut_ptr(), dskdsc.as_mut_ptr());
        c::dascls_c(handle);
    }
    let dladsc = unsafe { dladsc.assume_init() };
    let dskdsc = unsafe { dskdsc.assume_init() };

    assert_eq!(wrapped_found, found != 0);
    assert_eq!(wrapped_handle, handle, "the same file gets the same handle");
    assert_eq!(wrapped_dladsc.ibase, dladsc.ibase);
    assert_eq!(wrapped_dladsc.dbase, dladsc.dbase);
    assert_eq!(wrapped_dladsc.csize, dladsc.csize);
    assert_eq!(wrapped_dskdsc.surfce, dskdsc.surfce);
    assert_eq!(wrapped_dskdsc.center, dskdsc.center);
    assert_eq!(wrapped_dskdsc.corsys, dskdsc.corsys);
    assert_slice_eq(&wrapped_dskdsc.corpar, &dskdsc.corpar, 0.0);
    assert_eq!(wrapped_dskdsc.co3max, dskdsc.co3max);

    common::unload();
}

#[test]
#[serial]
fn arrays_of_arrays() {
    common::load();

    let handle = spice::dasopr(&common::kernel("test.bds"));
    let (dladsc, _) = spice::dlabfs(handle);

    // dskv02 and dskp02 read arrays of three component records.
    let mut raw_dladsc = dladsc;
    let mut vertices = vec![[0.0; 3]; 16];
    let mut count = 0;
    unsafe {
        c::dskv02_c(
            handle,
            &mut raw_dladsc,
            1,
            16,
            &mut count,
            vertices.as_mut_ptr(),
        );
    }
    vertices.truncate(count as usize);
    assert_eq!(spice::dskv02(handle, dladsc), vertices);

    let mut plates = vec![[0; 3]; 16];
    let mut count = 0;
    unsafe {
        c::dskp02_c(
            handle,
            &mut raw_dladsc,
            1,
            16,
            &mut count,
            plates.as_mut_ptr(),
        );
    }
    plates.truncate(count as usize);
    assert_eq!(spice::dskp02(handle, dladsc), plates);

    spice::dascls(handle);
    common::unload();
}

#[test]
#[serial]
fn cells() {
    common::load();

    // The wrapper's cell has to be laid out exactly as the one CSPICE declares for itself.
    let path = cs(&common::kernel("test.bsp"));
    let mut data = vec![0 as c::SpiceInt; 6 + 64];
    let mut cell = c::SpiceCell {
        dtype: c::_SpiceDataType_SPICE_INT,
        length: 0,
        size: 64,
        card: 0,
        isSet: 1,
        adjust: 0,
        init: 0,
        base: data.as_mut_ptr().cast(),
        data: unsafe { data.as_mut_ptr().add(6) }.cast(),
    };
    unsafe { c::spkobj_c(cptr!(path), &mut cell) };
    let expected = (0..cell.card as usize)
        .map(|index| data[6 + index])
        .collect::<Vec<_>>();

    let mut wrapped = spice::Cell::<i32>::new(64);
    spice::raw::spkobj(&common::kernel("test.bsp"), &mut wrapped);
    assert_eq!(wrapped.card, cell.card);
    assert_eq!(wrapped.to_vec(), expected);

    // Coverage windows are cells of doubles.
    let mut data = vec![0.0; 6 + 64];
    let mut cell = c::SpiceCell {
        dtype: c::_SpiceDataType_SPICE_DP,
        length: 0,
        size: 64,
        card: 0,
        isSet: 1,
        adjust: 0,
        init: 0,
        base: data.as_mut_ptr().cast(),
        data: unsafe { data.as_mut_ptr().add(6) }.cast(),
    };
    unsafe { c::spkcov_c(cptr!(path), common::TARGET, &mut cell) };
    let expected = (0..cell.card as usize)
        .map(|index| data[6 + index])
        .collect::<Vec<_>>();
    assert_eq!(
        spice::spkcov(&common::kernel("test.bsp"), common::TARGET).to_vec(),
        expected
    );

    common::unload();
}

#[test]
#[serial]
fn slices_in() {
    common::load();

    // spkw09 takes the states and the epochs as arrays; write the same segment twice, once
    // through each interface, and compare what comes back out.
    let epochs = (0..12)
        .map(|index| index as f64 * 120.0)
        .collect::<Vec<f64>>();
    let states = epochs
        .iter()
        .map(|&et| [et, -et, 2.0 * et, 1.0, -1.0, 2.0])
        .collect::<Vec<[f64; 6]>>();

    let through_c = common::kernel("through_c.bsp");
    let through_rust = common::kernel("through_rust.bsp");
    for path in [&through_c, &through_rust] {
        let _ = std::fs::remove_file(path);
    }

    let (path, ifname) = (cs(&through_c), cs("through C"));
    let (frame, segid) = (cs("J2000"), cs("COMPARISON"));
    let mut handle = 0;
    let (mut c_states, mut c_epochs) = (states.clone(), epochs.clone());
    unsafe {
        c::spkopn_c(cptr!(path), cptr!(ifname), 0, &mut handle);
        c::spkw09_c(
            handle,
            common::SPACECRAFT,
            common::TARGET,
            cptr!(frame),
            c_epochs[0],
            c_epochs[c_epochs.len() - 1],
            cptr!(segid),
            3,
            c_states.len() as c::SpiceInt,
            c_states.as_mut_ptr(),
            c_epochs.as_mut_ptr(),
        );
        c::spkcls_c(handle);
    }

    let handle = spice::spkopn(&through_rust, "through Rust", 0);
    spice::spkw09(
        handle,
        common::SPACECRAFT,
        common::TARGET,
        "J2000",
        epochs[0],
        epochs[epochs.len() - 1],
        "COMPARISON",
        3,
        states.len() as i32,
        &states,
        &epochs,
    );
    spice::spkcls(handle);
    common::assert_ok("writing through the wrapper");

    // Both files describe the same trajectory.
    let et = 500.0;
    spice::furnsh(&through_c);
    let (from_c, _) = spice::spkezr("TEST_SPACECRAFT", et, "J2000", "NONE", "EARTH");
    spice::kclear();

    spice::furnsh(&common::meta());
    spice::furnsh(&through_rust);
    let (from_rust, _) = spice::spkezr("TEST_SPACECRAFT", et, "J2000", "NONE", "EARTH");
    common::assert_ok("reading both segments back");
    assert_eq!(from_c, from_rust);

    common::unload();
    for path in [&through_c, &through_rust] {
        let _ = std::fs::remove_file(path);
    }
}

#[test]
#[serial]
fn truncated_string_outputs() {
    common::load();

    // The `#[lenout]` argument has to bound the buffer the wrapper allocates, or CSPICE would
    // write past its end.
    for length in [2, 5, 8, 40] {
        let mut buffer = vec![0 as c::SpiceChar; length];
        let mut found = 0;
        unsafe { c::bodc2n_c(399, length as c::SpiceInt, buffer.as_mut_ptr(), &mut found) };
        let expected = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        assert_eq!(spice::raw::bodc2n(399, length as i32).0, expected);
    }

    common::unload();
}
