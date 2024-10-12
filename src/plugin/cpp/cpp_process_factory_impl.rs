

use crate::prelude::*;
use crate::{object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, processes::process_ptr, utils::check_process_factory_manifest, value::obj_get_str};

//pub type CppFunctionType = Symbol<'static, extern "C" fn(*mut CapsuleMap, *mut Capsule) -> i64>;
//pub type PythonFunctionType = dyn Fn(CapsuleMap)->JuizResult<Capsule>;
#[repr(C)]
pub struct CppProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    //plugin: Rc<CppPlugin>,
    entry_point: unsafe fn(*mut CapsuleMap, *mut Capsule) -> i64,
}

impl CppProcessFactoryImpl {

    pub fn new(manifest: &Value, entry_point: unsafe fn(*mut CapsuleMap, *mut Capsule) -> i64) -> JuizResult<Self> {
        let type_name = obj_get_str(manifest, "type_name")?;
        
        Ok(
            CppProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ProcessFactory("ProcessFactoryImpl"),
                    type_name
                ),
                manifest: check_process_factory_manifest(manifest.clone())?, 
                entry_point
            }
        )
    }

    fn apply_default_manifest(&self, manifest: Value) -> Result<Value, JuizError> {
        let mut new_manifest = self.manifest.clone();
        for (k, v) in manifest.as_object().unwrap().iter() {
            new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
        }
        return Ok(new_manifest);
    }
}

impl JuizObjectCoreHolder for CppProcessFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}


impl JuizObject for CppProcessFactoryImpl {
}

impl ProcessFactory for CppProcessFactoryImpl {

    fn create_process(&self, manifest: Value) -> JuizResult<ProcessPtr>{
        log::trace!("CppaProcessFactoryImpl::create_process(manifest={}) called", manifest);
        let entry_point_name = "process_entry_point".to_owned();
        let entry_point = self.entry_point;
        let cppfunc: Box<dyn Fn(CapsuleMap)->JuizResult<Capsule>> = Box::new(move |mut argument: CapsuleMap| -> JuizResult<Capsule> {
            log::trace!("cppfunc (argument={argument:?}) called");
            let mut func_result : Capsule = Capsule::empty();
            unsafe {
                let v = entry_point(&mut argument, &mut func_result);
                if v < 0 {
                    return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError{function_name:entry_point_name.clone(), return_value:v}));
                } 
            }
            return Ok(func_result);
        });

        let proc = ProcessImpl::clousure_new_with_class_name(
            JuizObjectClass::Process("ProcessImpl"), 
            self.apply_default_manifest(manifest.clone())?, 
            cppfunc,
        )?;
        Ok(process_ptr(proc))
    }    
}
