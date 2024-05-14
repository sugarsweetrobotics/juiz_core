use std::{collections::{hash_map::Values, HashMap}, sync::{Arc, Mutex, RwLock}};

use crate::{brokers::{BrokerProxy, BrokerProxyFactory}, ecs::{execution_context_holder::ExecutionContextHolder, execution_context_holder_factory::ExecutionContextHolderFactory}, jvalue, utils::{juiz_lock, manifest_util::{get_array_mut, get_hashmap_mut}, sync_util::juiz_try_lock}, value::obj_get_str, Container, ContainerFactory, ContainerProcessFactory, Identifier, JuizError, JuizObject, JuizResult, Process, ProcessFactory, Value};



pub struct RwStoreWorker<T, TF> where T: JuizObject + ?Sized, TF: JuizObject + ?Sized {
    name: String,
    factories: HashMap<String, Arc<Mutex<TF>>>,
    objects: HashMap<Identifier, Arc<RwLock<T>>> ,
}

impl<T, TF> RwStoreWorker<T, TF> where T: JuizObject + ?Sized, TF: JuizObject + ?Sized {

    pub fn new(name: &str) -> Box<RwStoreWorker<T, TF>> {
        Box::new(RwStoreWorker { name: name.to_string(), factories: HashMap::new(), objects: HashMap::new() })
    }

    pub fn objects(&self) -> Values<String, Arc<RwLock<T>>> {
        self.objects.values()
    }

    pub fn register_factory(&mut self, pf: Arc<Mutex<TF>>) -> JuizResult<Arc<Mutex<TF>>> {
        let type_name = juiz_lock(&pf)?.type_name().to_string();
        log::trace!("StoreWorker({})::registerfactory(Factory(type_name={:?})) called",self.name,type_name);
        if self.factories.contains_key(&type_name) {
            return Err(anyhow::Error::from(JuizError::FactoryOfSameTypeNameAlreadyExistsError{type_name: type_name}));
        }
        self.factories.insert(type_name, Arc::clone(&pf));
        Ok(pf)
    }

    pub fn factory(&self, type_name: &str) -> JuizResult<&Arc<Mutex<TF>>> {
        match self.factories.get(type_name) {
            None => return Err(anyhow::Error::from(JuizError::FactoryCanNotFoundError{type_name: type_name.to_string()})),
            Some(pf) => return Ok(pf)
        }
    }

    pub fn register(&mut self, p: Arc<RwLock<T>>) -> JuizResult<Arc<RwLock<T>>> {
        // let id = proc_lock(&p)?.identifier().clone();
        let id = p.read().unwrap().identifier().clone();
        log::trace!("StoreWorker({})::register(Process(id={:?})) called", self.name, id);
        self.objects.insert(id.clone(), p);
        self.get(&id)
    }

    pub fn get(&self, id: &Identifier) -> JuizResult<Arc<RwLock<T>>> {
        match self.objects.get(id) {
            Some(p) => Ok(Arc::clone(p)),
            None => {
                log::trace!("StoreWorker({})::get(id={:?}) failed.", self.name, id);
                log::trace!(" - CoreStore includes processes[");
                for (k, _v) in self.objects.iter() {
                    log::trace!("    - {:?}", k);
                }
                log::trace!("]");
                Err(anyhow::Error::from(JuizError::ObjectCanNotFoundByIdError{id: id.clone()}))
            }
            
        }
    }

    pub fn factories_profile_full(&self) -> JuizResult<Value> {
        let mut prof = jvalue!({});
        let o_hashmap = get_hashmap_mut(&mut prof)?;
        self.factories.iter().for_each(|(identifier, arc_obj)| {
            match juiz_lock(&arc_obj) {
                Err(e) => {
                    o_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(p) => {
                    o_hashmap.insert(identifier.clone(), p.profile_full().unwrap().try_into().unwrap());
                }
            }
        });
        Ok(prof)
    }

    pub fn objects_profile_full(&self) -> JuizResult<Value> {
        let name = &self.name;
        log::trace!("StoreWorker({name})::objects_profile_full() called");
        let mut prof = jvalue!({});
        let o_hashmap = get_hashmap_mut(&mut prof)?;
        self.objects.iter().for_each(|(identifier, arc_obj)| {
            log::trace!(" - {identifier}");
            /* 
            match juiz_try_lock(&arc_obj) {
                Err(e) => {
                    o_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(p) => {
                    o_hashmap.insert(identifier.clone(), p.profile_full().unwrap().try_into().unwrap());
                }
            }
            */
            o_hashmap.insert(identifier.clone(), arc_obj.read().unwrap().profile_full().unwrap().try_into().unwrap());
        });
        Ok(prof)
    }

    pub fn list_ids(&self) -> JuizResult<Value> {
        let mut prof = jvalue!([]);
        let o_array = get_array_mut(&mut prof)?;
        self.objects.iter().for_each(|(identifier, _arc_obj)| {
            /*match juiz_lock(&arc_obj) {
                Err(e) => {
                    o_array.push(jvalue!(format!("Err({})", e)));
                },
                Ok(_p) => {
                    o_array.push(jvalue!(identifier));
                }
            }*/
            o_array.push(jvalue!(identifier));

        });
        Ok(prof)
    }

    pub fn cleanup_objects(&mut self) -> JuizResult<()> {
        self.objects.clear();
        Ok(())
    }
}


pub struct StoreWorker<T, TF> where T: JuizObject + ?Sized, TF: JuizObject + ?Sized {
    name: String,
    factories: HashMap<String, Arc<Mutex<TF>>>,
    objects: HashMap<Identifier, Arc<Mutex<T>>> ,
}

impl<T, TF> StoreWorker<T, TF> where T: JuizObject + ?Sized, TF: JuizObject + ?Sized {

    pub fn new(name: &str) -> Box<StoreWorker<T, TF>> {
        Box::new(StoreWorker { name: name.to_string(), factories: HashMap::new(), objects: HashMap::new() })
    }

    pub fn objects(&self) -> Values<String, Arc<Mutex<T>>> {
        self.objects.values()
    }

    pub fn register_factory(&mut self, pf: Arc<Mutex<TF>>) -> JuizResult<Arc<Mutex<TF>>> {
        let type_name = juiz_lock(&pf)?.type_name().to_string();
        log::trace!("StoreWorker({})::registerfactory(Factory(type_name={:?})) called",self.name,type_name);
        if self.factories.contains_key(&type_name) {
            return Err(anyhow::Error::from(JuizError::FactoryOfSameTypeNameAlreadyExistsError{type_name: type_name}));
        }
        self.factories.insert(type_name, Arc::clone(&pf));
        Ok(pf)
    }

    pub fn factory(&self, type_name: &str) -> JuizResult<&Arc<Mutex<TF>>> {
        match self.factories.get(type_name) {
            None => return Err(anyhow::Error::from(JuizError::FactoryCanNotFoundError{type_name: type_name.to_string()})),
            Some(pf) => return Ok(pf)
        }
    }

    pub fn register(&mut self, p: Arc<Mutex<T>>) -> JuizResult<Arc<Mutex<T>>> {
        let id = juiz_lock(&p)?.identifier().clone();
        log::trace!("StoreWorker({})::register(Process(id={:?})) called", self.name, id);
        self.objects.insert(id.clone(), p);
        self.get(&id)
    }

    pub fn get(&self, id: &Identifier) -> JuizResult<Arc<Mutex<T>>> {
        match self.objects.get(id) {
            Some(p) => Ok(Arc::clone(p)),
            None => {
                log::trace!("StoreWorker({})::get(id={:?}) failed.", self.name, id);
                log::trace!(" - CoreStore includes processes[");
                for (k, _v) in self.objects.iter() {
                    log::trace!("    - {:?}", k);
                }
                log::trace!("]");
                Err(anyhow::Error::from(JuizError::ObjectCanNotFoundByIdError{id: id.clone()}))
            }
            
        }
    }

    pub fn factories_profile_full(&self) -> JuizResult<Value> {
        let mut prof = jvalue!({});
        let o_hashmap = get_hashmap_mut(&mut prof)?;
        self.factories.iter().for_each(|(identifier, arc_obj)| {
            match juiz_lock(&arc_obj) {
                Err(e) => {
                    o_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(p) => {
                    o_hashmap.insert(identifier.clone(), p.profile_full().unwrap().try_into().unwrap());
                }
            }
        });
        Ok(prof)
    }

    pub fn objects_profile_full(&self) -> JuizResult<Value> {
        let name = &self.name;
        log::trace!("StoreWorker({name})::objects_profile_full() called");
        let mut prof = jvalue!({});
        let o_hashmap = get_hashmap_mut(&mut prof)?;
        self.objects.iter().for_each(|(identifier, arc_obj)| {
            log::trace!(" - {identifier}");
            match juiz_try_lock(&arc_obj) {
                Err(e) => {
                    o_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(p) => {
                    o_hashmap.insert(identifier.clone(), p.profile_full().unwrap().try_into().unwrap());
                }
            }
        });
        Ok(prof)
    }

    pub fn list_ids(&self) -> JuizResult<Value> {
        let mut prof = jvalue!([]);
        let o_array = get_array_mut(&mut prof)?;
        self.objects.iter().for_each(|(identifier, _arc_obj)| {
            /*match juiz_lock(&arc_obj) {
                Err(e) => {
                    o_array.push(jvalue!(format!("Err({})", e)));
                },
                Ok(_p) => {
                    o_array.push(jvalue!(identifier));
                }
            }*/
            o_array.push(jvalue!(identifier));

        });
        Ok(prof)
    }

    pub fn cleanup_objects(&mut self) -> JuizResult<()> {
        self.objects.clear();
        Ok(())
    }
}

pub struct CoreStore {
    broker_factories_manifests: HashMap<Identifier, Value>,
    brokers_manifests: HashMap<Identifier, Value>,

    pub processes: Box<RwStoreWorker::<dyn Process, dyn ProcessFactory>>,
    pub containers: Box<RwStoreWorker::<dyn Container, dyn ContainerFactory>>,
    pub container_processes: Box<RwStoreWorker::<dyn Process, dyn ContainerProcessFactory>>,
    pub ecs: Box<StoreWorker::<ExecutionContextHolder, ExecutionContextHolderFactory>>,
    pub broker_proxies: Box<StoreWorker::<dyn BrokerProxy, dyn BrokerProxyFactory>>,
}


impl CoreStore {
    pub fn new() -> CoreStore {
        CoreStore{
            brokers_manifests: HashMap::new(),
            broker_proxies: StoreWorker::new("broker_proxy"),
            broker_factories_manifests: HashMap::new(),
            
            processes: RwStoreWorker::new("process"), 
            containers: RwStoreWorker::new("container"), 
            container_processes: RwStoreWorker::new("container_process"), 
            ecs: StoreWorker::new("ecs"),
        }
    }

    pub fn register_broker_manifest(&mut self, type_name: &str, b: Value) -> JuizResult<()> {
        self.brokers_manifests.insert(type_name.to_string(), b);
        Ok(())
    }

    pub fn register_broker_factory_manifest(&mut self, type_name: &str, b: Value) -> JuizResult<()> {
        log::trace!("core_store::register_broker_factory_manifest(type_name={type_name:?}) called");
        self.broker_factories_manifests.insert(type_name.to_string(), b);
        Ok(())
    }

    
    fn broker_factories_profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!(self.broker_factories_manifests))
    }

    pub fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        match self.brokers_manifests.get(id) {
            Some(p) => Ok(p.clone()),
            None => {
                Err(anyhow::Error::from(JuizError::BrokerProfileNotFoundError{id: id.to_string()}))
            }
        }
    }

    pub fn brokers_profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!(self.brokers_manifests))
    }

    pub fn brokers_list_ids(&self) -> JuizResult<Value> {
        let vec_value = self.brokers_manifests.values().collect::<Vec<&Value>>();
        let vec_str = vec_value.iter().map(|pv| { obj_get_str(*pv, "identifier").unwrap().to_string() }).collect::<Vec<String>>();
        Ok(jvalue!(vec_str))
    }

    pub fn profile_full(&self) -> JuizResult<Value> {
        
        Ok(jvalue!({
            "process_factories": self.processes.factories_profile_full()?,
            "container_factories": self.containers.factories_profile_full()?,
            "container_process_factories": self.container_processes.factories_profile_full()?,
            "processes": self.processes.objects_profile_full()?,
            "containers": self.containers.objects_profile_full()?,
            "container_processes": self.container_processes.objects_profile_full()?,
            "brokers": self.brokers_profile_full()?,
            "broker_factories": self.broker_factories_profile_full()?,
            "ecs": self.ecs.objects_profile_full()?,
            "ec_factories": self.ecs.factories_profile_full()?,
        }))
    }
}