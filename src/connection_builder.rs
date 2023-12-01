

pub mod connection_builder {
    use std::sync::{Arc, Mutex};
    use crate::{process::Process, error::JuizError, Value};
    
    pub fn connect(source: Arc<Mutex<dyn Process>>, destination: Arc<Mutex<dyn Process>>, arg_name: &String, manifest: Value) -> Result<Value, JuizError> {
        let source_connect_result_manifest = match source.try_lock() {
            Err(_e) => return Err(JuizError::ConnectionBuilderCanNotBorrowSourceProcessError{}),
            Ok(mut proc_s) => {
                proc_s.connection_to(Arc::clone(&destination), arg_name, manifest) 
            }
        }?;
        return match destination.try_lock() {
            Err(_e) => return Err(JuizError::ConnectionBuilderCanNotBorrowDestinationProcessError{}),
            Ok(mut proc_d) => {
                proc_d.connected_from(source, arg_name, source_connect_result_manifest)
            }
        };
    }
}