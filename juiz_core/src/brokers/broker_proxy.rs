

use uuid::Uuid;

use crate::prelude::*;

use std::path::PathBuf;



pub trait SystemBrokerProxy {
    /// Systemのプロファイルを取得する
    /// 
    /// System登録のプロセスやコンテナの情報を全て列挙したデータを取得する
    /// 
    fn system_profile_full(&self) -> JuizResult<Value>;

    /// 
    /// 
    /// 
    fn system_filesystem_list(&self, path_buf: PathBuf) -> JuizResult<Value>;

    /// サブシステムを追加登録する
    /// 
    fn system_add_subsystem(&mut self, profile: Value) -> JuizResult<Value>;

    /// マスターシステムを追加登録する
    /// 
    /// 
    fn system_add_mastersystem(&mut self, profile: Value) -> JuizResult<Value>;

    /// SystemのUUIDを取得する
    /// 
    /// 
    fn system_uuid(&self) -> JuizResult<Value>;

    /// Processのfactoryをファイルシステムからロードする
    /// 
    /// 
    fn system_load_process(&mut self, language: String, filepath: String) -> JuizResult<Value>;

    /// Containerのfactoryをファイルシステムからロードする
    /// 
    /// 
    fn system_load_container(&mut self, language: String, filepath: String) -> JuizResult<Value>;


    /// ContainerProcessのfactoryをファイルシステムからロードする
    /// 
    /// 
    fn system_load_container_process(&mut self, language: String, filepath: String) -> JuizResult<Value>;

    /// Componentのfactoryをファイルシステムからロードする
    /// 
    /// 
    fn system_load_component(&mut self, language: String, filepath: String) -> JuizResult<Value>;

}

pub trait ProcessBrokerProxy {

    /// プロセス作成。
    /// 引数はマニフェスト
    /// type_name, nameが最低限の引数。
    /// use_memoはオプション
    fn process_create(&mut self, manifest: ProcessManifest) -> JuizResult<Value>;

    fn process_destroy(&mut self, identifier: &Identifier) -> JuizResult<Value>;

    /// プロセスリスト取得
    ///
    /// Broker支配下のプロセスのIDのリストを取得する
    /// 
    /// * `recursive` - サブシステムのプロセスを再起的に読み込む場合はtrue
    fn process_list(&self, recursive: bool) -> JuizResult<Value>;

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    /// プロセスをCallする
    /// 
    /// * id: プロセスのID
    /// * args: 引数
    fn process_call(&self, id: &Identifier, _args: CapsuleMap) -> JuizResult<CapsulePtr>;


    /// プロセスをExecuteする
    /// 
    /// * id: プロセスのID
    fn process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr>;

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value>;

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value>;

    fn process_p_apply(&mut self, id: &Identifier, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr>;
}

pub trait ContainerBrokerProxy {

    fn container_create(&mut self, manifest: CapsuleMap) -> JuizResult<Value>;

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

    fn container_process_create(&mut self, container_id: &Identifier, manifest: ProcessManifest) -> JuizResult<Value>;

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



pub trait TopicBrokerProxy {
    fn topic_list(&self) -> JuizResult<Value>;

    fn topic_push(&self, name: &str, capsule: CapsulePtr, pushed_system: Option<Uuid>) -> JuizResult<()>;

    /// Topic をSubscribeする必要があるか問い合わせる
    /// {"subscribe": true/false } が返る
    fn topic_request_subscribe(&mut self, name: &str, system_uuid: Option<Uuid>) -> JuizResult<Value>;

    /// Topic をPublishする必要があるか問い合わせる
    /// {"subscribe": true/false } が返る
    fn topic_request_publish(&mut self, name: &str, system_uuid: Option<Uuid>) -> JuizResult<Value>;

}

pub trait BrokerProxy : Send + JuizObject + SystemBrokerProxy + ProcessBrokerProxy + ContainerBrokerProxy + ContainerProcessBrokerProxy + ExecutionContextBrokerProxy + BrokerBrokerProxy + ConnectionBrokerProxy + TopicBrokerProxy {

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