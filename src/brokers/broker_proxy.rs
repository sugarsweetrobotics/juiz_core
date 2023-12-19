
use crate::{Identifier, Value, JuizResult, JuizObject};




pub trait SystemBrokerProxy {
    fn system_profile_full(&self) -> JuizResult<Value>;
}

pub trait ProcessBrokerProxy {

    fn process_list(&self) -> JuizResult<Value>;

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn process_call(&self, id: &Identifier, _args: Value) -> JuizResult<Value>;

    fn process_execute(&self, id: &Identifier) -> JuizResult<Value>;

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
}

pub trait ExecutionContextBrokerProxy {
    
    fn ec_list(&self) -> JuizResult<Value>;

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Value>;
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

    fn any_process_list(&self) -> JuizResult<Value> {
        todo!()
    }

    fn any_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        log::info!("BrokerProxy.any_process_profile_full({id}) called");
        let result = self.process_profile_full(id).or_else(|_e| {
            println!("Error {_e}");
            self.container_process_profile_full(id)
        });
        result
    }
}