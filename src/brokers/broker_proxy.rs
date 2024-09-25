

use crate::prelude::*;

use std::path::PathBuf;

use crate::{identifier::IdentifierStruct, value::{Capsule, CapsuleMap}, value::value_merge};




pub trait SystemBrokerProxy {
    fn system_profile_full(&self) -> JuizResult<Value>;

    fn system_filesystem_list(&self, path_buf: PathBuf) -> JuizResult<Value>;

    fn system_add_subsystem(&mut self, profile: Value) -> JuizResult<Value>;

    fn system_uuid(&self) -> JuizResult<Value>;

}

pub trait ProcessBrokerProxy {

    /// プロセス作成。
    /// 引数はマニフェスト
    /// type_name, nameが最低限の引数。
    /// use_memoはオプション
    fn process_create(&mut self, manifest: &Value) -> JuizResult<Value>;

    fn process_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value>;

    /// プロセスリスト取得
    ///
    /// Broker支配下のプロセスのIDのリストを取得する
    /// 
    /// * `recursive` - サブシステムのプロセスを再起的に読み込む場合はtrue
    fn process_list(&self, recursive: bool) -> JuizResult<Value>;

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn process_call(&self, id: &Identifier, _args: CapsuleMap) -> JuizResult<CapsulePtr>;

    fn process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr>;

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value>;

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value>;

    fn process_bind(&mut self, id: &Identifier, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr>;
}

pub trait ContainerBrokerProxy {

    fn container_create(&mut self, manifest: &Value) -> JuizResult<Value>;

    fn container_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value>;


    /// コンテナリスト取得
    ///
    /// Broker支配下のプロセスのIDのリストを取得する
    /// 
    /// * `recursive` - サブシステムのプロセスを再起的に読み込む場合はtrue
    fn container_list(&self, recursive: bool) -> JuizResult<Value>;

    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Value>;
}

pub trait ContainerProcessBrokerProxy {

    fn container_process_create(&mut self, container_id: &Identifier, manifest: &Value) -> JuizResult<Value>;

    fn container_process_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value>;


    /// コンテナプロセスのリスト取得
    ///
    /// Broker支配下のプロセスのIDのリストを取得する
    /// 
    /// * `recursive` - サブシステムのプロセスを再起的に読み込む場合はtrue
    fn container_process_list(&self, rucursive: bool) -> JuizResult<Value>;

    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn container_process_call(&self, id: &Identifier, _args: CapsuleMap) -> JuizResult<CapsulePtr>;

    fn container_process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr>;
}

pub trait ExecutionContextBrokerProxy {

    fn ec_create(&mut self, manifest: &Value) -> JuizResult<Value>;

    fn ec_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value>;
    
    /// 実行コンテキストのリスト取得
    ///
    /// Broker支配下のプロセスのIDのリストを取得する
    /// 
    /// * `recursive` - サブシステムのプロセスを再起的に読み込む場合はtrue
    fn ec_list(&self, recursive: bool) -> JuizResult<Value>;

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Value>;

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Value>;

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Value>;
}
pub trait BrokerBrokerProxy {
    
    /// ブローカのリスト取得
    ///
    /// Broker支配下のプロセスのIDのリストを取得する
    /// 
    /// * `recursive` - サブシステムのプロセスを再起的に読み込む場合はtrue
    fn broker_list(&self, recursive: bool) -> JuizResult<Value>;

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value>;
}

pub trait ConnectionBrokerProxy {

    /// Connectionのリスト取得
    ///
    /// Broker支配下のプロセスのIDのリストを取得する
    /// 
    /// * `recursive` - サブシステムのプロセスを再起的に読み込む場合はtrue
    fn connection_list(&self, recursive: bool) -> JuizResult<Value>;

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Value>;

    fn connection_destroy(&mut self, id: &Identifier) -> JuizResult<Value>;

}

pub trait BrokerProxy : Send + JuizObject + SystemBrokerProxy + ProcessBrokerProxy + ContainerBrokerProxy + ContainerProcessBrokerProxy + ExecutionContextBrokerProxy + BrokerBrokerProxy + ConnectionBrokerProxy {

    fn is_in_charge_for_process(&self, _id: &Identifier) -> JuizResult<bool>;

    fn any_process_list(&self, recursive: bool) -> JuizResult<Capsule> {
        let processes = self.process_list(recursive)?;
        let container_processes = self.container_process_list(recursive)?;
        Ok(value_merge(processes, &container_processes)?.into())
    }

    fn any_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        log::info!("BrokerProxy::any_process_profile_full({id}) called");
        let id_struct = IdentifierStruct::try_from(id.clone())?;
        log::info!("id_struct{:?}", id_struct);        
        if id_struct.class_name == "Process" {
            return self.process_profile_full(id)
        }
        self.container_process_profile_full(id)
    }

    fn any_process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        log::info!("BrokerProxy::any_process_profile_call({id}) called");
        let id_struct = IdentifierStruct::try_from(id.clone())?;
        if id_struct.class_name == "Process" {
            return self.process_call(id, args)
        }
        self.container_process_call(id, args)
    }
}