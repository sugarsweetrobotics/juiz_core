

use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use juiz_sdk::anyhow::{anyhow, Context};
use juiz_sdk::connections::ConnectionManifest;
use juiz_sdk::utils::check_corebroker_manifest;
use uuid::Uuid;
use crate::prelude::*;


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


use crate::core::CoreWorker;
use crate::core::SubSystemProxy;
use crate::core::SystemStorePtr;

#[allow(unused)]
// #[derive(Debug)]
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
            worker: CoreWorker::new(uuid, manifest.clone()), 
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

    pub fn reserve_master_broker(&mut self, master_info: Value) -> JuizResult<()> {
        log::trace!("reserve_master_broker({master_info:}) called");
        // let broker_type = obj_get_str(&master_info, "broker_type");
        self.worker_mut().reserve_master_broker(master_info)
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
        let master_profile = if let Some(system) = self.master_system_proxy.as_ref() { system.profile_full()? } else {  Value::Null };
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
        if path_buf.is_relative() {
            let cwd = std::env::current_dir()?;
            let mut entries = std::fs::read_dir(path_buf.clone())?
                .map(|res| res.map(|e| {
                    jvalue!({
                        "path": cwd.join(e.path().to_str().unwrap()),
                        "is_dir": e.path().is_dir()
                    })
                }).or::<JuizError>(Ok(jvalue!("Error"))).unwrap())
                .collect::<Vec<Value>>();
            entries.push(jvalue!({
                //"path": ".",
                "is_dir": true,
                "path": std::env::current_dir()?.join(path_buf)
            }));  
            Ok(entries.into())
        } else {
            let entries = std::fs::read_dir(path_buf.clone())?
                .map(|res| res.map(|e| {
                    jvalue!({
                        "path": e.path().to_str().unwrap(),
                        "is_dir": e.path().is_dir()
                    })
                }).or::<JuizError>(Ok(jvalue!("Error"))).unwrap())
                .collect::<Vec<Value>>();
            Ok(entries.into())
        }
    }
    
    /// サブシステムの追加
    /// 
    /// 
    /// 
    fn system_add_subsystem(&mut self, profile: Value) -> JuizResult<Value> {
        log::debug!("system_add_subsystem({profile}) called");
        // 相手方のBrokerProxyを作成
        //let bp = self.system_store.create_broker_proxy(self.worker(), &profile)?;
        // 相手のUUIDを得る。
        let (confirmation_request, uuid_value, bp) = match profile.as_object().unwrap().get("mastersystem") {
            Some(msv) => {
                log::trace!("found uuid in the passed profile");
                let bp = self.system_store.create_broker_proxy(self.worker(), &msv)?;
                match msv.as_object().unwrap().get("uuid") {
                    Some(v) => {
                        Ok((true, v.clone(), bp))
                    }
                    None => Err(anyhow!(JuizError::InvalidArgumentError { message: "system_add_subsystem failed".to_owned() }))
                }
            }
            None => {
                log::trace!("Not found uuid in the passed profile");

                let bp = self.system_store.create_broker_proxy(self.worker(), &profile)?;
                let v = bp.lock().or_else(|_e|{
                    Err(anyhow!(JuizError::ObjectLockError { target: "system_store".to_owned() }))
                }).and_then(|b|{ 
                    let v = b.system_uuid()?;
                   Ok(v)
                })?;
                Ok((false, v, bp))
            }
        }?;
        // 相手のuuidをUuid型に変換
        let uuid_str = uuid_value.as_str().unwrap();
        let uuid: Uuid = Uuid::parse_str(uuid_str).unwrap();
        // ここですでにuuidが登録されているかを確認する。
        for subsystem_proxy in self.subsystem_proxies.iter() {
            if &uuid == subsystem_proxy.uuid() {
                log::error!("system_add_subsystem failed. Subsystem(uuid={uuid}) has already added.");
                return Err(anyhow!(JuizError::ObjectAlreadyRegisteredError{message: format!("system_add_subsystem failed. Subsystem(uuid={uuid}) has already added.")}));
            }
        }
        // さらにサブシステムのサブシステムまでこれから登録するUUIDがあるかみようとするけど、これは無意味かも。
        // ループ構造ができないようにする責任は設計者にある
        for subsystem_proxy in self.subsystem_proxies.iter() {
            let ss = subsystem_proxy.subsystems()?;
            log::warn!("WARNING: SUBSYSTEM's SUBSYSTEM mining.... But this is useless...");
            log::warn!("value is {ss:}");
        }

        // 自分のUUID
        let my_uuid = self.system_store.uuid()?;
        self.worker_mut().store_mut().broker_proxies.register(bp.clone())?; // 作った相手方のBrokerProxyを自分に登録しておく
        let subsystem_proxy = SubSystemProxy::new(uuid, bp.clone())?;
        let ssprofile = juiz_lock(&subsystem_proxy.broker_proxy())?.profile_full().context("subsystem_proxy.broker_proxy().profile_full() in system_add_subsystem")?;
        // 相手方がどのIPアドレスを辿ってきたかがaccessed_broker_idでわかる
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
        log::info!("Added subsystem({})", ssprofile);
        //log::info!("accessed_broker_id = {}", accessed_broker_id);

        // 相手にこちら側のBrokerの名前を教えるために検索
        let broker_type = ssprofile.as_object().unwrap().get("type_name").unwrap().as_str().unwrap();
        let mut broker_name: Option<String> = None;
        for (_type_name, prof) in self.worker().store().brokers_profile_full()?.as_object().unwrap().iter() {
            let broker_broker_type = prof.as_object().unwrap().get("type_name").unwrap().as_str().unwrap();
            log::trace!("system includes broker ({broker_broker_type})");
            if broker_broker_type == broker_type {
                broker_name = Some(prof.as_object().unwrap().get("name").unwrap().as_str().unwrap().to_owned());
            }
        }
        if broker_name.is_none() {
            log::error!("Broker (type={broker_type}) can not be found.");
            return Err(anyhow!(JuizError::InvalidArgumentError { message: "system_add_subsystem() failed. Invalid argument".to_owned() }))
        }
        
        // 確認のためのリクエストでなければ
        if !confirmation_request {
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
        }
        // 最後にサブシステムのProxyを登録しておく。
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
                    None => {
                        log::error!(" - no record 'subsystem' in request argument for add_mastersystem()");
                        Err(anyhow!(JuizError::InvalidIdentifierError { message: "".to_owned() }))
                    } 
                }
            },
            None => {

                log::error!(" - the request argument for add_mastersystem() is not object type.");
                Err(anyhow!(JuizError::ValueIsNotObjectError { value: profile.clone() }))
            }
        }?;
        let uuid_value: Value = match obj_get_obj(&profile, "subsystem")?.get("uuid") {
            Some(v) =>  {
                log::trace!(" - subsystem uuid is {:?}", v);
                Ok(v.clone())
            },
            None => {
                log::trace!(" - no uuid found in request. This is the first request. Send subsystem add_subsystem request.");
                let my_uuid = self.system_store.uuid()?;
                let bprof = bp.lock().unwrap().profile_full()?;
                let broker_type_name = bprof.as_object().unwrap().get("type_name").unwrap().as_str().unwrap();
                let broker_prof = self.broker_list(false)?.as_array().unwrap().iter().find(|x| {
                    let idstruct: IdentifierStruct = IdentifierStruct::from_broker_identifier(&x.as_str().unwrap().to_owned()).unwrap();
                    log::debug!(" --- {:?}", idstruct);
                    idstruct.broker_type_name == broker_type_name
                }).unwrap().clone();
                log::trace!(" - broker_profile:{broker_prof:?}");
                let idstruct: IdentifierStruct = IdentifierStruct::from_broker_identifier(&broker_prof.as_str().unwrap().to_owned()).unwrap();
                    
                let broker_name = idstruct.object_name;
                juiz_lock(&bp)?.system_add_subsystem(jvalue!({
                    "mastersystem": {
                        "uuid": my_uuid.to_string(),
                        "type_name": broker_type_name,
                        "name": broker_name,
                    }
                }))?;
                juiz_lock(&bp)?.system_uuid()
            },
        }?;
        let uuid_str = uuid_value.as_str().unwrap();
        let uuid: Uuid = Uuid::parse_str(uuid_str).unwrap();
        
        self.worker_mut().store_mut().broker_proxies.register(bp.clone())?;
        log::info!("Add mastersystem(uuid={uuid_str})");
        let subsystem_proxy = SubSystemProxy::new(uuid, bp)?;
        self.master_system_proxy = Some(subsystem_proxy);
        Ok(profile)
    }
    
    fn system_load_process(&mut self, language: String, filepath: String) -> JuizResult<Value> {
        log::trace!("system_load_process({language}, {filepath}) called");
        self.worker_mut().load_process_factory(language, filepath)
    }

    fn system_load_container(&mut self, language: String, filepath: String) -> JuizResult<Value> {
        log::trace!("system_load_container({language}, {filepath}) called");
        self.worker_mut().load_container_factory(language, filepath)
    }

    fn system_load_container_process(&mut self, language: String, filepath: String) -> JuizResult<Value> {
        log::trace!("system_load_container_process({language}, {filepath}) called");
        self.worker_mut().load_container_process_factory(language, filepath)
    }

    fn system_load_component(&mut self, language: String, filepath: String) -> JuizResult<Value> {
        log::trace!("system_load_component({language}, {filepath}) called");
        self.worker_mut().load_component(language, filepath)
    }

}


impl ProcessBrokerProxy for CoreBroker { 
    fn process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            self.worker().store().processes.get(id)?.lock()?.call(args)
        } else {
            self.worker().process_proxy_from_identifier(id, true)?.lock()?.call(args)
        }
    }

    fn process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        log::trace!("CoreBroker::process_execute({id:}) called");
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            self.worker().store().processes.get(id)?.lock()?.execute()
        } else {
            self.worker().process_proxy_from_identifier(id, true)?.lock()?.execute()
        }
    }

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        Ok(self.worker().store().processes.get(id)?.lock()?.profile_full()?.into())
    }

    fn process_list(&self, recursive: bool) -> JuizResult<Value> {
        log::trace!("process_list({recursive}) called");
        if !recursive {
            return Ok(self.worker().store().processes_id());
        } 

        let mut ids = self.worker().store().processes_id();
        match ids.as_array_mut() {
            Some(ids_arr) => {
                for ssp in self.subsystem_proxies.iter() {
                    log::trace!(" - process_list for subsystem({ssp:})");
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


    fn process_push_by(&self, id: &Identifier, arg_name: String, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.worker().store().processes.get(id)?.lock()?.push_by(arg_name.as_str(), value)
    }

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, connection_type: String, connection_id: Option<String>) -> JuizResult<Value> {
        let mut source_process_id_struct = IdentifierStruct::try_from(source_process_id.clone())?;
        source_process_id_struct.broker_type_name = "core".to_owned();
        source_process_id_struct.broker_name = "core".to_owned();

        let connection_manifest = ConnectionManifest::new(
            connection_type.as_str().into(),
            source_process_id_struct.to_identifier(),
            arg_name.to_owned(),
            destination_process_id.clone(),
            connection_id,
        );
        let destination_process = self.worker_mut().any_process_proxy_from_identifier(destination_process_id, true)?;
        //self.worker_mut().any_process_proxy_from_identifier(source_process_id)?.lock_mut()?.try_connect_to(destination_process, arg_name, manifest)
        
        Ok(self.worker_mut().any_process_proxy_from_identifier(source_process_id, true)?.lock_mut()?.try_connect_to(destination_process, connection_manifest)?.into())
    }

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, connection_type: String, connection_id: Option<String>) -> JuizResult<Value> {
        
        let mut destination_process_id_struct = IdentifierStruct::try_from(destination_process_id.clone())?;
        destination_process_id_struct.broker_type_name = "core".to_owned();
        destination_process_id_struct.broker_name = "core".to_owned();

        let connection_manifest = ConnectionManifest::new(
            connection_type.as_str().into(),
            source_process_id.clone(),
            arg_name.to_owned(),
            destination_process_id_struct.to_identifier(),
            connection_id,
        );
        let source_process = self.worker_mut().any_process_proxy_from_identifier(source_process_id, true)?;//self.store().processes.get(source_process_id)?;
        //self.worker_mut().any_process_proxy_from_identifier(destination_process_id)?.lock_mut()?.notify_connected_from(source_process, arg_name, manifest)
        Ok(self.worker_mut().any_process_proxy_from_identifier(destination_process_id, true)?.lock_mut()?.notify_connected_from(source_process, connection_manifest)?.into())
     }
     
    fn process_p_apply(&mut self, id: &Identifier, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        Ok(self.worker().store().processes.get(id)?.lock_mut()?.p_apply(arg_name, value)?.into())
    }
    
    fn process_create(&mut self, manifest: ProcessManifest) -> JuizResult<Value> {
        self.worker_mut().create_process_ref(manifest)?.lock()?.profile_full()
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
        let mut ids = self.worker().store().containers_id();
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
    
    fn container_create(&mut self, manifest: CapsuleMap) -> JuizResult<Value> {
        let type_name: String =  manifest.get("type_name")?.try_into()?;
       //let type_name = obj_get_str(&manifest, "type_name")?;
        self.worker_mut().create_container_ref(type_name.as_str(), manifest)?.lock()?.profile_full()
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
        let mut ids = self.worker().store().container_processes_id();
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
            self.worker().process_proxy_from_identifier(id, true)?.lock()?.call(args)
        }
    }

    fn container_process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        let idstruct = IdentifierStruct::try_from(id.clone())?;
        if idstruct.broker_type_name == "core" {
            self.worker().store().container_processes.get(id)?.lock().with_context(||format!("locking process(id={id:}) in CoreBroker::execute_process() function"))?.execute()
        } else {
            self.worker().process_proxy_from_identifier(id, true)?.lock()?.execute()
        }
    }
 
    fn container_process_create(&mut self, container_id: &Identifier, manifest: ProcessManifest) -> JuizResult<Value> {
        let container = self.worker_mut().container_from_identifier(container_id)?;
        self.worker_mut().create_container_process_ref(container, manifest)?.lock()?.profile_full()
    }
    
    fn container_process_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value> {
        log::trace!("container_process_destroy({}) called", identifier);
        self.worker_mut().destroy_container_process_ref(identifier)
    }
    
    fn container_process_p_apply(&mut self, id: &Identifier, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        Ok(self.worker().store().container_processes.get(id)?.lock_mut()?.p_apply(arg_name, value)?.into())
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
        let cons = self.worker().connection_profile_list()?;
        let mut ids_arr = cons.iter().map(|con_prof| { obj_get(con_prof, "identifier").unwrap().clone() }).collect::<Vec<Value>>();
        if recursive {
            for subsystem_proxy in self.subsystem_proxies.iter() {
                let plist = juiz_lock(&subsystem_proxy.broker_proxy())?.connection_list(recursive)?;
                for v in get_array(&plist)?.iter() {
                    let id = v.as_str().unwrap();
                    ids_arr.push(id.into());
                }
            }
        }
        Ok(jvalue!(ids_arr))
    }

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.worker().connection_profile_full(id.clone(), true)
    }

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Value> {
        log::trace!("CoreBroker::connection_create({manifest}) called");
        Ok(self.worker_mut().create_connection(manifest.try_into()?)?.into())
    }
    
    fn connection_destroy(&mut self, _id: &Identifier) -> JuizResult<Value> {
        todo!()
    }
}

// fn check_if_both_side_is_on_same_host(source_id: Identifier, destination_id: Identifier) -> JuizResult<(Identifier, Identifier)> {
//     log::trace!("check_if_both_side_is_on_same_host({source_id}, {destination_id}) called");
//     let mut source_id_struct = IdentifierStruct::try_from(source_id)?;
//     let mut destination_id_struct = IdentifierStruct::try_from(destination_id)?;
//     if (source_id_struct.broker_name == destination_id_struct.broker_name) &&
//         (source_id_struct.broker_type_name == destination_id_struct.broker_type_name) {
//         source_id_struct.broker_name = "core".to_owned();
//         source_id_struct.broker_type_name = "core".to_owned();
//         destination_id_struct.broker_name = "core".to_owned();
//         destination_id_struct.broker_type_name = "core".to_owned();
//     }
//     Ok((source_id_struct.to_identifier(), destination_id_struct.to_identifier()))
// }

// fn check_connection_source_destination(manifest: &Value) -> JuizResult<(Identifier, Identifier)> {
//     let source = obj_get(manifest, "source")?;
//     let destination = obj_get(manifest, "destination")?;

//     let source_id_result = obj_get_str(source, "identifier");
//     let destination_id_result = obj_get_str(destination, "identifier");
    
//     // まずIDが両方ともあったら、brokerが同じものを指していたらcore/coreに直して接続する
//     if source_id_result.is_ok() && destination_id_result.is_ok() {
//         return check_if_both_side_is_on_same_host(source_id_result.unwrap().to_owned(), destination_id_result.unwrap().to_owned());
//     }

//     // IDがない場合はProcessかContainerProcessかが曖昧だが一旦Processで
//     return Ok((id_from_manifest(source)?, id_from_manifest(destination)?))
// }

impl BrokerProxy for CoreBroker {

    fn is_in_charge_for_process(&self, id: &Identifier) -> JuizResult<bool> {
        Ok(self.worker().store().processes.get(id).is_ok())
    }
}


unsafe impl Send for CoreBroker {

}