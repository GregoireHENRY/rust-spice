//! DAF, DAS and DLA: the array files the kernels are built on.

use crate::common;

#[test]
#[serial]
fn daf_summaries() {
    common::load();

    // The test SPK is a DAF holding three segments.
    let handle = spice::dafopr(&common::kernel("test.bsp"));
    common::assert_ok("dafopr");
    assert!(handle != 0);

    let (nd, ni, ifname, fward, bward, free) = spice::dafrfr(handle, spice::MAX_LEN_OUT);
    common::assert_ok("dafrfr");
    assert_eq!(
        (nd, ni),
        (2, 6),
        "an SPK has two doubles and six integers per summary"
    );
    assert_eq!(ifname, "rust-spice test SPK");
    assert!(fward > 0 && bward > 0 && free > 0);

    // Walk the segments forward, reading each summary and name.
    spice::dafbfs(handle);
    let mut segments = Vec::new();
    while spice::daffna() {
        let summary = spice::dafgs();
        let (dc, ic) = spice::dafus(&summary, nd as usize, ni as usize);
        segments.push((
            spice::dafgn(spice::MAX_LEN_OUT as i32),
            dc[0],
            dc[1],
            ic[0],
            ic[1],
        ));
    }
    common::assert_ok("walking the segments");

    assert_eq!(segments.len(), 3);
    let names = segments.iter().map(|s| s.0.as_str()).collect::<Vec<_>>();
    assert!(names.contains(&"TEST ORBIT"));
    assert!(names.contains(&"TEST SATELLITE"));
    // Body and centre of the first segment, and its coverage.
    let orbit = segments.iter().find(|s| s.0 == "TEST ORBIT").unwrap();
    assert_eq!((orbit.3, orbit.4), (common::TARGET, common::CENTER));
    assert_relative_eq!(orbit.1, common::FIRST, epsilon = 1e-6);
    assert_relative_eq!(orbit.2, common::LAST, epsilon = 1e-6);

    // Searching backward finds the same segments.
    spice::dafbbs(handle);
    let mut backward = 0;
    while spice::daffpa() {
        backward += 1;
    }
    assert_eq!(backward, 3);

    spice::dafcls(handle);
    common::assert_ok("dafcls");
    common::unload();
}

#[test]
#[serial]
fn daf_summary_packing() {
    common::reset();

    // Packing and unpacking a summary round trips.
    let dc = [1.5, -2.5];
    let ic = [1, 2, 3, 4, 5, 6];
    let summary = spice::dafps(&dc, &ic);
    common::assert_ok("dafps");

    let (back_dc, back_ic) = spice::dafus(&summary, dc.len(), ic.len());
    assert_eq!(back_dc, dc.to_vec());
    assert_eq!(back_ic, ic.to_vec());
}

#[test]
#[serial]
fn daf_raw_reads() {
    common::load();

    let handle = spice::dafopr(&common::kernel("test.bsp"));
    spice::dafbfs(handle);
    assert!(spice::daffna());

    // The first summary record holds the same words however it is read.
    let (record, found) = spice::dafgsr(handle, 2, 1, 8);
    common::assert_ok("dafgsr");
    assert!(found);
    assert_eq!(record.len(), 8);

    // And the summary sizes agree with the file record.
    let (nd, ni) = spice::dafhsf(handle);
    assert_eq!((nd, ni), (2, 6));

    // `dafgh` reports the handle of the file the search is in.
    assert_eq!(spice::dafgh(), handle);

    spice::dafcls(handle);
    common::unload();
}

#[test]
#[serial]
fn daf_comments() {
    common::load();

    let path = common::kernel("commented.bsp");
    let _ = std::fs::remove_file(&path);

    // Write an SPK, then add comments to it.
    let handle = spice::spkopn(&path, "commented", 0);
    let epochs = (0..8).map(|i| i as f64).collect::<Vec<f64>>();
    let states = vec![[0.0; 6]; epochs.len()];
    spice::spkw09(
        handle,
        common::SPACECRAFT,
        common::TARGET,
        "J2000",
        epochs[0],
        epochs[7],
        "SEG",
        3,
        8,
        &states,
        &epochs,
    );
    spice::spkcls(handle);
    common::assert_ok("writing the SPK");

    let handle = spice::dafopw(&path);
    spice::dafac(handle, &["first line", "second line"]);
    common::assert_ok("dafac");
    spice::dafcls(handle);

    // Read them back.
    let handle = spice::dafopr(&path);
    let comments = spice::dafec(handle);
    common::assert_ok("dafec");
    assert_eq!(
        comments,
        vec!["first line".to_string(), "second line".to_string()]
    );

    // And delete them.
    spice::dafcls(handle);
    let handle = spice::dafopw(&path);
    spice::dafdc(handle);
    spice::dafcls(handle);
    let handle = spice::dafopr(&path);
    assert!(spice::dafec(handle).is_empty());
    spice::dafcls(handle);
    common::assert_ok("dafdc");

    common::unload();
    let _ = std::fs::remove_file(&path);
}

#[test]
#[serial]
fn das_file_record_and_data() {
    common::load();

    // The test DSK is a DAS.
    let handle = spice::dasopr(&common::kernel("test.bds"));
    common::assert_ok("dasopr");

    let (idword, ifname, nresvr, nresvc, ncomr, ncomc) =
        spice::dasrfr(handle, spice::MAX_LEN_OUT, spice::MAX_LEN_OUT);
    common::assert_ok("dasrfr");
    assert!(idword.starts_with("DAS/DSK"), "got {idword:?}");
    assert_eq!(ifname, "rust-spice test DSK");
    assert_eq!((nresvr, nresvc), (0, 0));
    let _ = (ncomr, ncomc);

    // The file name can be recovered from the handle.
    assert_eq!(
        spice::dashfn(handle, spice::MAX_LEN_OUT as i32),
        common::kernel("test.bds")
    );

    // The last addresses in use, and reading the first few words back.
    let (lastc, lastd, lasti) = spice::daslla(handle);
    common::assert_ok("daslla");
    assert!(lastd > 0 && lasti > 0);
    let _ = lastc;

    let doubles = spice::dasrdd(handle, 1, 4);
    assert_eq!(doubles.len(), 4);
    let integers = spice::dasrdi(handle, 1, 4);
    assert_eq!(integers.len(), 4);
    common::assert_ok("dasrdd and dasrdi");

    spice::dascls(handle);
    common::unload();
}

#[test]
#[serial]
fn das_writing() {
    common::load();

    let path = common::kernel("written.das");
    let _ = std::fs::remove_file(&path);

    // Create a DAS, append data of each kind, and add a comment.
    let handle = spice::dasonw(&path, "TEST", "rust-spice test DAS", 0);
    common::assert_ok("dasonw");
    spice::dasadd(handle, &[1.5, 2.5, 3.5]);
    spice::dasadi(handle, &[10, 20, 30]);
    spice::dasadc(handle, 0, 4, &["alpha", "betas"]);
    spice::dasac(handle, &["a comment"]);
    common::assert_ok("appending to the DAS");

    // Update one of the words of each kind before closing. The characters are addressed one at a
    // time, so the second five-character line runs from six to ten.
    spice::dasudd(handle, 2, 2, &[9.5]);
    spice::dasudi(handle, 3, 3, &[99]);
    spice::dasudc(handle, 6, 10, 0, 4, &["gamma"]);
    common::assert_ok("updating the DAS");
    spice::daswbr(handle);
    spice::dasllc(handle);
    common::assert_ok("closing the DAS");

    // Read it all back.
    let handle = spice::dasopr(&path);
    assert_eq!(spice::dasrdd(handle, 1, 3), vec![1.5, 9.5, 3.5]);
    assert_eq!(spice::dasrdi(handle, 1, 3), vec![10, 20, 99]);
    assert_eq!(spice::dasrdc(handle, 1, 10, 0, 4), vec!["alpha", "gamma"]);
    assert_eq!(spice::dasec(handle), vec!["a comment".to_string()]);
    common::assert_ok("reading the DAS back");
    spice::dascls(handle);

    common::unload();
    let _ = std::fs::remove_file(&path);
}

#[test]
#[serial]
fn dla_segment_walk() {
    common::load();

    let handle = spice::dasopr(&common::kernel("test.bds"));
    let (first, found) = spice::dlabfs(handle);
    assert!(found);

    // One segment, so there is nothing before or after it.
    assert!(!spice::dlafns(handle, first).1);
    assert!(!spice::dlafps(handle, first).1);
    common::assert_ok("dlafps");

    spice::dascls(handle);
    common::unload();
}

#[test]
#[serial]
fn ck_and_pck_handles() {
    common::load();

    // A CK can be loaded and unloaded through the pointing subsystem.
    let handle = spice::cklpf(&common::kernel("test.bc"));
    common::assert_ok("cklpf");
    assert!(handle != 0);

    // The pointing is reachable while it is loaded.
    let (matrix, reference, found) = spice::ckfrot(common::INSTRUMENT, common::EPOCH);
    common::assert_ok("ckfrot");
    assert!(found);
    assert_eq!(reference, 1, "the test CK is relative to J2000");
    crate::assert_matrix_eq(&matrix, &spice::ident(), 1e-15);

    // And the state form agrees with it.
    let (xform, reference, found) = spice::ckfxfm(common::INSTRUMENT, common::EPOCH);
    common::assert_ok("ckfxfm");
    assert!(found);
    assert_eq!(reference, 1);
    crate::assert_matrix_eq(&spice::xf2rav(xform).0, &matrix, 1e-15);

    // The clock and spacecraft a CK object belongs to.
    assert_eq!(
        spice::ckmeta(common::INSTRUMENT, "SCLK"),
        common::SPACECRAFT
    );
    assert_eq!(spice::ckmeta(common::INSTRUMENT, "SPK"), common::SPACECRAFT);
    common::assert_ok("ckmeta");

    spice::ckupf(handle);

    // The binary PCK loads and unloads the same way.
    let handle = spice::pcklof(&common::kernel("test.bpc"));
    common::assert_ok("pcklof");
    assert!(handle != 0);
    spice::pckuof(handle);
    common::assert_ok("pckuof");

    common::unload();
}
