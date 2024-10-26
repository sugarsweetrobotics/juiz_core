use std::sync::Arc;

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


