
use crate::{processes::capsule::{Capsule, CapsuleMap}, Identifier, JuizObject, JuizResult, Value};




pub trait SystemBrokerProxy {
    fn system_profile_full(&self) -> JuizResult<Capsule>;
}

pub trait ProcessBrokerProxy {

    fn process_list(&self) -> JuizResult<Capsule>;

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Capsule>;

    fn process_call(&self, id: &Identifier, _args: CapsuleMap) -> JuizResult<Capsule>;

    fn process_execute(&self, id: &Identifier) -> JuizResult<Capsule>;

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Capsule>;

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Capsule>;
}

pub trait ContainerBrokerProxy {

    fn container_list(&self) -> JuizResult<Capsule>;

    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Capsule>;
}

pub trait ContainerProcessBrokerProxy {

    fn container_process_list(&self) -> JuizResult<Capsule>;

    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Capsule>;

    fn container_process_call(&self, id: &Identifier, _args: CapsuleMap) -> JuizResult<Capsule>;

    fn container_process_execute(&self, id: &Identifier) -> JuizResult<Capsule>;
}

pub trait ExecutionContextBrokerProxy {
    
    fn ec_list(&self) -> JuizResult<Capsule>;

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Capsule>;

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Capsule>;

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Capsule>;

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Capsule>;
}
pub trait BrokerBrokerProxy {
    
    fn broker_list(&self) -> JuizResult<Capsule>;

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Capsule>;
}

pub trait ConnectionBrokerProxy {

    fn connection_list(&self) -> JuizResult<Capsule>;

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Capsule>;

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Capsule>;

}

pub trait BrokerProxy : Send + JuizObject + SystemBrokerProxy + ProcessBrokerProxy + ContainerBrokerProxy + ContainerProcessBrokerProxy + ExecutionContextBrokerProxy + BrokerBrokerProxy + ConnectionBrokerProxy {

    fn is_in_charge_for_process(&self, _id: &Identifier) -> JuizResult<bool>;

    fn any_process_list(&self) -> JuizResult<Capsule> {
        todo!()
    }

    fn any_process_profile_full(&self, id: &Identifier) -> JuizResult<Capsule> {
        log::info!("BrokerProxy.any_process_profile_full({id}) called");
        let result = self.process_profile_full(id).or_else(|_e| {
            println!("Error {_e}");
            self.container_process_profile_full(id)
        });
        result
    }
}