#[test]
#[serial]
fn test_load() {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm").unwrap();
    kernel.unload().unwrap();
}
