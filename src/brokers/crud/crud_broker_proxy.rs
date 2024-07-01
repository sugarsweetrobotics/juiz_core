use std::{collections::HashMap, sync::{Mutex, Arc}};

use crate::{brokers::{broker_proxy::{BrokerBrokerProxy, ConnectionBrokerProxy, ContainerBrokerProxy, ContainerProcessBrokerProxy, ExecutionContextBrokerProxy, ProcessBrokerProxy, SystemBrokerProxy}, BrokerProxy}, identifier::IdentifierStruct, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, processes::{capsule::CapsuleMap, capsule_to_value}, utils::{get_array, manifest_util::get_hashmap_mut}, CapsulePtr, Identifier, JuizError, JuizObject, JuizResult, Value};


pub trait CRUDBrokerProxy : Send + Sync {
    fn create(&self, class_name: &str, function_name: &str, payload: Value, param: HashMap<String, String>) -> JuizResult<CapsulePtr>;
    fn delete(&self, class_name: &str, function_name: &str, param: HashMap<String, String>) -> JuizResult<CapsulePtr>;
    fn read(&self, class_name: &str, function_name: &str, param: HashMap<String, String>) -> JuizResult<CapsulePtr>;
    fn update(&self, class_name: &str, function_name: &str, payload: CapsuleMap, param: HashMap<String, String>) -> JuizResult<CapsulePtr>;
}


pub struct CRUDBrokerProxyHolder {
    core: ObjectCore,
    broker: Box<dyn CRUDBrokerProxy>,
}

fn param(param_map: &[(&str, &str)]) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (k, v) in param_map.iter() {
        if *k == "identifier" {
            map.insert((*k).to_owned(), modify_id(*v));
        }
    }
    map
}

fn modify_id(id: &str) -> String {
    let mut id_struct = IdentifierStruct::from(id.to_owned());
    id_struct.broker_name = "core".to_owned();
    id_struct.broker_type_name = "core".to_owned();
    id.into()
}


impl CRUDBrokerProxyHolder {

    pub fn new(impl_class_name: &'static str, type_name: &str, name: &str, broker_proxy: Box<dyn CRUDBrokerProxy>) -> JuizResult<Arc<Mutex<CRUDBrokerProxyHolder>>> {

        Ok(Arc::new(Mutex::new(CRUDBrokerProxyHolder{
            core: ObjectCore::create(JuizObjectClass::BrokerProxy(impl_class_name), type_name, name),
            broker: broker_proxy,
        })))
    }

    fn convert_identifier_name(&self, id_array: &Value) -> JuizResult<Value> {
        let mut ids: Vec<String> = Vec::new();
        for vid in get_array(id_array)?.iter() {
            let id = vid.as_str().ok_or(anyhow::Error::from(JuizError::ValueIsNotStringError{}))?.to_owned();
            let mut id_struct = IdentifierStruct::from(id);
            id_struct.broker_type_name = self.type_name().to_owned();
            id_struct.broker_name = self.name().to_owned();
            ids.push(id_struct.into());
        }
        Ok(jvalue!(ids))
    }

    fn modify_profile(&self, capsule: CapsulePtr) -> CapsulePtr {
        let key_id = "identifier".to_owned();
        let _ = capsule.lock_modify_as_value(|v| {
            let map = get_hashmap_mut(v).unwrap();
            if map.contains_key(&key_id) {
                let id = map.get(&key_id).unwrap().as_str().unwrap().to_owned();
                let mut id_struct = IdentifierStruct::from(id);
                id_struct.broker_name = self.name().to_owned();
                id_struct.broker_type_name = self.type_name().to_owned();
                let new_id: Identifier = id_struct.into();
                map.insert(key_id, jvalue!(new_id));
            }
        });
        capsule
    }
}

impl JuizObjectCoreHolder for CRUDBrokerProxyHolder {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for CRUDBrokerProxyHolder {
    
}

impl ContainerProcessBrokerProxy for CRUDBrokerProxyHolder {
    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        log::info!("CRUDBrokerProxy.contaienr_process_profile_full({id}) called");
        let result = capsule_to_value(self.modify_profile(self.broker.read("container_process", "profile_full", param(&[("identifier", id)]))?));
        log::trace!("CRUDBrokerProxy.container_process_profile_full({id}) = {result:?}");
        return result;
    }

    fn container_process_list(&self) -> JuizResult<Value> {
        let v = capsule_to_value(self.broker.read("container_process", "list", HashMap::new())?)?;
        self.convert_identifier_name(&v)
    }
    
    fn container_process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.broker.update("container_process", "call", args, param(&[("identifier", id)]))
    }
    
    fn container_process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        self.broker.update("container_process", "execute", CapsuleMap::new(), param(&[("identifier", id)]))
    }
}


impl ContainerBrokerProxy for CRUDBrokerProxyHolder {
    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.modify_profile(self.broker.read("container", "profile_full", param(&[("identifier", id)]))?))
    }

    fn container_list(&self) -> JuizResult<Value> {
        let v = capsule_to_value(self.broker.read("container", "list", HashMap::new())?)?;
        self.convert_identifier_name(&v)
    }
}

impl ProcessBrokerProxy for CRUDBrokerProxyHolder {
    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        log::info!("CRUDBrokerProxy.process_profile_full({id}) called");
        let result = capsule_to_value(self.modify_profile(self.broker.read("process", "profile_full", param(&[("identifier", id)]))?));
        log::trace!("CRUDBrokerProxy.process_profile_full({id}) = {result:?}");
        return result;
    }

    fn process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.broker.update("process", "call", args, param(&[("identifier", id)]))
    }

    fn process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        self.broker.update("process", "execute", CapsuleMap::new(), param(&[("identifier", id)]))
    }

    fn process_list(&self) -> JuizResult<Value> {
        let v = capsule_to_value(self.broker.read("process", "list", HashMap::new())?)?;
        self.convert_identifier_name(&v)
    }

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        capsule_to_value(self.broker.update(
            "process", 
            "try_connect_to", 
            CapsuleMap::try_from(jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }))?, 
            HashMap::from([]))?)
    }

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        capsule_to_value(self.broker.update(
            "process", 
            "notify_connected_from", 
            CapsuleMap::try_from(jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }))?, 
            HashMap::from([]))?)
    }
    
    fn process_bind(&mut self, id: &Identifier, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        let mut map = CapsuleMap::new();
        map.insert("arg_name".to_owned(), jvalue!(arg_name).into());
        map.insert("value".to_owned(), value);
        self.broker.update("process", "bind", map, param(&[("identifier", id)]))
    }
}

impl SystemBrokerProxy for CRUDBrokerProxyHolder {
    fn system_profile_full(&self) -> JuizResult<Value> {
        capsule_to_value(self.modify_profile(self.broker.read("system", "profile_full", HashMap::new())?))
    }

    fn system_filesystem_list(&self, path_buf: std::path::PathBuf) -> JuizResult<Value> {
        capsule_to_value(self.broker.read("system", "profile_full", HashMap::from([("path".to_owned(), path_buf.to_str().unwrap().to_owned())]) )?)
    }
}

impl BrokerBrokerProxy for CRUDBrokerProxyHolder {
    fn broker_list(&self) -> JuizResult<Value> {
        capsule_to_value(self.broker.read("broker", "list", HashMap::new())?)
    }

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.modify_profile(self.broker.read("broker", "profile_full", param(&[("identifier", id)]))?))
    }
}

impl ExecutionContextBrokerProxy for CRUDBrokerProxyHolder {
    fn ec_list(&self) -> JuizResult<Value> {
        capsule_to_value(self.broker.read("execution_context", "list", HashMap::new())?)
    }

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Value> { 
        capsule_to_value(self.modify_profile(self.broker.read("execution_context", "profile_full", param(&[("identifier", id)]))?))
    }

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.broker.read("execution_context", "get_state", param(&[("identifier", id)]))?)
    }

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.broker.update("execution_context", "start", CapsuleMap::new(), param(&[("identifier", id)]))?)
    }

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.broker.update("execution_context", "stop",  CapsuleMap::new(), param(&[("identifier", id)]))?)
    }
}

impl ConnectionBrokerProxy for CRUDBrokerProxyHolder {
    fn connection_list(&self) -> JuizResult<Value> {
        capsule_to_value(self.broker.read("connection", "list", HashMap::new())?)
    }

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.broker.read("connection", "profile_full", param(&[("identifier", id)]))?)
    }

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Value> {
        capsule_to_value(self.broker.create("connection", "create", manifest, HashMap::new())?)
    }
}

impl BrokerProxy for CRUDBrokerProxyHolder {
    fn is_in_charge_for_process(&self, _id: &Identifier) -> JuizResult<bool> {
        todo!()
    }
}