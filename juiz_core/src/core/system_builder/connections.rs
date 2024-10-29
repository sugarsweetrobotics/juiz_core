
use juiz_sdk::anyhow::{self, anyhow, Context};
use crate::{connections::connection_builder::connection_builder, prelude::*};

pub(super) fn setup_connections(system: &System, manifest: &Value) -> JuizResult<()> {
    log::trace!("system_builder::setup_connections() called");
    for c in get_array(manifest)?.iter() {
        let srcv = obj_get_obj(c, "source")?;
        let dstv = obj_get_obj(c, "destination")?;
        //let p_type_name = obj_get_str(c, "type_name")?;
        log::debug!("Connection ({:?}->{:?}) Creating...", srcv, dstv);
        connection_builder::create_connection(system, &c).context("connection_builder::create_connections faled in system_builder::setup_connections()")?;
        log::info!("Connection ({:?}->{:?}) Created", srcv, dstv);
    } 
    Ok(())
}
