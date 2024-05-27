


use crate::{identifier::IdentifierStruct, processes::capsule::{Capsule, CapsuleMap}, CapsulePtr, Identifier, JuizObject, JuizResult, Value};




pub trait SystemBrokerProxy {
    fn system_profile_full(&self) -> JuizResult<Value>;
}

pub trait ProcessBrokerProxy {

    fn process_list(&self) -> JuizResult<Value>;

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn process_call(&self, id: &Identifier, _args: CapsuleMap) -> JuizResult<CapsulePtr>;

    fn process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr>;

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value>;

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value>;
}

pub trait ContainerBrokerProxy {

    fn container_list(&self) -> JuizResult<Value>;

    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Value>;
}

pub trait ContainerProcessBrokerProxy {

    fn container_process_list(&self) -> JuizResult<Value>;

    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn container_process_call(&self, id: &Identifier, _args: CapsuleMap) -> JuizResult<CapsulePtr>;

    fn container_process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr>;
}

pub trait ExecutionContextBrokerProxy {
    
    fn ec_list(&self) -> JuizResult<Value>;

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Value>;

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Value>;

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Value>;
}
pub trait BrokerBrokerProxy {
    
    fn broker_list(&self) -> JuizResult<Value>;

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value>;
}

pub trait ConnectionBrokerProxy {

    fn connection_list(&self) -> JuizResult<Value>;

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Value>;

}

pub trait BrokerProxy : Send + JuizObject + SystemBrokerProxy + ProcessBrokerProxy + ContainerBrokerProxy + ContainerProcessBrokerProxy + ExecutionContextBrokerProxy + BrokerBrokerProxy + ConnectionBrokerProxy {

    fn is_in_charge_for_process(&self, _id: &Identifier) -> JuizResult<bool>;

    fn any_process_list(&self) -> JuizResult<Capsule> {
        todo!()
    }

    fn any_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        log::info!("BrokerProxy::any_process_profile_full({id}) called");
        let id_struct = IdentifierStruct::from(id.clone());
        log::info!("id_struct{:?}", id_struct);        
        if id_struct.class_name == "Process" {
            return self.process_profile_full(id)
        }
        self.container_process_profile_full(id)
    }

    fn any_process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        log::info!("BrokerProxy::any_process_profile_call({id}) called");
        let id_struct = IdentifierStruct::from(id.clone());
        // log::info!("id_struct{:?}", id_struct);        
        if id_struct.class_name == "Process" {
            return self.process_call(id, args)
        }
        self.container_process_call(id, args)
    }
}