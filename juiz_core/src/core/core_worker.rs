
use std::sync::{Arc, Mutex};

use juiz_base::{identifier::identifier_from_manifest, utils::manifest_util::{construct_id, id_from_manifest, id_from_manifest_and_class_name, type_name}};
use uuid::Uuid;

use crate::{connections::connection_builder::connection_builder, containers::{ContainerProcessImpl, ContainerProxy}, ecs::{execution_context_function::ExecutionContextFunction, execution_context_proxy::ExecutionContextProxy}, object::JuizObjectClass, prelude::*, topics::TopicPtr};

use super::core_store::CoreStore;
use crate::anyhow::anyhow;



pub struct CoreWorker {
    store: CoreStore,
    system_uuid: Uuid,
}



impl CoreWorker {
    pub fn new(uuid: Uuid) -> Self {
        CoreWorker{store: CoreStore::new(), system_uuid: uuid}
    }

    pub fn store(&self) -> &CoreStore {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut CoreStore {
        &mut self.store
    }

    pub fn process_from_identifier(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        let s = IdentifierStruct::try_from(id.clone())?;
        if s.broker_type_name == "core" {
            return Ok(self.store().processes.get(id)?.clone());
        }
        self.process_proxy_from_identifier(id)
    }

    pub fn process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        Ok(self.store().processes.get(&construct_id("Process", type_name, name, "core", "core"))?.clone())
    }

    pub fn process_proxy_from_identifier(&self, identifier: &Identifier) -> JuizResult<ProcessPtr> {
        log::info!("CoreBroker::process_proxy_from_identifier({identifier}) called");
        let id_struct = IdentifierStruct::try_from(identifier.clone())?;
        if id_struct.broker_name == "core" && id_struct.broker_type_name == "core" {
            return self.process_from_identifier(identifier)
        }
        let broker_proxy = self.broker_proxy(&id_struct.broker_type_name, &id_struct.broker_name, false)?;
        Ok(ProcessProxy::new(JuizObjectClass::Process("ProcessProxy"),identifier, broker_proxy)?)
    }

    pub fn process_proxy_from_manifest(&mut self, manifest: &Value) -> JuizResult<ProcessPtr> {
        self.process_proxy_from_identifier(&id_from_manifest_and_class_name(manifest, "Process")?)
    }



    pub fn broker_proxy(&self, broker_type_name: &str, broker_name: &str, create_when_not_found: bool) ->JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        log::trace!("CoreBroker::broker_proxy({broker_type_name}, {broker_name}) called");
        let mut type_name = broker_type_name;
        if type_name == "core" { type_name = "local"; }

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
        let bp = juiz_lock(&bf)?.create_broker_proxy(self, manifest).or_else(|e| {
            log::error!("creating BrokerProxy(type_name={type_name}) failed. Error ({e})");
            Err(e)
        })?;
        self.store().broker_proxies.register(bp.clone())?;
        Ok(bp)
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


    pub fn create_process_ref(&mut self, manifest: ProcessManifest) -> JuizResult<ProcessPtr> {
        log::trace!("CoreBroker::create_process_ref(manifest={:?}) called", manifest);
        let arc_pf = self.store().processes.factory(manifest.type_name.as_str())?;
        let p = arc_pf.lock()?.create_process(manifest)?;
        let id = p.identifier().clone();
        Ok(self.store_mut().processes.register(&id, p)?.clone())
    }

    pub fn destroy_process_ref(&mut self, identifier: &Identifier) -> JuizResult<ProcessPtr> {
        log::trace!("CoreBroker::destroy_process(identifier={}) called", identifier);
        self.store_mut().processes.deregister_by_id(identifier)
    }



    pub fn create_container_ref(&mut self, manifest: ContainerManifest) -> JuizResult<ContainerPtr> {
        log::trace!("CoreBroker::create_container(manifest={:?}) called", manifest);
        let arc_pf = self.store().containers.factory(manifest.type_name.as_str())?.clone();
        let p = arc_pf.lock()?.create_container(self, manifest)?;
        let id = p.identifier().clone();
        Ok(self.store_mut().containers.register(&id, p)?.clone())
    }

    pub fn destroy_container_ref(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("CoreBroker::destroy_container_ref(identifier={}) called", identifier);
        let cont = self.store().containers.get(identifier)?.clone();
        let ids = cont.lock_mut()?.processes().iter().map(|cp|{
            cp.identifier().clone()
        }).collect::<Vec<Identifier>>();
        for pid in ids.iter() {
            self.destroy_container_process_ref(pid)?;
            //container_lock_mut(&mut cont.clone())?.purge_process(pid)?;
        }
        self.store_mut().containers.deregister_by_id(identifier)?;
        let f = self.store().containers.factory(cont.type_name().as_str())?;
        log::trace!("container_destroy({}) exit", identifier);
        f.lock_mut()?.destroy_container(cont.clone())
    }

    pub fn create_container_process_ref(&mut self, container: ContainerPtr, manifest: ProcessManifest) -> JuizResult<ProcessPtr> {
        log::trace!("CoreBroker::create_container_process_ref(manifest={:?}) called", manifest);
        //let typ_name = type_name(&manifest)?;
        let arc_pf = self.store().container_processes.factory(manifest.type_name.as_str())?;
        let p = arc_pf.lock()?.create_container_process(container.clone(), manifest)?;
        container.lock_mut()?.register_process(p.clone())?;
        let id = p.identifier().clone();
        Ok(self.store_mut().container_processes.register(&id, p)?.clone())
    }

    pub fn destroy_container_process_ref(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("CoreBroker::destroy_container_process_ref(identifier={}) called", identifier);
        let process = self.store_mut().container_processes.deregister_by_id(identifier)?;
        let c = process.downcast_and_then(|cp: &ContainerProcessImpl| {
            self.store().containers.get(cp.identifier())
        })??;
        //let c = self.store().containers.get(con_id)?;
        c.lock_mut()?.purge_process(identifier)?;
        process.lock_mut()?.purge()?;
        let f = self.store().container_processes.factory(process.type_name())?;
        let v = f.lock_mut()?.destroy_container_process(process);
        log::trace!("destroy_container_process_ref({}) exit", identifier);
        v
    }


    pub fn container_from_identifier(&mut self, id: &Identifier) -> JuizResult<ContainerPtr> {
        let s = IdentifierStruct::try_from(id.clone())?;
        if s.broker_type_name == "core" {
            return Ok(self.store().containers.get(id)?.clone())
        }
        self.container_proxy_from_identifier(id)
       
    }

    pub fn container_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ContainerPtr> {
        Ok(self.store().containers.get(&construct_id("Container", type_name, name, "core", "core"))?.clone())
    }

    pub fn container_from_manifest(&mut self, manifest: &Value) -> JuizResult<ContainerPtr> {
        self.container_from_identifier(&id_from_manifest_and_class_name(manifest, "Container")?)
    }

    pub fn container_process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        Ok(self.store().container_processes.get(id)?.clone())
    }

    pub fn container_process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        Ok(self.store().container_processes.get(&construct_id("ContainerProcess", type_name, name, "core", "core"))?.clone())
    }

    pub fn container_proxy_from_identifier(&mut self, identifier: &Identifier) -> JuizResult<ContainerPtr> {
        log::info!("CoreBroker::container_proxy_from_identifier({identifier}) called");
        let id_struct = IdentifierStruct::try_from(identifier.clone())?;
        if id_struct.broker_name == "core" && id_struct.broker_type_name == "core" {
            return self.container_from_identifier(identifier)
        }
        let broker_proxy = self.broker_proxy(&id_struct.broker_type_name, &id_struct.broker_name, false)?;
        Ok(ContainerPtr::new(ContainerProxy::new(JuizObjectClass::Container("ContainerProxy"),identifier, broker_proxy)?))
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

    pub fn any_process_from_identifier(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        self.process_from_identifier(id).or_else(|_| { self.container_process_from_id(id) })
    }

    pub fn any_process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        self.process_from_typename_and_name(type_name, name).or_else(|_| {self.container_process_from_typename_and_name(type_name, name)})
    }

    pub fn any_process_from_manifest(&self, manifest: &Value) -> JuizResult<ProcessPtr> {
        match id_from_manifest(manifest) {
            Ok(id) => {
                return self.any_process_from_identifier(&id);
            },
            Err(_) => {
                let type_name = obj_get_str(manifest, "type_name")?;
                let name = obj_get_str(manifest, "name")?;
                self.any_process_from_typename_and_name(type_name, name)
            }
        }
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

    

    pub fn create_ec_ref(&mut self, manifest: Value) -> JuizResult<Arc<Mutex<dyn ExecutionContextFunction>>> {
        log::trace!("CoreBroker::create_ec_ref(manifest={}) called", manifest);
        let arc_pf = self.store().ecs.factory(type_name(&manifest)?).or_else(|e| {
            log::error!("create_ec_ref({manifest:}) failed. Searching factory failed. Error({e:})");
            Err(e)
        })?;
        let p = juiz_lock(arc_pf)?.create(precreate_check(manifest.clone())?).or_else(|e| {
            log::error!("create_ec_ref({:}) failed. Error({e})", manifest.clone());
            Err(e)
        })?;

        self.store_mut().ecs.register(p)
    }

    pub fn ec_from_id(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn ExecutionContextFunction>>> {
        self.store().ecs.get(id)
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
    
    pub fn cleanup_ecs(&mut self) -> JuizResult<()> {
        for ec in self.store_mut().ecs.objects().values() {
            juiz_lock(&ec)?.stop()?;
        }
        self.store_mut().ecs.cleanup_objects()
    }


    pub fn create_topic(&mut self, topic_name_str: String) -> JuizResult<TopicPtr> {
        return Ok(if self.store().topics.contains_key(&topic_name_str) {
            // Topicがすでにある時
            self.store().topics.get(&topic_name_str).unwrap().clone()
        } else {
            // Topicがない時
            self.do_create_topic(topic_name_str)?
        })
    }

    fn do_create_topic(&mut self, topic_name: String) -> JuizResult<TopicPtr> {
        log::error!("do_create_topic({topic_name}) called");
        let uuid = self.system_uuid.clone();
        self.store_mut().topics.insert(topic_name.clone(), TopicPtr::new(topic_name.as_str(), uuid));
        Ok(self.store().topics.get(&topic_name).unwrap().clone())
    }


    pub fn process_publish_topic(&mut self, process: ProcessPtr, topic_info: TopicManifest) -> JuizResult<()> {
        log::error!("process_publish_topic({topic_info:?}) called");
        let topic_name = topic_info.name.as_str();
        let topic = self.create_topic(topic_name.to_owned())?;
        self.connect_to_topic(process, topic)?;
        Ok(())
    }

    pub fn process_subscribe_topic(&mut self, process: ProcessPtr, arg_name: &String, topic_info: TopicManifest) -> JuizResult<()> {
        log::error!("process_subscribe_topic({arg_name}, {topic_info:?}) called");
        let topic_name = topic_info.name.as_str();
        let topic = self.create_topic(topic_name.to_owned())?;
        //let p = self.process_from_id(&id)?.clone();
        self.connect_from_topic(process, arg_name, topic)?;
        Ok(())
    }

    fn connect_to_topic(&mut self, process: ProcessPtr, topic: TopicPtr) -> JuizResult<()> {
        log::error!("connect_to_topic");
        let topic_publish_connection_manifest = jvalue!({
            "type": "push",
        });
        let _connection_profile = connection_builder::connect(process, topic.process_ptr(), &"input".to_owned(), topic_publish_connection_manifest)?;
        Ok(())
    }

    fn connect_from_topic(&mut self, process: ProcessPtr, arg_name: &String, topic: TopicPtr) -> JuizResult<()> {
        log::error!("connect_from_topic");
        let topic_subscribe_connection_manifest = jvalue!({
            "type": "push",
        });
        let _connection_profile = connection_builder::connect(topic.process_ptr(), process, arg_name, topic_subscribe_connection_manifest)?;
        Ok(())
    }
}

fn precreate_check(manifest: Value) -> JuizResult<Value> {
    log::trace!("precreate_check(manifest={manifest:}) called");
    gen_identifier(gen_name_if_noname(check_has_type_name(manifest)?)?).or_else(|e| {
        log::trace!("precreate_check() failed. Error({e})");
        Err(e)
    })
}

fn gen_name_if_noname(mut manifest: Value) -> JuizResult<Value> {
    if manifest.get("name").is_some() {
        return Ok(manifest);
    }
    let name = type_name(&manifest)?.to_string() + "0";
    manifest.as_object_mut().unwrap().insert("name".to_string(), jvalue!(name));
    return Ok(manifest);
}

fn gen_identifier(mut manifest: Value) -> JuizResult<Value> {
    let name = obj_get_str(&manifest, "name")?;
    let type_name = obj_get_str(&manifest, "type_name")?;
    let id = "core://" .to_string()+ name + ":" + type_name;
    manifest.as_object_mut().unwrap().insert("identifier".to_string(), jvalue!(id));
    return Ok(manifest);
}

fn check_has_type_name(manifest: Value) -> JuizResult<Value> {
    let manifest_updated = manifest.clone();
    // let _ = obj_get_str(&manifest,"name")?;
    let _ = obj_get_str(&manifest, "type_name")?;
    return Ok(manifest_updated)
}