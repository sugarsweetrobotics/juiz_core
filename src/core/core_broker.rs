

use std::sync::Arc;
use std::sync::Mutex;


use anyhow::Context;

use crate::Container;
use crate::JuizObject;


use crate::brokers::BrokerProxy;
use crate::brokers::broker_proxy::ContainerBrokerProxy;
use crate::brokers::broker_proxy::ContainerProcessBrokerProxy;
use crate::brokers::broker_proxy::ProcessBrokerProxy;
use crate::brokers::broker_proxy::SystemBrokerProxy;

use crate::ecs::execution_context_holder::ExecutionContextHolder;
use crate::object::JuizObjectClass;
use crate::object::JuizObjectCoreHolder;
use crate::object::ObjectCore;
use crate::utils::check_corebroker_manifest;
use crate::utils::juiz_lock;
use crate::utils::manifest_util::type_name;
use crate::value::obj_get_str;
use crate::value::obj_merge;
use crate::{Value, jvalue, Process,Identifier, JuizResult, core::core_store::CoreStore};

use crate::connections::connect;

#[allow(unused)]
pub struct CoreBroker {
    core: ObjectCore,
    manifest: Value,
    core_store: CoreStore,
}

impl CoreBroker {

    pub fn new(manifest: Value) -> JuizResult<CoreBroker> {
        Ok(CoreBroker{
            core: ObjectCore::create(JuizObjectClass::BrokerProxy("CoreBroker"), "core", "core"),
            manifest: check_corebroker_manifest(manifest)?,
            core_store: CoreStore::new()
        })
    }

    pub fn store(&self) -> &CoreStore {
        &self.core_store
    }

    pub fn store_mut(&mut self) -> &mut CoreStore {
        &mut self.core_store
    }

    fn gen_identifier(&self, mut manifest: Value) -> JuizResult<Value> {
        let name = obj_get_str(&manifest, "name")?;
        let type_name = obj_get_str(&manifest, "type_name")?;
        let id = "core://" .to_string()+ name + ":" + type_name;
        manifest.as_object_mut().unwrap().insert("identifier".to_string(), jvalue!(id));
        return Ok(manifest);
    }

    fn gen_name_if_noname(&self, mut manifest: Value) -> JuizResult<Value> {
        if manifest.get("name").is_some() {
            return Ok(manifest);
        }
        let name = type_name(&manifest)?.to_string() + "0";
        manifest.as_object_mut().unwrap().insert("name".to_string(), jvalue!(name));
        return Ok(manifest);
    }

    fn check_has_type_name(&self, manifest: Value) -> JuizResult<Value> {
        let manifest_updated = manifest.clone();
        // let _ = obj_get_str(&manifest,"name")?;
        let _ = obj_get_str(&manifest, "type_name")?;
        return Ok(manifest_updated)
    }

    fn precreate_check<'b>(&'b self, manifest: Value) -> JuizResult<Value> {
        self.gen_identifier(self.gen_name_if_noname(self.check_has_type_name(manifest)?)?)
    }

    //pub fn process_ref(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn Process>>> {
    //    self.store().process(id)
    //}

    pub fn create_process_ref(&mut self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>> {
        log::trace!("CoreBroker::create_process(manifest={}) called", manifest);
        let arc_pf = self.core_store.processes.factory(type_name(&manifest)?)?;
        let p = juiz_lock(arc_pf)?.create_process(self.precreate_check(manifest)?)?;
        self.store_mut().processes.register(p)
    }

    pub fn create_container_ref(&mut self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Container>>> {
        log::trace!("CoreBroker::create_container(manifest={}) called", manifest);
        let arc_pf = self.core_store.containers.factory(type_name(&manifest)?)?;
        let p = juiz_lock(arc_pf)?.create_container(self.precreate_check(manifest)?)?;
        self.store_mut().containers.register(p)
    }

    pub fn create_container_process_ref(&mut self, container: Arc<Mutex<dyn Container>>, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>> {
        log::trace!("CoreBroker::create_container_process(manifest={}) called", manifest);
        let typ_name = type_name(&manifest)?;
        let arc_pf = self.core_store.container_processes.factory(typ_name).with_context(||format!("CoreBroker::create_container_process({})", typ_name))?;
        let p = juiz_lock(arc_pf)?.create_container_process(Arc::clone(&container), self.precreate_check(manifest)?)?;
        self.store_mut().container_processes.register(p)
    }

    pub fn create_ec_ref(&mut self, manifest: Value) -> JuizResult<Arc<Mutex<ExecutionContextHolder>>> {
        log::trace!("CoreBroker::create_ec(manifest={}) called", manifest);
        let arc_pf = self.core_store.ecs.factory(type_name(&manifest)?)?;
        let p = juiz_lock(arc_pf)?.create(self.precreate_check(manifest)?)?;
        self.store_mut().ecs.register(p)
    }

    

}


impl JuizObjectCoreHolder for CoreBroker {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for CoreBroker {

    fn profile_full(&self) -> JuizResult<Value> {
        obj_merge(self.core.profile_full()?, &jvalue!({
            "core_store" : self.core_store.profile_full()?,
        }))
    }
}

impl SystemBrokerProxy for CoreBroker {
    fn system_profile_full(&self) -> JuizResult<Value> {
        log::trace!("CoreBroker::system_profile_full() called");
        self.profile_full()
    }
}


impl ProcessBrokerProxy for CoreBroker { 

    fn process_call(&self, id: &Identifier, args: Value) -> JuizResult<Value> {
        juiz_lock(&self.store().processes.get(id)?)?.call(args)
    }

    fn process_execute(&self, id: &Identifier) -> JuizResult<Value> {
        juiz_lock(&self.store().processes.get(id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::execute_process() function"))?.execute()
    }

    fn process_connect_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        connect(self.store().processes.get(source_process_id)?, self.store().processes.get(target_process_id)?, arg_name, manifest)
    }

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        juiz_lock(&self.store().processes.get(id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::process_profile_full() function"))?.profile_full()
    }
}

impl ContainerBrokerProxy for CoreBroker {
    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        juiz_lock(&self.store().containers.get(id)?).with_context(||format!("locking container(id={id:}) in CoreBroker::container_profile_full() function"))?.profile_full()
    }
}

impl ContainerProcessBrokerProxy for CoreBroker {
    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        juiz_lock(&self.store().container_processes.get(id)?).with_context(||format!("locking container_procss(id={id:}) in CoreBroker::container_process_profile_full() function"))?.profile_full()
    }
}


impl BrokerProxy for CoreBroker {

    fn is_in_charge_for_process(&self, id: &Identifier) -> JuizResult<bool> {
        Ok(self.store().processes.get(id).is_ok())
    }
}


unsafe impl Send for CoreBroker {

}