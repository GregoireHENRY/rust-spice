#[test]
#[serial]
fn test_load() {
    spice::load("rsc/data/hera_PO_EMA_2024.tm");
    spice::unload("rsc/data/hera_PO_EMA_2024.tm");
}
