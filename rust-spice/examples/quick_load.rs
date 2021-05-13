use spice;

fn main() -> Result<(), spice::SystemError> {
    let mut system = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
    system.unload()?;
    Ok(())
}
