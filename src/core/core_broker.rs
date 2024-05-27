

use std::sync::Arc;
use std::sync::Mutex;


use anyhow::Context;

use crate::containers::container_lock;
use crate::containers::container_proxy::ContainerProxy;
use crate::identifier::connection_identifier_split;

use crate::processes::capsule::CapsuleMap;

use crate::processes::proc_lock;
use crate::processes::proc_lock_mut;
use crate::CapsulePtr;
use crate::ContainerPtr;
use crate::JuizError;
use crate::JuizObject;
use crate::jvalue;

use crate::brokers::BrokerProxy;
use crate::brokers::broker_proxy::BrokerBrokerProxy;
use crate::brokers::broker_proxy::ConnectionBrokerProxy;
use crate::brokers::broker_proxy::ContainerBrokerProxy;
use crate::brokers::broker_proxy::ContainerProcessBrokerProxy;
use crate::brokers::broker_proxy::ExecutionContextBrokerProxy;
use crate::brokers::broker_proxy::ProcessBrokerProxy;
use crate::brokers::broker_proxy::SystemBrokerProxy;

use crate::ecs::execution_context_holder::ExecutionContextHolder;


use crate::identifier::IdentifierStruct;

use crate::identifier::identifier_from_manifest;
use crate::object::JuizObjectClass;
use crate::object::JuizObjectCoreHolder;
use crate::object::ObjectCore;

use crate::processes::process_proxy::ProcessProxy;
use crate::utils::check_corebroker_manifest;
use crate::utils::juiz_lock;
use crate::utils::manifest_util::construct_id;
use crate::utils::manifest_util::id_from_manifest;
use crate::utils::manifest_util::id_from_manifest_and_class_name;
use crate::utils::manifest_util::type_name;
use crate::value::obj_get;
use crate::value::obj_get_str;

use crate::value::obj_merge;
use crate::ProcessPtr;
use crate::{connections::connection_builder::connection_builder, core::core_store::CoreStore, Identifier, JuizResult,  Value};


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

    pub fn create_process_ref(&mut self, manifest: Value) -> JuizResult<ProcessPtr> {
        log::trace!("CoreBroker::create_process(manifest={}) called", manifest);
        let arc_pf = self.core_store.processes.factory(type_name(&manifest)?)?;
        let p = juiz_lock(arc_pf)?.create_process(self.precreate_check(manifest)?)?;
        self.store_mut().processes.register(p)
    }

    pub fn create_container_ref(&mut self, manifest: Value) -> JuizResult<ContainerPtr> {
        log::trace!("CoreBroker::create_container(manifest={}) called", manifest);
        let arc_pf = self.core_store.containers.factory(type_name(&manifest)?)?;
        let p = juiz_lock(arc_pf)?.create_container(self.precreate_check(manifest)?)?;
        self.store_mut().containers.register(p)
    }

    pub fn create_container_process_ref(&mut self, container: ContainerPtr, manifest: Value) -> JuizResult<ProcessPtr> {
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


    pub fn process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        self.store().processes.get(id)
    }

    pub fn process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        self.store().processes.get(&construct_id("Process", type_name, name, "core", "core"))
    }

    pub fn process_from_manifest(&self, manifest: &Value) -> JuizResult<ProcessPtr> {
        self.process_from_id(&id_from_manifest_and_class_name(manifest, "Process")?)
    }

    pub fn container_from_id(&self, id: &Identifier) -> JuizResult<ContainerPtr> {
        self.store().containers.get(id)
    }

    pub fn container_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ContainerPtr> {
        self.store().containers.get(&construct_id("Container", type_name, name, "core", "core"))
    }

    pub fn container_from_manifest(&self, manifest: &Value) -> JuizResult<ContainerPtr> {
        self.container_from_id(&id_from_manifest_and_class_name(manifest, "Container")?)
    }

    pub fn container_process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        self.store().container_processes.get(id)
    }

    pub fn container_process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        self.store().container_processes.get(&construct_id("ContainerProcess", type_name, name, "core", "core"))
    }

    pub fn container_process_from_manifest(&self, manifest: &Value) -> JuizResult<ProcessPtr> {
        self.container_process_from_id(&id_from_manifest_and_class_name(manifest, "ContainerProcess")?)
    }

    pub fn any_process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        self.process_from_id(id).or_else(|_| { self.container_process_from_id(id) })
    }

    pub fn any_process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        self.process_from_typename_and_name(type_name, name).or_else(|_| {self.container_process_from_typename_and_name(type_name, name)})
    }

    pub fn any_process_from_manifest(&self, manifest: &Value) -> JuizResult<ProcessPtr> {
        match id_from_manifest(manifest) {
            Ok(id) => {
                return self.any_process_from_id(&id);
            },
            Err(_) => {
                let type_name = obj_get_str(manifest, "type_name")?;
                let name = obj_get_str(manifest, "name")?;
                self.any_process_from_typename_and_name(type_name, name)
            }
        }
    }

    pub fn broker_proxy_from_manifest(&mut self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        let mut type_name = obj_get_str(manifest, "type_name")?;
        if type_name == "core" {
            type_name = "local";
        }

        let name = match obj_get_str(manifest, "name") {
            Ok(name) => name.to_string(),
            Err(_) => {
                let counter = 0;
                type_name.to_string() + counter.to_string().as_str()
            }
        };
        self.broker_proxy(type_name, name.as_str())
    }

    pub fn broker_proxy(&mut self, broker_type_name: &str, broker_name: &str) ->JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        log::trace!("CoreBroker::broker_proxy({broker_type_name}, {broker_name}) called");
        let mut type_name = broker_type_name;
        if type_name == "core" {
            type_name = "local";
        }

        let identifier = "core://core/BrokerProxy/".to_string() + broker_name + "::" + broker_type_name;
        match self.store().broker_proxies.get(&identifier) {
            Ok(bp) => return Ok(bp),
            Err(_) => {}
        };
        
        let manifest = jvalue!({
            "type_name": type_name,
            "name": broker_name
        });
        let bf = self.store().broker_proxies.factory(type_name)?.clone();
        let bp = juiz_lock(&bf)?.create_broker_proxy(manifest)?;
        self.store_mut().broker_proxies.register(bp.clone())?;
        Ok(bp)
    }

    pub fn container_proxy_from_identifier(&mut self, identifier: &Identifier) -> JuizResult<ContainerPtr> {
        log::info!("CoreBroker::container_proxy_from_identifier({identifier}) called");
        let id_struct = IdentifierStruct::from(identifier.clone());
        if id_struct.broker_name == "core" && id_struct.broker_type_name == "core" {
            return self.container_from_id(identifier)
        }
        let broker_proxy = self.broker_proxy(&id_struct.broker_type_name, &id_struct.broker_name)?;
        Ok(ContainerProxy::new(JuizObjectClass::Container("ContainerProxy"),identifier, broker_proxy)?)
    }

    pub fn process_proxy_from_identifier(&mut self, identifier: &Identifier) -> JuizResult<ProcessPtr> {
        log::info!("CoreBroker::process_proxy_from_identifier({identifier}) called");
        let id_struct = IdentifierStruct::from(identifier.clone());
        if id_struct.broker_name == "core" && id_struct.broker_type_name == "core" {
            return self.process_from_id(identifier)
        }
        let broker_proxy = self.broker_proxy(&id_struct.broker_type_name, &id_struct.broker_name)?;
        Ok(ProcessProxy::new(JuizObjectClass::Process("ProcessProxy"),identifier, broker_proxy)?)
    }

    pub fn process_proxy_from_manifest(&mut self, manifest: &Value) -> JuizResult<ProcessPtr> {
        self.process_proxy_from_identifier(&id_from_manifest_and_class_name(manifest, "Process")?)
    }

    pub fn container_process_proxy_from_identifier(&mut self, identifier: &Identifier) -> JuizResult<ProcessPtr> {
        let id_struct = IdentifierStruct::from(identifier.clone());
        if id_struct.broker_name == "core" && id_struct.broker_type_name == "core" {
            return self.container_process_from_id(identifier)
        }
        let broker_proxy = self.broker_proxy(&id_struct.broker_type_name, &id_struct.broker_name)?;
        Ok(ProcessProxy::new(JuizObjectClass::ContainerProcess("ProcessProxy"), identifier, broker_proxy)?)
    }

    pub fn container_process_proxy_from_manifest(&mut self, manifest: &Value) -> JuizResult<ProcessPtr> {
        self.container_process_proxy_from_identifier(&id_from_manifest_and_class_name(manifest, "ContainerProcess")?)
    }

    pub fn any_process_proxy_from_identifier(&mut self, identifier: &Identifier) -> JuizResult<ProcessPtr> {
        log::trace!("CoreBroker::any_process_proxy_from_identifier({identifier}) called");
        let mut id_struct = IdentifierStruct::from(identifier.clone());
        let p = self.process_proxy_from_identifier(&id_struct.set_class_name("Process").to_identifier());
        if p.is_ok() {
            return p;
        }
        self.container_process_proxy_from_identifier(&id_struct.set_class_name("ContainerProcess").to_identifier())
    }

    pub fn any_process_proxy_from_manifest(&mut self, manifest: &Value) -> JuizResult<ProcessPtr> {
        let identifier = identifier_from_manifest("core", "core", "Process", manifest)?;
        self.any_process_proxy_from_identifier(&identifier)
    }


    pub fn cleanup_ecs(&mut self) -> JuizResult<()> {
        for ec in self.store_mut().ecs.objects() {
            juiz_lock(&ec)?.stop()?;
        }
        self.store_mut().ecs.cleanup_objects()
    }

}


impl JuizObjectCoreHolder for CoreBroker {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for CoreBroker {

    fn profile_full(&self) -> JuizResult<Value> {
        Ok(obj_merge(self.core.profile_full()?, &jvalue!({
            "core_store" : self.core_store.profile_full()?,
        }))?.into())
    }
}

impl SystemBrokerProxy for CoreBroker {
    fn system_profile_full(&self) -> JuizResult<Value> {
        log::trace!("CoreBroker::system_profile_full() called");
        let result = self.profile_full();
        log::trace!("CoreBroker::system_profile_full() exit");
        result
    }
}


impl ProcessBrokerProxy for CoreBroker { 

    fn process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        proc_lock(&self.store().processes.get(id)?)?.call(args)
    }

    fn process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        log::trace!("CoreBroker::process_execute({id:}) called");
        proc_lock(&self.store().processes.get(id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::execute_process() function"))?.execute()
    }


    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        Ok(proc_lock(&self.store().processes.get(id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::process_profile_full() function"))?.profile_full()?.into())
    }

    fn process_list(&self) -> JuizResult<Value> {
        Ok(self.store().processes.list_ids()?.into())
    }

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        let destination_process = self.any_process_proxy_from_identifier(destination_process_id)?;
        proc_lock_mut(&self.any_process_proxy_from_identifier(source_process_id)?)?.try_connect_to(destination_process, arg_name, manifest)
    }

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        let source_process = self.any_process_proxy_from_identifier(source_process_id)?;//self.store().processes.get(source_process_id)?;
        // let destination_process = self.any_process_proxy_from_identifier(destination_process_id)?;
        proc_lock_mut(&self.any_process_proxy_from_identifier(destination_process_id)?)?.notify_connected_from(source_process, arg_name, manifest)
     }
}

impl ContainerBrokerProxy for CoreBroker {
    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        container_lock(&self.store().containers.get(id)?).with_context(||format!("locking container(id={id:}) in CoreBroker::container_profile_full() function"))?.profile_full()
    }

    fn container_list(&self) -> JuizResult<Value> {
        Ok(self.store().containers.list_ids()?.into())
    }
}

impl ContainerProcessBrokerProxy for CoreBroker {
    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        proc_lock(&self.store().container_processes.get(id)?).with_context(||format!("locking container_procss(id={id:}) in CoreBroker::container_process_profile_full() function"))?.profile_full()
    }

    fn container_process_list(&self) -> JuizResult<Value> {
        Ok(self.store().container_processes.list_ids()?.into())
    }

    fn container_process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        log::trace!("CoreBroker::container_process_call(id={id:}, args) called");
        proc_lock(&self.store().container_processes.get(id)?).with_context(||format!("locking container_procss(id={id:}) in CoreBroker::container_process_call() function"))?.call(args)
    }

    fn container_process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        proc_lock(&self.store().container_processes.get(id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::execute_process() function"))?.execute()
    }
}

impl BrokerBrokerProxy for CoreBroker {
    fn broker_list(&self) -> JuizResult<Value> {
        self.store().brokers_list_ids()
    }

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.store().broker_profile_full(id)
    }
}

impl ExecutionContextBrokerProxy for CoreBroker {
    fn ec_list(&self) -> JuizResult<Value> {
        Ok(self.store().ecs.list_ids()?.into())
    }

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        juiz_lock(&self.store().ecs.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::ec_profile_full() function"))?.profile_full()
    }

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Value> {
        Ok(jvalue!(juiz_lock(&self.store().ecs.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::ec_get_state() function"))?.get_state()?.to_string()).into())
    }

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Value> {
        Ok(juiz_lock(&self.store().ecs.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::ec_get_state() function"))?.start()?.into())
    }

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Value> {
        Ok(juiz_lock(&self.store().ecs.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::ec_get_state() function"))?.stop()?.into())
    }

}

impl ConnectionBrokerProxy for CoreBroker {

    fn connection_list(&self) -> JuizResult<Value> {
        let cons = connection_builder::list_connection_profiles(self)?;
        Ok(jvalue!(cons.iter().map(|con_prof| { obj_get(con_prof, "identifier").unwrap() }).collect::<Vec<&Value>>()).into())
    }

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        //juiz_lock(&self.store().connections.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::connection_profile_full() function"))?.profile_full()
        let (source_id, _destination_id, _arg_name) = connection_identifier_split(id.clone())?;
        println!("source_id: {:}", source_id);
        let result_src_proc = self.store().processes.get(&source_id);
        if result_src_proc.is_ok() {
            for src_con in proc_lock(&(result_src_proc.unwrap()))?.source_connections()?.into_iter() {
                if src_con.identifier().eq(id) {
                    return src_con.profile_full()
                }
            }
        } else {
            println!("Can not found process");
        }
        let result_src_con_proc = self.store().container_processes.get(&source_id);
        if result_src_con_proc.is_ok() {
            //let destination_proc = juiz_lock(&self.store().processes.get(&destination_id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::process_profile_full() function"))?;
            for dst_con in proc_lock(&(result_src_con_proc.unwrap()))?.destination_connections()?.into_iter() {
                println!("con: {:}", dst_con.identifier());
                if dst_con.identifier().eq(id) {
                    return dst_con.profile_full()
                }
            }
        } else {
            println!("Can not found container process");

        }
        Err(anyhow::Error::from(JuizError::ConnectionCanNotBeFoundError{identifier: id.clone()}))
    }

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Value> {
        log::trace!("CoreBroker::connection_create({manifest}) called");
        let (source_id, destination_id) = check_connection_source_destination(&manifest)?;
        let source = self.any_process_proxy_from_identifier(&source_id)?;
        let destination = self.any_process_proxy_from_identifier(&destination_id)?;
        let arg_name = obj_get_str(&manifest, "arg_name")?;
        Ok(connection_builder::connect(source, destination, &arg_name.to_string(), manifest)?.into())
    }
}

fn check_if_both_side_is_on_same_host(source_id: Identifier, destination_id: Identifier) -> JuizResult<(Identifier, Identifier)> {
    log::trace!("check_if_both_side_is_on_same_host({source_id}, {destination_id}) called");
    let mut source_id_struct = IdentifierStruct::from(source_id);
    let mut destination_id_struct = IdentifierStruct::from(destination_id);
    if (source_id_struct.broker_name == destination_id_struct.broker_name) &&
        (source_id_struct.broker_type_name == destination_id_struct.broker_type_name) {
        source_id_struct.broker_name = "core".to_owned();
        source_id_struct.broker_type_name = "core".to_owned();
        destination_id_struct.broker_name = "core".to_owned();
        destination_id_struct.broker_type_name = "core".to_owned();
    }
    Ok((source_id_struct.to_identifier(), destination_id_struct.to_identifier()))
}

fn check_connection_source_destination(manifest: &Value) -> JuizResult<(Identifier, Identifier)> {
    let source = obj_get(manifest, "source")?;
    let destination = obj_get(manifest, "destination")?;

    let source_id_result = obj_get_str(source, "identifier");
    let destination_id_result = obj_get_str(destination, "identifier");
    
    // まずIDが両方ともあったら、brokerが同じものを指していたらcore/coreに直して接続する
    if source_id_result.is_ok() && destination_id_result.is_ok() {
        return check_if_both_side_is_on_same_host(source_id_result.unwrap().to_owned(), destination_id_result.unwrap().to_owned());
    }

    // IDがない場合はProcessかContainerProcessかが曖昧だが一旦Processで
    return Ok((id_from_manifest(source)?, id_from_manifest(destination)?))
}

impl BrokerProxy for CoreBroker {

    fn is_in_charge_for_process(&self, id: &Identifier) -> JuizResult<bool> {
        Ok(self.store().processes.get(id).is_ok())
    }
}


unsafe impl Send for CoreBroker {

}