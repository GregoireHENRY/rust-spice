use spice;

fn main() -> Result<(), spice::SystemError> {
    let mut system = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
    system.unload()?;
    Ok(())
}
