//! juizにおける各オブジェクトのベースとなるtrait
//! 
//! 
use crate::prelude::*;
use crate::identifier::identifier_new;

#[derive(Clone, Debug)]
pub enum JuizObjectClass {

    Process(&'static str),
    Container(&'static str),
    ContainerProcess(&'static str),
    ProcessFactory(&'static str),
    ContainerFactory(&'static str),
    ContainerProcessFactory(&'static str),
    Connection(&'static str),
    ExecutionContext(&'static str),
    ExecutionContextFactory(&'static str),

    Broker(&'static str),
    BrokerFactory(&'static str),
    BrokerProxy(&'static str),
    BrokerProxyFactory(&'static str),
    System(&'static str),

    ProcessProxy(&'static str),
    Topic(&'static str),
}


impl JuizObjectClass {

    pub fn as_str(&self) -> &'static str {
        match *self {
            JuizObjectClass::Process(_) => "Process",
            JuizObjectClass::ProcessFactory(_) => "ProcessFactory",
            JuizObjectClass::Container(_) => "Container",
            JuizObjectClass::ContainerFactory(_) => "ContainerFactory",
            JuizObjectClass::ContainerProcess(_) => "ContainerProcess",
            JuizObjectClass::ContainerProcessFactory(_) => "ContainerProcessFactory",
            JuizObjectClass::Connection(_) => "Connection",
            JuizObjectClass::ExecutionContext(_) => "ExecutionContext",
            JuizObjectClass::ExecutionContextFactory(_) => "ExecutionContextFactory",

            JuizObjectClass::Broker(_) => "Broker",
            JuizObjectClass::BrokerFactory(_) => "BrokerFactory",
            JuizObjectClass::BrokerProxy(_) => "BrokerProxy",
            JuizObjectClass::BrokerProxyFactory(_) => "BrokerProxyFactory",

            JuizObjectClass::System(_) => "System",
            JuizObjectClass::ProcessProxy(_) => "ProcessProxy",
            JuizObjectClass::Topic(_) => "Topic", 
        }
    }
}

#[derive(Debug)]
pub struct ObjectCore {
    identifier: Identifier,
    class_name: JuizObjectClass,
    type_name: String,
    name: String,
    broker_type_name: String,
    broker_name: String,
}

impl Clone for ObjectCore {
    fn clone(&self) -> Self {
        Self { identifier: self.identifier.clone(), class_name: self.class_name.clone(), type_name: self.type_name.clone(), name: self.name.clone(), broker_type_name: self.broker_type_name.clone(), broker_name: self.broker_name.clone() }
    }
}

impl ObjectCore {
    
    pub fn new(identifier: Identifier, class_name: JuizObjectClass, type_name: &str, object_name: &str, broker_name: &str, broker_type_name: &str) -> ObjectCore{
        //let identifier = identifier_new(broker_type_name, broker_name, class_name.as_str(), type_name, object_name);
        ObjectCore { identifier, class_name, type_name: type_name.to_string(), name: object_name.to_string(), broker_type_name: broker_type_name.to_string(), broker_name: broker_name.to_string()}
    }

    pub fn create<T: ToString, D: ToString>(class_name: JuizObjectClass, type_name: T, object_name: D) -> ObjectCore{
        let identifier = identifier_new("core", "core", class_name.as_str(), type_name.to_string().as_str(), object_name.to_string().as_str());
        ObjectCore { identifier, class_name, type_name: type_name.to_string(), name: object_name.to_string(), broker_name: "core".to_string(), broker_type_name: "core".to_string()}
    }

    pub fn create_factory<T: ToString>(class_name: JuizObjectClass, type_name: T) -> ObjectCore{
        let identifier = identifier_new("core", "core", class_name.as_str(), type_name.to_string().as_str(), type_name.to_string().as_str());
        ObjectCore { identifier, class_name, type_name: type_name.to_string(), name: type_name.to_string(), broker_name: "core".to_string(), broker_type_name: "core".to_string()}
    }

    pub fn set_identifier(&mut self, id: Identifier) -> () {
        self.identifier = id
    }

    pub fn identifier(&self) -> Identifier {
        self.identifier.clone()
    }

    pub fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "identifier": self.identifier,
            "class_name": self.class_name.as_str(),
            "type_name": self.type_name,
            "name": self.name,
        }))
    }
}

pub trait JuizObjectCoreHolder {
    fn core(&self) -> &ObjectCore;
}

pub trait JuizObject : JuizObjectCoreHolder {

    fn identifier(&self) -> Identifier {
        self.core().identifier()
    }

    fn profile_full(&self) -> JuizResult<Value>{
        Ok(jvalue!({
            "identifier": self.identifier(),
            "class_name": self.class_name().as_str(),
            "type_name": self.type_name(),
            "name": self.name(),
        }).into())
    }

    fn class_name(&self) -> &JuizObjectClass {
        &self.core().class_name
    }

    fn type_name(&self) -> &str {
        self.core().type_name.as_str()
    }

    fn name(&self) -> &str {
        self.core().name.as_str()
    }

    fn broker_type(&self) -> &str {
        self.core().broker_type_name.as_str()
    }

    fn broker_name(&self) -> &str {
        self.core().broker_name.as_str()
    }
}