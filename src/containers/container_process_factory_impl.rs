use std::sync::{Arc, Mutex, RwLock};
use super::{container_impl::ContainerImpl, container_process_impl::{ContainerFunctionType, ContainerProcessImpl}};
use crate::{object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, value::obj_get_str, Capsule, CapsuleMap, ContainerProcessFactory, ContainerPtr, JuizObject, JuizResult, ProcessPtr, Value};

struct ContainerProcessFactoryImpl<T> {
    core: ObjectCore,
    manifest: Value,
    function: ContainerFunctionType<T>,
}

impl<T: 'static> ContainerProcessFactoryImpl<T> {
    pub fn new(manifest: crate::Value, function: ContainerFunctionType<T>) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        Ok(Arc::new(Mutex::new(
            ContainerProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryImpl"), 
                type_name),
                function,
                manifest,
            }
        )))
    }

    fn apply_default_manifest(&self, manifest: Value) -> JuizResult<Value> {
        let mut new_manifest = self.manifest.clone();
        for (k, v) in manifest.as_object().unwrap().iter() {
            new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
        }
        return Ok(new_manifest);
    }
}

pub fn create_container_process_factory<T: 'static>(
        manifest: crate::Value, 
        //function: ContainerFunctionType<T>
        f: &'static impl Fn(&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule>    
    ) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> 
//where F: 
{
    log::trace!("create_container_process_factory({}) called", manifest);
    //et function = f.clone();
    let ff = Arc::new(|c: &mut ContainerImpl<T>, v| { f(c, v) } );
    ContainerProcessFactoryImpl::<T>::new(manifest, ff)
}

impl<T: 'static> JuizObjectCoreHolder for ContainerProcessFactoryImpl<T> {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl<T: 'static> JuizObject for ContainerProcessFactoryImpl<T> {}

impl<T: 'static> ContainerProcessFactory for ContainerProcessFactoryImpl<T> {
    fn create_container_process(&self, container: ContainerPtr, manifest: crate::Value) -> JuizResult<ProcessPtr> {
        log::trace!("ContainerProcessFactoryImpl::create_container_process(container, manifest={}) called", manifest);
        Ok(Arc::new(RwLock::new(
            ContainerProcessImpl::new(
                self.apply_default_manifest(manifest)?, 
                Arc::clone(&container), 
                self.function.clone()
                //Box::new(|c, v|{ self.function(c, v) }),
            )?
        )))
    }
}