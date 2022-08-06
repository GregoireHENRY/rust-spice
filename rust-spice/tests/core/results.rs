// This is a collection of tests intended to verify the ability to return results when using the "results" feature flag
//
// These tests can be executed with the following:
// cargo test --features results core::results

#[test]
#[serial]
#[cfg(feature = "results")]
fn furnsh_err_empty_string() {
    let result = spice::furnsh("");
    match result {
        Err(e) => assert_eq!(e.kind, spice_results::error::Kind::EMPTYSTRING),
        _ => panic!("Furnishing an empty string should result in an error."),
    }
}

#[test]
#[serial]
#[cfg(feature = "results")]
fn furnsh_err_no_such_file() {
    let result = spice::furnsh("not_a_real_file.pck");
    match result {
        Err(e) => assert_eq!(e.kind, spice_results::error::Kind::NOSUCHFILE),
        _ => panic!("Furnishing an empty string should result in an error."),
    }
}
