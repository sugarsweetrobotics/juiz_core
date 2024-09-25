

use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::Context;
use uuid::Uuid;
use crate::ecs::execution_context_proxy::ExecutionContextProxy;
use crate::prelude::*;
use crate::anyhow::anyhow;

use crate::containers::container_process_impl::container_proc_lock_mut;
use crate::containers::container_lock;
use crate::containers::container_lock_mut;
use crate::containers::container_process_impl::container_proc_lock;

use crate::containers::container_proxy::ContainerProxy;
use crate::identifier::connection_identifier_split;

use crate::processes::proc_lock;
use crate::processes::proc_lock_mut;

use crate::brokers::BrokerProxy;
use crate::brokers::broker_proxy::{
    BrokerBrokerProxy, 
    ConnectionBrokerProxy, 
    ContainerBrokerProxy, 
    ContainerProcessBrokerProxy, 
    ExecutionContextBrokerProxy,
    ProcessBrokerProxy,
    SystemBrokerProxy
};

use crate::ecs::execution_context_function::ExecutionContextFunction;

use crate::identifier::IdentifierStruct;

use crate::identifier::identifier_from_manifest;
use crate::object::JuizObjectClass;
use crate::object::JuizObjectCoreHolder;
use crate::object::ObjectCore;

use crate::processes::process_proxy::ProcessProxy;
use crate::utils::{check_corebroker_manifest, get_array};
use crate::utils::juiz_lock;
use crate::utils::manifest_util::construct_id;
use crate::utils::manifest_util::id_from_manifest;
use crate::utils::manifest_util::id_from_manifest_and_class_name;
use crate::utils::manifest_util::type_name;
use crate::value::obj_get;
use crate::value::obj_get_str;

use crate::value::obj_merge;
use crate::{connections::connection_builder::connection_builder, core::core_store::CoreStore};
use super::subsystem_proxy::SubSystemProxy;
use super::system::SystemStorePtr;

#[allow(unused)]
pub struct CoreBroker {
    core: ObjectCore,
    manifest: Value,
    core_store: CoreStore,
    subsystem_proxies: Vec<SubSystemProxy>,
    system_store: SystemStorePtr,
}

#[derive(Clone)]
pub struct CoreBrokerPtr {
    ptr: Arc<RwLock<CoreBroker>>
}

unsafe impl Send for CoreBrokerPtr {}

impl CoreBrokerPtr {
    
    pub fn new(core_broker: CoreBroker) -> Self {
        Self{ptr: Arc::new(RwLock::new(core_broker))}
    }
    pub fn lock(&self) -> JuizResult<RwLockReadGuard<CoreBroker>> {
        self.ptr.read().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"CoreBrokerPtr".to_owned()})) })
    }

    pub fn lock_mut(&self) -> JuizResult<RwLockWriteGuard<CoreBroker>> {
        self.ptr.write().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"CoreBrokerPtr".to_owned()})) })
    }
}

impl CoreBroker {

    pub fn new(manifest: Value, system_store: SystemStorePtr) -> JuizResult<CoreBroker> {
        Ok(CoreBroker{
            core: ObjectCore::create(JuizObjectClass::BrokerProxy("CoreBroker"), "core", "core"),
            manifest: check_corebroker_manifest(manifest)?,
            core_store: CoreStore::new(),
            subsystem_proxies: Vec::new(),
            system_store
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
        log::trace!("precreate_check(manifest={manifest:}) called");
        self.gen_identifier(self.gen_name_if_noname(self.check_has_type_name(manifest)?)?).or_else(|e| {
            log::trace!("precreate_check() failed. Error({e})");
            Err(e)
        })
    }

    pub fn create_process_ref(&mut self, manifest: Value) -> JuizResult<ProcessPtr> {
        log::trace!("CoreBroker::create_process_ref(manifest={}) called", manifest);
        let arc_pf = self.core_store.processes.factory(type_name(&manifest)?)?;
        let p = juiz_lock(arc_pf)?.create_process(self.precreate_check(manifest)?)?;
        self.store_mut().processes.register(p)
    }

    pub fn destroy_process_ref(&mut self, identifier: &Identifier) -> JuizResult<ProcessPtr> {
        log::trace!("CoreBroker::destroy_process(identifier={}) called", identifier);
        self.store_mut().processes.deregister_by_id(identifier)
    }

    pub fn create_container_ref(&mut self, manifest: Value) -> JuizResult<ContainerPtr> {
        log::trace!("CoreBroker::create_container(manifest={}) called", manifest);
        let arc_pf = self.core_store.containers.factory(type_name(&manifest)?)?;
        let p = juiz_lock(arc_pf)?.create_container(self.precreate_check(manifest)?)?;
        self.store_mut().containers.register(p)
    }

    pub fn destroy_container_ref(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("CoreBroker::destroy_container_ref(identifier={}) called", identifier);
        let cont = self.store().containers.get(identifier)?;
        let tn = container_lock(&cont)?.type_name().to_owned();
        let ids = container_lock_mut(&mut cont.clone())?.processes().iter().map(|cp|{
            proc_lock(cp).unwrap().identifier().clone()
        }).collect::<Vec<Identifier>>();
        for pid in ids.iter() {
            self.container_process_destroy(pid)?;
            //container_lock_mut(&mut cont.clone())?.purge_process(pid)?;
        }
        self.store_mut().containers.deregister_by_id(identifier)?;
        let f = self.store().containers.factory(tn.as_str())?;
        log::trace!("container_destroy({}) exit", identifier);
        juiz_lock(f)?.destroy_container(cont)
    }

    pub fn create_container_process_ref(&mut self, container: ContainerPtr, manifest: Value) -> JuizResult<ProcessPtr> {
        log::trace!("CoreBroker::create_container_process_ref(manifest={}) called", manifest);
        let typ_name = type_name(&manifest)?;
        let arc_pf = self.core_store.container_processes.factory(typ_name).with_context(||format!("CoreBroker::create_container_process({})", typ_name))?;
        let p = juiz_lock(arc_pf)?.create_container_process(Arc::clone(&container), self.precreate_check(manifest)?)?;
        container_lock_mut(&container)?.register_process(p.clone())?;
        Ok(self.store_mut().container_processes.register(p)?)
    }

    pub fn destroy_container_process_ref(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("CoreBroker::destroy_container_process_ref(identifier={}) called", identifier);
        let process = self.store_mut().container_processes.deregister_by_id(identifier)?;
        let tn = container_proc_lock(&process)?.type_name().to_owned();
        let con_id  = container_lock(container_proc_lock(&process)?.container.as_ref().unwrap())?.identifier().clone();
        let c = self.store().containers.get(&con_id)?;
        container_lock_mut(&c)?.purge_process(identifier)?;
        container_proc_lock_mut(&process)?.purge()?;
        let f = self.store().container_processes.factory(tn.as_str())?;
        let v = juiz_lock(f)?.destroy_container_process(process);
        log::trace!("destroy_container_process_ref({}) exit", identifier);
        v
    }

    pub fn create_ec_ref(&mut self, manifest: Value) -> JuizResult<Arc<Mutex<dyn ExecutionContextFunction>>> {
        log::trace!("CoreBroker::create_ec_ref(manifest={}) called", manifest);
        let arc_pf = self.core_store.ecs.factory(type_name(&manifest)?).or_else(|e| {
            log::error!("create_ec_ref({manifest:}) failed. Searching factory failed. Error({e:})");
            Err(e)
        })?;
        let p = juiz_lock(arc_pf)?.create(self.precreate_check(manifest.clone())?).or_else(|e| {
            log::error!("create_ec_ref({:}) failed. Error({e})", manifest.clone());
            Err(e)
        })?;

        self.store_mut().ecs.register(p)
    }

    pub fn ec_from_id(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn ExecutionContextFunction>>> {
        self.store().ecs.get(id)
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
        Ok(self.store().container_processes.get(id)?)
    }

    pub fn container_process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        Ok(self.store().container_processes.get(&construct_id("ContainerProcess", type_name, name, "core", "core"))?)
    }

    pub fn container_processes_by_container(&self, _container: ContainerPtr) -> JuizResult<Vec<ProcessPtr>> {
        for _p in self.store().container_processes.objects().into_iter() {
            //let c = (p as Arc<RwLock<ContainerProcessImpl>>).container;
        }
        todo!();
        //self.container_process_from_id(&id_from_manifest_and_class_name(manifest, "ContainerProcess")?)
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

    pub fn broker_proxy_from_manifest(&mut self, manifest: &Value, create_when_not_found: bool) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
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
        self.broker_proxy(type_name, name.as_str(), create_when_not_found)
    }

    pub fn broker_proxy(&self, broker_type_name: &str, broker_name: &str, create_when_not_found: bool) ->JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
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
        
        log::warn!("broker_proxy({broker_type_name}, {broker_name}) can not find broker_proxy. creating....");
        if !create_when_not_found {
            return Err(anyhow!(JuizError::ObjectCanNotFoundByIdError { id: format!("{broker_type_name}://{broker_name}") }));
        }
        let manifest = jvalue!({
            "type_name": type_name,
            "name": broker_name
        });
        let bf = self.store().broker_proxies.factory(type_name).or_else(|e| {
            log::error!("creating BrokerProxyFactory(type_name={type_name}) failed. Error ({e})");
            Err(e)
        })?;
        let bp = juiz_lock(&bf)?.create_broker_proxy(manifest).or_else(|e| {
            log::error!("creating BrokerProxy(type_name={type_name}) failed. Error ({e})");
            Err(e)
        })?;
        self.store().broker_proxies.register(bp.clone())?;
        Ok(bp)
    }

    pub fn container_proxy_from_identifier(&mut self, identifier: &Identifier) -> JuizResult<ContainerPtr> {
        log::info!("CoreBroker::container_proxy_from_identifier({identifier}) called");
        let id_struct = IdentifierStruct::try_from(identifier.clone())?;
        if id_struct.broker_name == "core" && id_struct.broker_type_name == "core" {
            return self.container_from_id(identifier)
        }
        let broker_proxy = self.broker_proxy(&id_struct.broker_type_name, &id_struct.broker_name, false)?;
        Ok(ContainerProxy::new(JuizObjectClass::Container("ContainerProxy"),identifier, broker_proxy)?)
    }

    pub fn process_proxy_from_identifier(&self, identifier: &Identifier) -> JuizResult<ProcessPtr> {
        log::info!("CoreBroker::process_proxy_from_identifier({identifier}) called");
        let id_struct = IdentifierStruct::try_from(identifier.clone())?;
        if id_struct.broker_name == "core" && id_struct.broker_type_name == "core" {
            return self.process_from_id(identifier)
        }
        let broker_proxy = self.broker_proxy(&id_struct.broker_type_name, &id_struct.broker_name, false)?;
        Ok(ProcessProxy::new(JuizObjectClass::Process("ProcessProxy"),identifier, broker_proxy)?)
    }

    pub fn ec_proxy_from_identifier(&mut self, identifier: &Identifier) -> JuizResult<Arc<Mutex<dyn ExecutionContextFunction>>> {
        log::info!("CoreBroker::ec_proxy_from_identifier({identifier}) called");
        let id_struct = IdentifierStruct::try_from(identifier.clone())?;
        if id_struct.broker_name == "core" && id_struct.broker_type_name == "core" {
            return self.ec_from_id(identifier)
        }
        let broker_proxy = self.broker_proxy(&id_struct.broker_type_name, &id_struct.broker_name, false)?;
        Ok(ExecutionContextProxy::new(JuizObjectClass::ExecutionContext("ExecutionContextProxy"),identifier, broker_proxy)?)
    }

    pub fn process_proxy_from_manifest(&mut self, manifest: &Value) -> JuizResult<ProcessPtr> {
        self.process_proxy_from_identifier(&id_from_manifest_and_class_name(manifest, "Process")?)
    }

    pub fn container_process_proxy_from_identifier(&mut self, identifier: &Identifier) -> JuizResult<ProcessPtr> {
        let id_struct = IdentifierStruct::try_from(identifier.clone())?;
        if id_struct.broker_name == "core" && id_struct.broker_type_name == "core" {
            return self.container_process_from_id(identifier)
        }
        let broker_proxy = self.broker_proxy(&id_struct.broker_type_name, &id_struct.broker_name, false)?;
        Ok(ProcessProxy::new(JuizObjectClass::ContainerProcess("ProcessProxy"), identifier, broker_proxy)?)
    }

    pub fn container_process_proxy_from_manifest(&mut self, manifest: &Value) -> JuizResult<ProcessPtr> {
        self.container_process_proxy_from_identifier(&id_from_manifest_and_class_name(manifest, "ContainerProcess")?)
    }

    pub fn any_process_proxy_from_identifier(&mut self, identifier: &Identifier) -> JuizResult<ProcessPtr> {
        log::trace!("CoreBroker::any_process_proxy_from_identifier({identifier}) called");
        let mut id_struct = IdentifierStruct::try_from(identifier.clone())?;
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
        for ec in self.store_mut().ecs.objects().values() {
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
        let v = obj_merge(self.core.profile_full()?, &jvalue!({
            "core_store" : self.core_store.profile_full()?,
        }))?;
        Ok(obj_merge(v, &jvalue!({
            "system_store" : self.system_store.profile_full()?,
            "subsystems": self.subsystem_proxies.iter().map(|p|{p.profile_full().unwrap()}).collect::<Vec<Value>>()
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
    
    fn system_filesystem_list(&self, path_buf: PathBuf) -> JuizResult<Value> {
        let entries = std::fs::read_dir(path_buf)?
            .map(|res| res.map(|e| {
                jvalue!({
                    "path": e.path().to_str().unwrap(),
                    "is_dir": e.path().is_dir()
                })
            }).or::<JuizError>(Ok(jvalue!("Error"))).unwrap())
            .collect::<Vec<Value>>();
        Ok(jvalue!(entries))
    }
    
    fn system_add_subsystem(&mut self, profile: Value) -> JuizResult<Value> {
        log::trace!("system_add_subsystem({profile}) called");
        let bp = self.system_store.create_broker_proxy(&profile)?;
        let uuid_value = bp.lock().or_else(|_e|{Err(anyhow!(JuizError::ObjectLockError { target: "system_store".to_owned() }))})
            .and_then(|b|{ b.system_uuid() })?;
        log::trace!("uuid_value: {uuid_value:?}");
        // 相手のuuid
        let uuid_str = uuid_value.as_str().unwrap();
        let uuid: Uuid = Uuid::parse_str(uuid_str).unwrap();
        // ここですでにuuidが登録されているかを確認する。
        for subsystem_proxy in self.subsystem_proxies.iter() {
            if &uuid == subsystem_proxy.uuid() {
                log::error!("system_add_subsystem failed. Subsystem(uuid={uuid}) has already added.");
                return Err(anyhow!(JuizError::ObjectAlreadyRegisteredError{message: format!("system_add_subsystem failed. Subsystem(uuid={uuid}) has already added.")}));
            }
        }
        let my_uuid = self.system_store.uuid()?;
        for subsystem_proxy in self.subsystem_proxies.iter() {
            let ss = subsystem_proxy.subsystems()?;
            log::warn!("WARNING: SUBSYSTEM's SUBSYSTEM mining.... But this is useless...");
            log::warn!("value is {ss:}");
        }
        self.store_mut().broker_proxies.register(bp.clone())?;
        self.subsystem_proxies.push(SubSystemProxy::new(uuid, bp)?);
        // self.system_store.lock_mut()?
        Ok(profile)
    }
    
    fn system_uuid(&self) -> JuizResult<Value> {
        Ok(jvalue!(self.system_store.uuid()?.to_string()))
    }
}


impl ProcessBrokerProxy for CoreBroker { 

    fn process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            proc_lock(&self.store().processes.get(id)?)?.call(args)
        } else {
            proc_lock(&self.process_proxy_from_identifier(id)?)?.call(args)
        }
    }

    fn process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        log::trace!("CoreBroker::process_execute({id:}) called");
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            proc_lock(&self.store().processes.get(id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::execute_process() function"))?.execute()
        } else {
            proc_lock(&self.process_proxy_from_identifier(id)?)?.execute()
        }
    }

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        Ok(proc_lock(&self.store().processes.get(id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::process_profile_full() function"))?.profile_full()?.into())
    }

    fn process_list(&self, recursive: bool) -> JuizResult<Value> {
        log::trace!("process_list({recursive}) called");
        let mut ids = self.store().processes.list_ids()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            for (_str, proxy) in self.store().broker_proxies.objects().iter() {
                let plist = juiz_lock(proxy)?.process_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(ids)
    }

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        let destination_process = self.any_process_proxy_from_identifier(destination_process_id)?;
        proc_lock_mut(&self.any_process_proxy_from_identifier(source_process_id)?)?.try_connect_to(destination_process, arg_name, manifest)
    }

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        let source_process = self.any_process_proxy_from_identifier(source_process_id)?;//self.store().processes.get(source_process_id)?;
        // let destination_process = self.any_process_proxy_from_identifier(destination_process_id)?;
        proc_lock_mut(&self.any_process_proxy_from_identifier(destination_process_id)?)?.notify_connected_from(source_process, arg_name, manifest)
     }
     
    fn process_bind(&mut self, id: &Identifier, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        Ok(proc_lock_mut(&self.store().processes.get(id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::bind() function"))?.bind(arg_name, value)?.into())
    }
    
    fn process_create(&mut self, manifest: &Value) -> JuizResult<Value> {
        let proc = self.create_process_ref(manifest.clone())?;
        proc_lock(&proc.clone())?.profile_full()
    }
    
    fn process_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("process_destroy({}) called", identifier);
        let proc = self.destroy_process_ref(identifier)?;
        match proc_lock_mut(&proc.clone()) {
            Ok(mut p) => {
                let prof = p.profile_full()?;
                p.purge()?;
                log::trace!("process_destroy({}) exit", identifier);
                Ok(prof)
            },
            Err(_) => todo!(),
        }
    }
   
}

impl ContainerBrokerProxy for CoreBroker {
    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        container_lock(&self.store().containers.get(id)?).with_context(||format!("locking container(id={id:}) in CoreBroker::container_profile_full() function"))?.profile_full()
    }

    fn container_list(&self, recursive: bool) -> JuizResult<Value> {
        //Ok(self.store().containers.list_ids()?.into())
        let mut ids = self.store().containers.list_ids()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            for (_id, proxy) in self.store().broker_proxies.objects().iter() {
                let plist = juiz_lock(proxy)?.container_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(ids)
    }
    
    fn container_create(&mut self, manifest: &Value) -> JuizResult<Value> {
        let cont = self.create_container_ref(manifest.clone())?;
        container_lock(&cont.clone())?.profile_full()
    }
    
    fn container_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("container_destroy({}) called", identifier);
        self.destroy_container_ref(identifier)
    }
}

impl ContainerProcessBrokerProxy for CoreBroker {
    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        proc_lock(&self.store().container_processes.get(id)?).with_context(||format!("locking container_procss(id={id:}) in CoreBroker::container_process_profile_full() function"))?.profile_full()
    }

    fn container_process_list(&self, recursive: bool) -> JuizResult<Value> {
        let mut ids = self.store().container_processes.list_ids()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            for (_str, proxy) in self.store().broker_proxies.objects().iter() {
                let plist = juiz_lock(proxy)?.container_process_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(ids)
    }

    fn container_process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        log::trace!("CoreBroker::container_process_call(id={id:}, args) called");
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            proc_lock(&self.store().container_processes.get(id)?)?.call(args)
        } else {
            proc_lock(&self.process_proxy_from_identifier(id)?)?.call(args)
        }
    }

    fn container_process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            proc_lock(&self.store().container_processes.get(id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::execute_process() function"))?.execute()
        } else {
            proc_lock(&self.process_proxy_from_identifier(id)?)?.execute()
        }
    }
 
    fn container_process_create(&mut self, container_id: &Identifier, manifest: &Value) -> JuizResult<Value> {
        let container = self.container_from_id(container_id)?;
        let cp = self.create_container_process_ref(container, manifest.clone())?;
        proc_lock(&cp.clone())?.profile_full()
    }
    
    fn container_process_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("container_process_destroy({}) called", identifier);
        self.destroy_container_process_ref(identifier)
    }
}

impl BrokerBrokerProxy for CoreBroker {
    fn broker_list(&self, recursive: bool) -> JuizResult<Value> {
        // self.store().brokers_list_ids()

        let mut ids = self.store().brokers_list_ids()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            for (_, proxy ) in self.store().broker_proxies.objects().iter() {
                let plist = juiz_lock(proxy)?.broker_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(ids)
    }

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.store().broker_profile_full(id)
    }
}

impl ExecutionContextBrokerProxy for CoreBroker {
    fn ec_list(&self, recursive: bool) -> JuizResult<Value> {
        //Ok(self.store().ecs.list_ids()?.into())

        let mut ids = self.store().ecs.list_ids()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            for (_, proxy) in self.store().broker_proxies.objects().iter() {
                let plist = juiz_lock(proxy)?.ec_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(ids)
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
    
    fn ec_create(&mut self, manifest: &Value) -> JuizResult<Value> {
        let ec = self.create_ec_ref(manifest.clone())?;
        juiz_lock(&ec.clone())?.profile_full()
    }
    
    fn ec_destroy(&mut self, _identifier: &Identifier) -> JuizResult<Value> {
        todo!()
    }

}

impl ConnectionBrokerProxy for CoreBroker {

    fn connection_list(&self, recursive: bool) -> JuizResult<Value> {
        let cons = connection_builder::list_connection_profiles(self)?;
        let mut ids_arr = cons.iter().map(|con_prof| { obj_get(con_prof, "identifier").unwrap().clone() }).collect::<Vec<Value>>();
    
        // let mut ids = self.store().containers.list_ids()?;
        // let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            for (_, proxy ) in self.store().broker_proxies.objects().iter() {
                let plist = juiz_lock(proxy)?.connection_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(jvalue!(ids_arr))
    }

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        //juiz_lock(&self.store().connections.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::connection_profile_full() function"))?.profile_full()
        let (source_id, _destination_id, _arg_name) = connection_identifier_split(id.clone())?;
        // println!("source_id: {:}", source_id);
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
            for dst_con in container_proc_lock(&(result_src_con_proc.unwrap()))?.destination_connections()?.into_iter() {
                // println!("con: {:}", dst_con.identifier());
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
    
    fn connection_destroy(&mut self, _id: &Identifier) -> JuizResult<Value> {
        todo!()
    }
}

fn check_if_both_side_is_on_same_host(source_id: Identifier, destination_id: Identifier) -> JuizResult<(Identifier, Identifier)> {
    log::trace!("check_if_both_side_is_on_same_host({source_id}, {destination_id}) called");
    let mut source_id_struct = IdentifierStruct::try_from(source_id)?;
    let mut destination_id_struct = IdentifierStruct::try_from(destination_id)?;
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