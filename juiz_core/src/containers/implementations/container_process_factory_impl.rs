use std::sync::Arc;
use juiz_sdk::anyhow::anyhow;
use crate::connections::ConnectionFactoryImpl;
use crate::prelude::*;
use crate::containers::{ContainerImpl, ContainerProcessImpl};
use crate::processes::process_from_clousure_new_with_class_name;

pub type BindedContainerFunctionType = Arc<dyn Fn(ContainerPtr, CapsuleMap)->JuizResult<Capsule>>;
pub struct ContainerProcessFactoryImpl {
    core: ObjectCore,
    manifest: ProcessManifest,
   // function: ContainerFunctionTypePtr<T>,
    binded_function: BindedContainerFunctionType,
}

// pub type ContainerProcessConstructorType<T>=&'static dyn Fn(&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule> ;
impl ContainerProcessFactoryImpl {
    pub fn new_t(manifest: ProcessManifest, function: BindedContainerFunctionType) -> JuizResult<Self> {
        // let type_name = obj_get_str(&manifest, "type_name")?;
        Ok(ContainerProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryImpl"), 
                manifest.type_name.clone()),
               // function: function.clone(),
                manifest,
                binded_function: function
            }
        )
    }

    // pub fn new(manifest: ProcessManifest, function: &'static ContainerFunctionType<T>) -> JuizResult<Self> {
    //     //let type_name = obj_get_str(&manifest, "type_name")?;
    //     let f = Arc::new(|c: &mut ContainerImpl<T>, v| { function(c, v) } );
    //     Ok(Self::new_t(manifest, f)?)
        
    // }

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

impl JuizObjectCoreHolder for ContainerProcessFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ContainerProcessFactoryImpl {}


pub fn bind_container_function<T: 'static >(function: impl Fn(&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule> + 'static) -> BindedContainerFunctionType {
    Arc::new(move |container, args| -> JuizResult<Capsule> {
        match container.lock_mut()?.downcast_mut::<ContainerImpl<T>>() {
            Some(c) =>(function)(c, args),
            None => Err(anyhow!(JuizError::ContainerDowncastingError{identifier: "ContainerPtr".to_owned()}))
        }
    })
}

impl ContainerProcessFactory for ContainerProcessFactoryImpl {
    fn create_container_process(&self, container: ContainerPtr, manifest: ProcessManifest) -> JuizResult<ProcessPtr> {
        log::trace!("ContainerProcessFactoryImpl::create_container_process(container, manifest={:?}) called", manifest);
        
        //let function_clone = self.function.clone();
        // let func = move |args| -> JuizResult<Capsule> {
        //     container.downcast_mut_and_then(|c: &mut ContainerImpl<T> | {
        //         (function_clone)(c, args)
        //     })?
        // };
        let function_clone = self.binded_function.clone();
        let func = move |args| -> JuizResult<Capsule> {
            function_clone(container.clone(), args)
        };
        Ok(ProcessPtr::new(process_from_clousure_new_with_class_name(
            JuizObjectClass::ContainerProcess("ContainerProcessImpl"), 
            self.manifest.build_instance_manifest(manifest)?, 
            func, 
            Box::new(ConnectionFactoryImpl::new()))?))
        // Ok(ProcessPtr::new(
        //     ContainerProcessImpl::new(
        //         //self.apply_default_manifest(manifest)?, 
        //         self.manifest.build_instance_manifest(manifest)?,
        //         container, 
        //         self.function.clone()
        //         //Box::new(|c, v|{ self.function(c, v) }),
        //     )?
        // ))
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