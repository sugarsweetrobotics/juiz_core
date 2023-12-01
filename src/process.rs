
use std::sync::{Mutex, Arc};

use crate::error::JuizError;
use crate::identifier::Identifier;
use crate::value::*;


pub type ProcessFunction=fn(serde_json::Value) -> Result<serde_json::Value, JuizError>;


pub trait Process  {

    fn identifier(&self) -> &Identifier;

    fn call(&self, _args: Value) -> Result<Value, JuizError>;

    fn is_updated(& self) -> Result<bool, JuizError>;

    fn is_updated_exclude(& self, caller_id: &Identifier) -> Result<bool, JuizError>;

    /*
     * invokeは自身の入力側接続をすべてinvokeしてデータを収集した後に、収集したデータで自身をcallして結果を返す。
     * 出力はmemo化されるので、配下がupdateされなければメモを返す。
     * この方法は配下すべてに問い合わせが波及するので、updateされたかどうかをconnectionにpushする仕組みが必要
     * TODO: 
     */
    fn invoke<'b>(&self) -> Result<Value, JuizError>;

    fn invoke_exclude<'b>(&self, arg_name: &String, value: Value) -> Result<Value, JuizError>;

    /*
     * executeは自信をinvokeしてから、自分の出力側接続先をすべてexecuteする。
     * memo化があるので自身を2度実行はしないはず (スレッドやマルチプロセスがあると問題があるので、invoke_excludeを実装すべきだ)
     * TODO:
     * 
     * 自分の配下はinvokeによってinvokeされるが、配下の枝分かれ先はexecuteされない
     */
    fn execute(&self) -> Result<Value, JuizError>;

    fn push_by(&self, arg_name: &String, value: &Value) -> Result<Value, JuizError>;

    fn get_output(&self) -> Option<Value>;

    fn connected_from<'b>(&'b mut self, source: Arc<Mutex<dyn Process>>, connecting_arg: &String, connection_manifest: Value) -> Result<Value, JuizError>;

    fn connection_to(&mut self, target: Arc<Mutex<dyn Process>>, connect_arg_to: &String, connection_manifest: Value) -> Result<Value, JuizError>;
    
    // fn disconnect_to(&mut self, target: Arc<Mutex<dyn Process>>, connect_arg_to: &String) -> ();

}


pub trait Connectable {


    fn is_connected_from(&self) -> bool;

}