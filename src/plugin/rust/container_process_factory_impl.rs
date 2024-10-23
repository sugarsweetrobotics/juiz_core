use std::sync::Arc;

use crate::prelude::*;
use crate::containers::{ContainerFunctionType, ContainerFunctionTypePtr, ContainerImpl, ContainerProcessImpl};
use crate::{object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, value::obj_get_str};
pub struct ContainerProcessFactoryImpl<T> where T: 'static {
    core: ObjectCore,
    manifest: ProcessManifest,
    function: ContainerFunctionTypePtr<T>,
}

// pub type ContainerProcessConstructorType<T>=&'static dyn Fn(&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule> ;
impl<T: 'static> ContainerProcessFactoryImpl<T> {
    pub fn new_t(manifest: ProcessManifest, function: ContainerFunctionTypePtr<T>) -> JuizResult<Self> {
        // let type_name = obj_get_str(&manifest, "type_name")?;
        Ok(ContainerProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryImpl"), 
                manifest.type_name.clone()),
                function,
                manifest,
            }
        )
    }

    pub fn new(manifest: ProcessManifest, function: &'static ContainerFunctionType<T>) -> JuizResult<Self> {
        //let type_name = obj_get_str(&manifest, "type_name")?;
        let f = Arc::new(|c: &mut ContainerImpl<T>, v| { function(c, v) } );
        Ok(Self::new_t(manifest, f)?)
        
    }

    // fn apply_default_manifest(&self, manifest: Value) -> JuizResult<Value> {
    //     let mut new_manifest = self.manifest.clone();
    //     for (k, v) in manifest.as_object().unwrap().iter() {
    //         new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
    //     }
    //     return Ok(new_manifest);
    // }
}

// pub fn create_container_process_factory<T: 'static>(
//         manifest: crate::Value, 
//         //function: ContainerFunctionType<T>
//         f: &'static impl Fn(&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule>    
//     ) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> 
// //where F: 
// {
//     log::trace!("create_container_process_factory({}) called", manifest);
//     //et function = f.clone();
//     let ff = Arc::new(|c: &mut ContainerImpl<T>, v| { f(c, v) } );
//     ContainerProcessFactoryImpl::<T>::create(manifest, ff)
// }

impl<T: 'static> JuizObjectCoreHolder for ContainerProcessFactoryImpl<T> {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl<T: 'static> JuizObject for ContainerProcessFactoryImpl<T> {}

impl<T: 'static> ContainerProcessFactory for ContainerProcessFactoryImpl<T> {
    fn create_container_process(&self, container: ContainerPtr, manifest: ProcessManifest) -> JuizResult<ProcessPtr> {
        log::trace!("ContainerProcessFactoryImpl::create_container_process(container, manifest={:?}) called", manifest);
        Ok(ProcessPtr::new(
            ContainerProcessImpl::new(
                //self.apply_default_manifest(manifest)?, 
                self.manifest.build_instance_manifest(manifest)?,
                container, 
                self.function.clone()
                //Box::new(|c, v|{ self.function(c, v) }),
            )?
        ))
    }
    
    fn destroy_container_process(&mut self, proc: ProcessPtr) -> JuizResult<Value> {
        log::trace!("ContainerProcessFactoryImpl({})::destroy_container_process() called", self.type_name());
        proc.downcast_mut_and_then(|p: &mut ContainerProcessImpl| { 
            let prof = p.profile_full()?;
            p.container.take();
            log::trace!("ContainerFactoryImpl({})::destroy_container_process() exit", self.type_name());
            Ok(prof)
        })?
    }
}