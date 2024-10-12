
/// process.rs
/// 
/// 
/// 
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::prelude::*;
use crate::connections::{DestinationConnection, SourceConnection};


pub type ProcessPtr = Arc<RwLock<dyn Process>>;


pub type FunctionType = fn(CapsuleMap) -> JuizResult<Capsule>;
pub type FunctionTrait = dyn Fn(CapsuleMap) -> JuizResult<Capsule>;
///
/// 
pub trait Process : Send + Sync + JuizObject {

    fn call(&self, _args: CapsuleMap) -> JuizResult<CapsulePtr>;

    fn is_updated(& self) -> JuizResult<bool>;

    //fn is_updated_exclude(& self, inlet_name: &str) -> JuizResult<bool>;

    fn manifest(&self) -> &Value;
    
    // fn profile_full(&self) -> JuizResult<Value>;
    /*
     * invokeは自身の入力側接続をすべてinvokeしてデータを収集した後に、収集したデータで自身をcallして結果を返す。
     * 出力はmemo化されるので、配下がupdateされなければメモを返す。
     * この方法は配下すべてに問い合わせが波及するので、updateされたかどうかをconnectionにpushする仕組みが必要
     * TODO: 
     */
    fn invoke<'b>(&self) -> JuizResult<CapsulePtr>;

    //fn invoke_exclude<'b>(&self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr>;

    /*
     * executeは自信をinvokeしてから、自分の出力側接続先をすべてexecuteする。
     * memo化があるので自身を2度実行はしないはず (スレッドやマルチプロセスがあると問題があるので、invoke_excludeを実装すべきだ)
     * TODO:
     * 
     * 自分の配下はinvokeによってinvokeされるが、配下の枝分かれ先はexecuteされない
     */
    fn execute(&self) -> JuizResult<CapsulePtr>;

    fn push_by(&self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr>;

    fn get_output(&self) -> CapsulePtr;

    fn notify_connected_from<'b>(&'b mut self, source: ProcessPtr, connecting_arg: &str, connection_manifest: Value) -> JuizResult<Value>;

    fn try_connect_to(&mut self, target: ProcessPtr, connect_arg_to: &str, connection_manifest: Value) -> JuizResult<Value>;
    
    fn source_connections(&self) -> JuizResult<Vec<&Box<dyn SourceConnection>>>;

    fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn DestinationConnection>>>;

    fn bind(&mut self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr>;

    fn purge(&mut self) -> JuizResult<()>;
}



pub fn process_ptr<T>(proc: T) -> ProcessPtr where T: Process + 'static {
    Arc::new(RwLock::new(proc))
}

// pub fn process_ptr_clone(ptr: &ProcessPtr) -> ProcessPtr {
//     Arc::clone(ptr)
// }

pub fn proc_lock<'a>(obj: &'a ProcessPtr) -> JuizResult<RwLockReadGuard<'a, dyn Process>> {
    match obj.read() {
        Err(e) => {
            log::error!("juiz_lock() failed. Error is {:?}", e);
            Err(anyhow::Error::from(JuizError::MutexLockFailedError{error: e.to_string()}))
        },
        Ok(v) => Ok(v)
    }
}

pub fn proc_lock_mut<'b, T: Process + ?Sized>(obj: &'b Arc<RwLock<T>>) -> JuizResult<RwLockWriteGuard<'b, T>>{
    match obj.write() {
        Err(e) => {
            log::error!("juiz_lock() failed. Error is {:?}", e);
            Err(anyhow::Error::from(JuizError::MutexLockFailedError{error: e.to_string()}))
        },
        Ok(v) => Ok(v)
    }
}
