#[test]
#[serial]
fn test_load_no_adviced() {
    // Do not call spice::load() directly, use the type Kernel.
    spice::load("rsc/data/hera_PO_EMA_2024.tm").unwrap();
    spice::unload("rsc/data/hera_PO_EMA_2024.tm").unwrap();
}

#[test]
#[serial]
fn test_load() {
    // This is how a kernel should be loaded.
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm").unwrap();
    kernel.unload().unwrap();
}
