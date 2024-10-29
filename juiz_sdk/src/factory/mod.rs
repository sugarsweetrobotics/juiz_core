use std::sync::Arc;
use anyhow::anyhow;
use crate::{containers::ContainerImpl, prelude::*};

pub struct ProcessFactoryStruct(pub ProcessManifest, pub fn(CapsuleMap)->JuizResult<Capsule>);

pub fn process_factory(manifest: ProcessManifest, func: fn(CapsuleMap)->JuizResult<Capsule>) -> ProcessFactoryStruct {
    ProcessFactoryStruct(manifest, func)
}

pub struct ContainerFactoryStruct(pub ContainerManifest, pub Arc<dyn Fn(ContainerManifest)->JuizResult<ContainerPtr>+'static>);


pub fn container_factory<T: 'static>(manifest: ContainerManifest, function: impl Fn(ContainerManifest)->JuizResult<Box<T>> + 'static)-> ContainerFactoryStruct {
    ContainerFactoryStruct(manifest, Arc::new(bind_container_constructor(function)))
}

pub fn bind_container_constructor<T: 'static>(function: impl Fn(ContainerManifest)->JuizResult<Box<T>>) -> impl Fn(ContainerManifest)->JuizResult<ContainerPtr> {
    move |cn: ContainerManifest| -> JuizResult<ContainerPtr> {
        Ok(ContainerPtr::new(ContainerImpl::new(cn.clone(), function(cn)?)?))
    }
}


pub struct ContainerProcessFactoryStruct(pub ProcessManifest, pub Arc<dyn Fn(ContainerPtr, CapsuleMap)->JuizResult<Capsule>+'static>);

pub fn container_process_factory<T: 'static>(manifest: ProcessManifest, function: impl Fn(&mut ContainerImpl<T>, CapsuleMap)->JuizResult<Capsule> + 'static)-> ContainerProcessFactoryStruct {
    ContainerProcessFactoryStruct(manifest, Arc::new(bind_container_process_function(function)))
}

fn bind_container_process_function<T: 'static>(function: impl Fn(&mut ContainerImpl<T>, CapsuleMap)->JuizResult<Capsule>) -> impl Fn(ContainerPtr, CapsuleMap)->JuizResult<Capsule> {
    move |container_ptr: ContainerPtr, capmap: CapsuleMap| -> JuizResult<Capsule> {
        match container_ptr.lock_mut()?.downcast_mut::<ContainerImpl<T>>() {
            Some(cn) => (function)(cn, capmap),
            None => Err(anyhow!(JuizError::ContainerDowncastingError { identifier: "ContainerPTr".to_owned() }))
        }
    }
}
