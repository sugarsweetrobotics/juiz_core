use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::{anyhow::anyhow, core::SubSystemProxy, object::JuizObjectClass, prelude::*, proc_lock};
pub type TopicName = String;

#[derive(Clone)]
#[allow(unused)]
pub struct Topic {
    name: TopicName,
    subsystem_proxies: Vec<SubSystemProxy>,
}

#[allow(unused)]
impl Topic {

    pub fn new(name: &str) -> Self {
        Self{name: name.to_owned(), subsystem_proxies: Vec::new()}
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}


#[derive(Clone)]
#[allow(unused)]
pub struct TopicPtr {
    name: String,
    system_uuid: Uuid,
    topic: Arc<RwLock<Topic>>,
    ptr: Arc<RwLock<dyn Process>>,
}


#[cfg(feature="opencv4")]
fn capsule_ptr_to_capsule(v: &CapsulePtr) -> JuizResult<Capsule> {
    if v.is_value()? {
        v.lock_as_value(|v| -> Capsule { Capsule::from(v.clone()) } )
    } else if v.is_mat()? {
        v.lock_as_mat(|m| -> Capsule { Capsule::from(m.clone()) })
    } else {
        Err(anyhow!(JuizError::ArgumentError { message: "CapsulePtr is not available for Topic".to_owned() }))
    }
}

#[cfg(not(feature="opencv4"))]
fn capsule_ptr_to_capsule(v: &CapsulePtr) -> JuizResult<Capsule> {
    if v.is_value()? {
        v.lock_as_value(|v| -> Capsule { Capsule::from(v.clone()) } )
    } else {
        Err(anyhow!(JuizError::ArgumentError { message: "CapsulePtr is not available for Topic".to_owned() }))
    }
    /* else if v.is_image()? {
        v.lock_as_mat(|m| -> Capsule { Capsule::from(m.clone()) })
    }; */
}

impl TopicPtr {

    pub fn new(name: &str, system_uuid: Uuid) -> Self {
        log::trace!("new(name={name}, uuid={system_uuid}) called");
        let manifest = jvalue!({
            "type_name": "topic",
            "topic_name": name,
            "name": name,
            "use_memo": false,
            "arguments": {
                "input": {
                    "default": {},
                    "type": "object",
                }
            }
        });

        let topic = Arc::new(RwLock::new(Topic::new(name)));
        let my_topic_name = name.to_owned();
        let my_topic = topic.clone();
        let my_uuid = system_uuid.clone();
        let topic_func = move |arg: CapsuleMap| -> JuizResult<Capsule> {
            //println!("Topic {my_topic:?}");
            log::trace!("Topic ({my_topic_name}) / topic_func called");
            let v = arg.get("input")?;
            let result = capsule_ptr_to_capsule(&v);
            log::trace!("- value is copied");
            match my_topic.read() {
                Ok(t) => {
                    log::trace!(" - my_topic.read() OK.");
                    for subsystem in t.subsystem_proxies.iter() {
                        log::trace!("- broker_proxy: subsystem={:?}", subsystem.uuid());
                        match juiz_lock(&subsystem.broker_proxy())?.topic_push(my_topic_name.as_str(), v.clone(), Some(my_uuid)) {
                            Ok(_) => {
                                log::trace!("SubsystemProxy.topic_push() success");
                                Ok(())
                            }
                            Err(e) => {
                                log::error!("Error {e} occurred in SubsystemProxy.topic_push()");
                                Err(e)
                            }
                        }?;
                    }
                },
                Err(_) => {
                    log::error!("my_topic.read() failed.");
                    return Err(anyhow!(JuizError::ObjectLockError { target: "Topic".to_owned() }))
                }
            }
            log::trace!("Topic ({my_topic_name}) / topic_func exit");
            result
        };
        TopicPtr{
            name: name.to_owned(),
            system_uuid,
            topic,
            ptr: Arc::new(RwLock::new(
                ProcessImpl::clousure_new_with_class_name(JuizObjectClass::Topic("Topic"), manifest, Box::new(topic_func)).unwrap()))
        
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn process_ptr(&self) ->ProcessPtr {
        self.ptr.clone()
    }

    pub fn profile_full(&self) -> JuizResult<Value> {
        self.ptr.read().or_else(|_|{Err(anyhow!(JuizError::ObjectLockError { target: "TopicPtr".to_owned() }))})
            .and_then(|p| { p.profile_full() })
    }

    pub fn push(&self, capsule: CapsulePtr, pushed_system_uuid: Option<Uuid>) -> JuizResult<()> {
        log::trace!("push(uuid={pushed_system_uuid:?}) called");
        let r = proc_lock(&self.ptr)?.push_by("input", capsule).and_then(|_|{Ok(())});
        log::trace!("push(uuid={pushed_system_uuid:?}) exit");
        r
    }

    pub fn num_local_publishers(&self) -> JuizResult<usize> {
        Ok(proc_lock(&self.ptr)?.source_connections()?.len())
    }

    pub fn num_local_subscribers(&self) -> JuizResult<usize> {
        Ok(proc_lock(&self.ptr)?.destination_connections()?.len())
    }

    pub fn register_subscriber_subsystem(&self, subsystem_proxy: SubSystemProxy) -> JuizResult<()> {
        log::trace!("register_subscriber_subsystem(name={}, subsystem_proxy={}) called", self.name(), subsystem_proxy.uuid());
        match self.topic.write() {
            Ok(mut t) => {
                t.subsystem_proxies.push(subsystem_proxy);
                Ok(())
            },  
            Err(_e) => {
                Err(anyhow!(JuizError::ObjectLockError { target: "Topic".to_owned() }))
            }
        }
    }

    /*
    pub fn register_publisher_subsystem(&self, subsystem_proxy: SubSystemProxy) -> JuizResult<()> {
        log::trace!("register_publisher_subsystem(name={}, subsystem_proxy={}) called", self.name(), subsystem_proxy.uuid());
        match self.topic.write() {
            Ok(mut t) => {
                for subsystem in t.subsystem_proxies.iter() {
                    if subsystem.uuid() == subsystem_proxy.uuid() { // もうすでにpublish登録している
                        return Ok(())
                    }
                }
                t.subsystem_proxies.push(subsystem_proxy);
                Ok(())
            },  
            Err(_e) => {
                Err(anyhow!(JuizError::ObjectLockError { target: "Topic".to_owned() }))
            }
        }
    }
    */
}