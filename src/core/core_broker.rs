

use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::Context;
use uuid::Uuid;
use crate::prelude::*;
use crate::anyhow::anyhow;

use crate::identifier::connection_identifier_split;

use crate::brokers::BrokerProxy;
use crate::brokers::broker_proxy::{
    BrokerBrokerProxy, 
    ConnectionBrokerProxy, 
    ContainerBrokerProxy, 
    ContainerProcessBrokerProxy, 
    ExecutionContextBrokerProxy,
    ProcessBrokerProxy,
    SystemBrokerProxy, TopicBrokerProxy
};

use crate::identifier::IdentifierStruct;
use crate::object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore};



use crate::utils::{check_corebroker_manifest, get_array};
use crate::utils::manifest_util::id_from_manifest;
use crate::connections::connection_builder::connection_builder;
use super::core_worker::CoreWorker;
use super::subsystem_proxy::SubSystemProxy;
use super::system_store::SystemStorePtr;

#[allow(unused)]
pub struct CoreBroker {
    core: ObjectCore,
    manifest: Value,
    worker: CoreWorker,
    master_system_proxy: Option<SubSystemProxy>,
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
        let uuid = system_store.uuid()?;
        Ok(CoreBroker{
            worker: CoreWorker::new(uuid), 
            core: ObjectCore::create(JuizObjectClass::BrokerProxy("CoreBroker"), "core", "core"),
            manifest: check_corebroker_manifest(manifest)?,
            master_system_proxy: None, 
            subsystem_proxies: Vec::new(),
            system_store
        })
    }


    pub fn worker(&self) -> &CoreWorker {
        &self.worker
    }

    pub fn worker_mut(&mut self) -> &mut CoreWorker {
        &mut self.worker
    }

    pub fn system_store(&self) -> &SystemStorePtr { 
        &self.system_store
    }

    pub fn system_store_mut(&mut self) -> &mut SystemStorePtr { 
        &mut self.system_store
    }



    fn find_subsystem_by_uuid(&self, uuid: Uuid) -> Option<SubSystemProxy> {
        if self.master_system_proxy.is_some() && self.master_system_proxy.as_ref().unwrap().uuid() == &uuid {
            Some(self.master_system_proxy.as_ref().unwrap().clone())
        } else {
            for ssp in self.subsystem_proxies.iter() {
                // 呼び出し元のUUIDがサブシステムと一緒でなければ検査
                if ssp.uuid() == &uuid {
                    return Some(ssp.clone());
                }
            }
            None
        }
    }

    pub fn create_broker_proxy(&self, broker_manifest: Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        self.system_store.create_broker_proxy(self.worker(), &broker_manifest)
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
            "core_store" : self.worker().store().profile_full()?,
        }))?;
        let master_profile = if let Some(system) = self.master_system_proxy.as_ref() { system.profile_full()? } else { serde_json::Value::Null };
        Ok(obj_merge(v, &jvalue!({
            "system_store" : self.system_store.profile_full()?,
            "mastersystem": master_profile,
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
    
    /// サブシステムの追加
    /// 
    /// 
    /// 
    fn system_add_subsystem(&mut self, profile: Value) -> JuizResult<Value> {
        log::trace!("system_add_subsystem({profile}) called");
        let bp = self.system_store.create_broker_proxy(self.worker(), &profile)?;
        let uuid_value = bp.lock().or_else(|_e|{
            Err(anyhow!(JuizError::ObjectLockError { target: "system_store".to_owned() }))
        }).and_then(|b|{ 
            b.system_uuid() 
        })?;
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
        for subsystem_proxy in self.subsystem_proxies.iter() {
            let ss = subsystem_proxy.subsystems()?;
            log::warn!("WARNING: SUBSYSTEM's SUBSYSTEM mining.... But this is useless...");
            log::warn!("value is {ss:}");
        }


        let my_uuid = self.system_store.uuid()?;
        self.worker_mut().store_mut().broker_proxies.register(bp.clone())?;
        let subsystem_proxy = SubSystemProxy::new(uuid, bp.clone())?;
        let ssprofile = juiz_lock(&subsystem_proxy.broker_proxy())?.profile_full().context("subsystem_proxy.broker_proxy().profile_full() in system_add_subsystem")?;
        let _accessed_broker_id = match profile.as_object() {
            Some(obj) => {
                match obj.get("accessed_broker_id") {
                    Some(accessed_broker_id) => {
                        accessed_broker_id.as_str().or_else(||{ Some("") }).unwrap()
                    }
                    None => "",
                }
            }
            None => ""
        };
        log::info!("Subsystem = {}", ssprofile);
        //log::info!("accessed_broker_id = {}", accessed_broker_id);
        let broker_type = ssprofile.as_object().unwrap().get("type_name").unwrap().as_str().unwrap();
        let mut broker_name: Option<String> = None;
        for (_type_name, prof) in self.worker().store().brokers_profile_full()?.as_object().unwrap().iter() {
            let broker_broker_type = prof.as_object().unwrap().get("type_name").unwrap().as_str().unwrap();
            if broker_broker_type == broker_type {
                broker_name = Some(prof.as_object().unwrap().get("name").unwrap().as_str().unwrap().to_owned());
            }
        }
        
        // 相手にmaster側のproxyのbrokerのタイプや名前を教える
        let master_profile = jvalue!({
            "subsystem": {
                "uuid": my_uuid.to_string(),
                "broker_type": broker_type,
                "broker_name": broker_name.unwrap(),
            }
        });
        log::info!("master_profile: {master_profile:}");
        let _ = subsystem_proxy.broker_proxy().lock().or_else(|_e|{
            Err(anyhow!(JuizError::ObjectLockError { target: "system_proxy".to_owned() }))
        }).and_then(|mut bp|{
            bp.system_add_mastersystem(master_profile)
        }).or_else(|e|{
            log::error!("subsystem_proxy.broker_proxy().system_add_mastersystem() failed. Error: {e:?}");
            Err(e)
        })?;
        self.subsystem_proxies.push(subsystem_proxy);
        Ok(profile)
    }
    
    fn system_uuid(&self) -> JuizResult<Value> {
        Ok(jvalue!(self.system_store.uuid()?.to_string()))
    }
    
    /// マスターシステムの追加
    fn system_add_mastersystem(&mut self, profile: Value) -> JuizResult<Value> {
        log::trace!("system_add_mastersystem({profile}) called");
        let bp = match profile.as_object() {
            Some(prof_obj) => {
                match prof_obj.get("subsystem") {
                    Some(subsystem_value) => {
                        let broker_name = obj_get_str(subsystem_value, "broker_name")?;
                        let broker_type = obj_get_str(subsystem_value, "broker_type")?;
                        let id_str = IdentifierStruct::new_broker(broker_type, broker_name);
                        self.system_store.create_broker_proxy(self.worker(), &id_str.to_broker_manifest())
                    },
                    None => Err(anyhow!(JuizError::InvalidIdentifierError { message: "".to_owned() }))
                }
            },
            None => Err(anyhow!(JuizError::ValueIsNotObjectError { value: profile.clone() }))
        }?;
        let uuid_value: Value = match obj_get_obj(&profile, "subsystem")?.get("uuid") {
            Some(v) => Ok(v.clone()),
            None => {
                let my_uuid = self.system_store.uuid()?;
                juiz_lock(&bp)?.system_add_subsystem(jvalue!({
                    "mastersystem": {
                        "uuid": my_uuid.to_string()
                    }
                }))?;
                juiz_lock(&bp)?.system_uuid()
            },
        }?;
        let uuid_str = uuid_value.as_str().unwrap();
        let uuid: Uuid = Uuid::parse_str(uuid_str).unwrap();
        
        self.worker_mut().store_mut().broker_proxies.register(bp.clone())?;

        let subsystem_proxy = SubSystemProxy::new(uuid, bp)?;
        self.master_system_proxy = Some(subsystem_proxy);
        Ok(profile)
    }
}


impl ProcessBrokerProxy for CoreBroker { 
    fn process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            self.worker().store().processes.get(id)?.lock()?.call(args)
        } else {
            self.worker().process_proxy_from_identifier(id)?.lock()?.call(args)
        }
    }

    fn process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        log::trace!("CoreBroker::process_execute({id:}) called");
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            self.worker().store().processes.get(id)?.lock()?.execute()
        } else {
            self.worker().process_proxy_from_identifier(id)?.lock()?.execute()
        }
    }

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        Ok(self.worker().store().processes.get(id)?.lock()?.profile_full()?.into())
    }

    fn process_list(&self, recursive: bool) -> JuizResult<Value> {
        log::trace!("process_list({recursive}) called");
        if !recursive {
            return self.worker().store().processes_id();
        } 

        let mut ids = self.worker().store().processes_id()?;
        match ids.as_array_mut() {
            Some(ids_arr) => {
                for ssp in self.subsystem_proxies.iter() {
                    let plist = juiz_lock(&ssp.broker_proxy())?.process_list(recursive)?;
                    for v in get_array(&plist)?.iter() {
                        let id = v.as_str().unwrap();
                        ids_arr.push(id.into());
                    }
                }
            }
            None => {
                return Err(anyhow!(JuizError::ValueIsNotArrayError { value: jvalue!({})}));
            }
        }
        Ok(ids)
    }

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        let destination_process = self.worker_mut().any_process_proxy_from_identifier(destination_process_id)?;
        self.worker_mut().any_process_proxy_from_identifier(source_process_id)?.lock_mut()?.try_connect_to(destination_process, arg_name, manifest)
    }

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        let source_process = self.worker_mut().any_process_proxy_from_identifier(source_process_id)?;//self.store().processes.get(source_process_id)?;
        self.worker_mut().any_process_proxy_from_identifier(destination_process_id)?.lock_mut()?.notify_connected_from(source_process, arg_name, manifest)
     }
     
    fn process_bind(&mut self, id: &Identifier, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        Ok(self.worker().store().processes.get(id)?.lock_mut()?.bind(arg_name, value)?.into())
    }
    
    fn process_create(&mut self, manifest: &Value) -> JuizResult<Value> {
        self.worker_mut().create_process_ref(manifest.clone())?.lock()?.profile_full()
    }
    
    fn process_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("process_destroy({}) called", identifier);
        match self.worker_mut().destroy_process_ref(identifier)?.lock_mut() {
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
        self.worker().store().containers.get(id)?.clone().lock()?.profile_full()
    }

    fn container_list(&self, recursive: bool) -> JuizResult<Value> {
        //Ok(self.store().containers.list_ids()?.into())
        let mut ids = self.worker().store().containers_id()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            //for (_id, proxy) in self.store().broker_proxies.objects().iter() {
            for ssp in self.subsystem_proxies.iter() {
                let proxy = ssp.broker_proxy();
                let plist = juiz_lock(&proxy)?.container_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(ids)
    }
    
    fn container_create(&mut self, manifest: &Value) -> JuizResult<Value> {
        self.worker_mut().create_container_ref(manifest.clone())?.lock()?.profile_full()
    }
    
    fn container_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("container_destroy({}) called", identifier);
        self.worker_mut().destroy_container_ref(identifier)
    }
}

impl ContainerProcessBrokerProxy for CoreBroker {
    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.worker().store().container_processes.get(id)?.lock().with_context(||format!("locking container_procss(id={id:}) in CoreBroker::container_process_profile_full() function"))?.profile_full()
    }

    fn container_process_list(&self, recursive: bool) -> JuizResult<Value> {
        let mut ids = self.worker().store().container_processes_id()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            for ssp in self.subsystem_proxies.iter() {
                let proxy = ssp.broker_proxy();
            // for (_str, proxy) in self.store().broker_proxies.objects().iter() {
                let plist = juiz_lock(&proxy)?.container_process_list(recursive)?;
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
            self.worker().store().container_processes.get(id)?.lock()?.call(args)
        } else {
            self.worker().process_proxy_from_identifier(id)?.lock()?.call(args)
        }
    }

    fn container_process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            self.worker().store().container_processes.get(id)?.lock().with_context(||format!("locking process(id={id:}) in CoreBroker::execute_process() function"))?.execute()
        } else {
            self.worker().process_proxy_from_identifier(id)?.lock()?.execute()
        }
    }
 
    fn container_process_create(&mut self, container_id: &Identifier, manifest: &Value) -> JuizResult<Value> {
        let container = self.worker_mut().container_from_identifier(container_id)?;
        self.worker_mut().create_container_process_ref(container, manifest.clone())?.lock()?.profile_full()
    }
    
    fn container_process_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("container_process_destroy({}) called", identifier);
        self.worker_mut().destroy_container_process_ref(identifier)
    }
}

impl BrokerBrokerProxy for CoreBroker {
    fn broker_list(&self, recursive: bool) -> JuizResult<Value> {
        let mut ids = self.worker().store().brokers_list_ids()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            // for (_, proxy ) in self.store().broker_proxies.objects().iter() {
            for ssp in self.subsystem_proxies.iter() {
                let proxy = ssp.broker_proxy();
                let plist = juiz_lock(&proxy)?.broker_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(ids)
    }

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.worker().store().broker_profile_full(id)
    }
}


impl TopicBrokerProxy for CoreBroker {
    fn topic_list(&self) -> JuizResult<Value> {
        let mut ids = self.worker().store().topics_list_ids()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if true {
            //for (_, proxy ) in self.store().broker_proxies.objects().iter() {
            for ssp in self.subsystem_proxies.iter() {
                let proxy = ssp.broker_proxy();
        
                let plist = juiz_lock(&proxy)?.topic_list()?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(ids)
    }
    
    fn topic_push(&self, name: &str, capsule: CapsulePtr, pushed_system_uuid: Option<Uuid>) -> JuizResult<()> {
        log::trace!("topic_push(name={name}) called");
        match self.worker().store().topics.get(name) {
            Some(topic) => {
                let r = topic.push(capsule, pushed_system_uuid);
                log::trace!("topic_push(name={name}) exit");
                r
            },
            None => Err(anyhow!(JuizError::ObjectCanNotFoundByIdError { id: name.to_owned() + ":topic" }))
        }
    }
    
    fn topic_request_subscribe(&mut self, name: &str, opt_system_uuid: Option<Uuid>) -> JuizResult<Value> {
        log::trace!("topic_request_subscribe(name={name}) called");
        let mut do_subscribe = false;
        for (topic_name, topic) in self.worker().store().topics.iter() {
            if topic_name.as_str() == name {
                if topic.num_local_subscribers()? > 0 {
                    log::trace!("found subscriber of topic(name={name})");
                    do_subscribe = true;
                }
            }
        }
        let my_uuid = Uuid::parse_str(self.system_uuid()?.as_str().unwrap())?;
        // ここでシステムとマスターシステムに問い合わせて、subscribe要求があれば、自身にTopicを追加して、
        // Systemに対するProxyを新しく生成したTopicに登録してデータがリレーされるようにする
        if let Some(msp) = self.master_system_proxy.clone() {
            // 呼び出し元のUUIDがマスターと一緒でなければマスターを検査
            if opt_system_uuid.is_some() && (msp.uuid() != &opt_system_uuid.unwrap()) {
                let result_value = juiz_lock(&msp.broker_proxy())?.topic_request_subscribe(name, Some(my_uuid))?;
                if obj_get_bool(&result_value, "subscribe")? {
                    // システムが購読を希望していたら、自分のlocalにTopicPtrを作り、それと相手システムを接続する
                    log::trace!("found subscriber of topic(name={name}) in master system");
                    let topic = self.worker_mut().create_topic(name.to_owned())?;
                    topic.register_subscriber_subsystem(msp.clone())?;
                    do_subscribe = true;
                }
            }
        }
        for ssp in self.subsystem_proxies.clone().iter() {
            // 呼び出し元のUUIDがサブシステムと一緒でなければ検査
            if opt_system_uuid.is_some() && (ssp.uuid() != &opt_system_uuid.unwrap()) {
                let result_value = juiz_lock(&ssp.broker_proxy())?.topic_request_subscribe(name, Some(my_uuid))?;
                if obj_get_bool(&result_value, "subscribe")? {
                    log::trace!("found subscriber of topic(name={name}) in subsystem");
                    // システムが購読を希望していたら、自分のlocalにTopicPtrを作り、それと相手システムを接続する
                    let topic = self.worker_mut().create_topic(name.to_owned())?;
                    topic.register_subscriber_subsystem(ssp.clone())?;
                    do_subscribe = true;
                }
            }
        }
        Ok(jvalue!({"subscribe": do_subscribe}))
    }
    

    fn topic_request_publish(&mut self, name: &str, opt_system_uuid: Option<Uuid>) -> JuizResult<Value> {
        log::trace!("topic_request_publish(name={name}, uuid={opt_system_uuid:?}) called");

        let opt_parent_system = if opt_system_uuid.is_some() {
            let uuid = opt_system_uuid.unwrap();
            self.find_subsystem_by_uuid(uuid)
        } else {
            // log::trace!("topic_request_publish was NOT requested by subsystem.");
            None
        };

        //まず、自分がpublisherならばtrueを返す準備。
        let mut do_publish = false;
        for (topic_name, topic) in self.worker().store().topics.iter() {
            if topic_name.as_str() == name {
                if topic.num_local_publishers()? > 0 { // 該当する名前をもつTopicをPublishするものを持っている。
                    do_publish = true;
                    if let Some(parent_system) = opt_parent_system.as_ref() {
                        topic.register_subscriber_subsystem(parent_system.clone())?;
                    }
                }
            }
        }


        // サブシステムについて確認する。サブシステムがpublisherとしてtrueを返してきたら、
        // 自分サイドのtopicを
        let my_uuid = Uuid::parse_str(self.system_uuid()?.as_str().unwrap())?;
        // ここでシステムとマスターシステムに問い合わせて、subscribe要求があれば、自身にTopicを追加して、
        // Systemに対するProxyを新しく生成したTopicに登録してデータがリレーされるようにする
        if let Some(msp) = self.master_system_proxy.clone() {
            // 呼び出し元のUUIDがマスターと一緒でなければマスターを検査
            if opt_system_uuid.is_some() && (msp.uuid() != &opt_system_uuid.unwrap()) {
                let uuid = msp.uuid();
                let result_value = juiz_lock(&msp.broker_proxy())?.topic_request_publish(name, Some(my_uuid))?;
                if obj_get_bool(&result_value, "publish")? {
                    log::trace!("Subsystem({}) publishes topic({})", uuid, name);
                    // システムが出版を宣言していたら、自分のlocalにTopicPtrを作り、それと相手システムを接続する
                    let topic = self.worker_mut().create_topic(name.to_owned())?;
                    if let Some(parent_system) = opt_parent_system.as_ref() {
                        topic.register_subscriber_subsystem(parent_system.clone())?;
                    }
                    do_publish = true;
                }
            } 
        }
        for ssp in self.subsystem_proxies.clone().iter() {
            // 呼び出し元のUUIDがサブシステムと一緒でなければ検査
            if opt_system_uuid.is_some() && (ssp.uuid() != &opt_system_uuid.unwrap()) {
                let result_value = juiz_lock(&ssp.broker_proxy())?.topic_request_publish(name, Some(my_uuid))?;
                if obj_get_bool(&result_value, "publish")? {
                    let uuid = ssp.uuid();
                    log::trace!("Subsystem({}) publishes topic({})", uuid, name);
                    // システムが購読を希望していたら、自分のlocalにTopicPtrを作り、それと相手システムを接続する
                    let topic = self.worker_mut().create_topic(name.to_owned())?;
                    if let Some(parent_system) = opt_parent_system.as_ref() {
                        topic.register_subscriber_subsystem(parent_system.clone())?;
                    }
                    do_publish = true;
                }
            }
        }
        Ok(jvalue!({"publish": do_publish}))
    }

}

impl ExecutionContextBrokerProxy for CoreBroker {
    fn ec_list(&self, recursive: bool) -> JuizResult<Value> {
        //Ok(self.store().ecs.list_ids()?.into())

        let mut ids = self.worker().store().ecs.list_ids()?;
        let ids_arr = ids.as_array_mut().unwrap();
        if recursive {
            //for (_, proxy) in self.store().broker_proxies.objects().iter() {
            for ssp in self.subsystem_proxies.iter() {
                let proxy = ssp.broker_proxy();
        
                let plist = juiz_lock(&proxy)?.ec_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(ids)
    }

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        juiz_lock(&self.worker().store().ecs.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::ec_profile_full() function"))?.profile_full()
    }

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Value> {
        Ok(jvalue!(juiz_lock(&self.worker().store().ecs.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::ec_get_state() function"))?.get_state()?.to_string()).into())
    }

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Value> {
        Ok(juiz_lock(&self.worker().store().ecs.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::ec_get_state() function"))?.start()?.into())
    }

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Value> {
        Ok(juiz_lock(&self.worker().store().ecs.get(id)?).with_context(||format!("locking ec(id={id:}) in CoreBroker::ec_get_state() function"))?.stop()?.into())
    }
    
    fn ec_create(&mut self, manifest: &Value) -> JuizResult<Value> {
        let ec = self.worker_mut().create_ec_ref(manifest.clone())?;
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
            //for (_, proxy ) in self.store().broker_proxies.objects().iter() {
            for ssp in self.subsystem_proxies.iter() {
                let proxy = ssp.broker_proxy();
                
                let plist = juiz_lock(&proxy)?.connection_list(recursive)?;
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
        let result_src_proc = self.worker().store().processes.get(&source_id);
        if result_src_proc.is_ok() {
            for src_con in result_src_proc.unwrap().lock()?.source_connections()?.into_iter() {
                if src_con.identifier().eq(id) {
                    return src_con.profile_full()
                }
            }
        } else {
            println!("Can not found process");
        }
        let result_src_con_proc = self.worker().store().container_processes.get(&source_id);
        if result_src_con_proc.is_ok() {
            //let destination_proc = juiz_lock(&self.store().processes.get(&destination_id)?).with_context(||format!("locking process(id={id:}) in CoreBroker::process_profile_full() function"))?;
            for dst_con in result_src_con_proc.unwrap().lock()?.destination_connections()?.into_iter() {
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
        let source = self.worker_mut().any_process_proxy_from_identifier(&source_id)?;
        let destination = self.worker_mut().any_process_proxy_from_identifier(&destination_id)?;
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
        Ok(self.worker().store().processes.get(id).is_ok())
    }
}


unsafe impl Send for CoreBroker {

}