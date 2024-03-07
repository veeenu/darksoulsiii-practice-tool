use anyhow::Result;

mod aob_scans;
mod params;

pub(crate) fn codegen() -> Result<()> {
    aob_scans::get_base_addresses();
    params::codegen()?;

    Ok(())
}
