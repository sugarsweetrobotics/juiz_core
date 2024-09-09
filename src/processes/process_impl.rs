





use anyhow::Context;
use serde_json::Map;

use crate::identifier::{identifier_from_manifest, create_identifier_from_manifest};
use crate::object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass};

use crate::processes::proc_lock;
use crate::value::{obj_get_bool, obj_get_obj, obj_get_str, obj_merge_mut};

use crate::prelude::*;

use crate::utils::{check_manifest_before_call, check_process_manifest};
use crate::connections::{SourceConnection, SourceConnectionImpl, DestinationConnection, DestinationConnectionImpl};

use crate::value::{Capsule, CapsuleMap};
use super::inlet::Inlet;
use super::outlet::Outlet;

pub type FunctionType = fn(CapsuleMap) -> JuizResult<Capsule>;
pub type FunctionTrait = dyn Fn(CapsuleMap) -> JuizResult<Capsule>;

pub struct ProcessImpl {
    core: ObjectCore,
    manifest: Value,
    function: Box<FunctionTrait>,
    identifier: Identifier,
    outlet: Outlet,
    inlets: Vec<Inlet>,
}


pub fn argument_manifest(process_manifest: &Value) -> JuizResult<&Map<String, Value>>{
    obj_get_obj(process_manifest, "arguments")
}

impl ProcessImpl {

    pub fn new_with_class(class_name: JuizObjectClass, manif: Value, func: FunctionType) -> JuizResult<Self> {
        log::trace!("ProcessImpl::new(manifest={}) called", manif);
        let manifest = check_process_manifest(manif)?;
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        let use_memo = match obj_get_bool(&manifest, "use_memo") {
            Err(_) => true,
            Ok(v) => v
        };
        Ok(Self{
            core: ObjectCore::create(class_name, 
                type_name,
                object_name,
            ),
            manifest: manifest.clone(), 
            function: Box::new(func), 
            identifier: create_identifier_from_manifest("Process", &manifest)?,
            outlet: Outlet::new(object_name, use_memo),
            inlets: Self::create_inlets(&manifest)?,
        })
    }

    fn create_inlets(manifest: &Value) -> JuizResult<Vec<Inlet>> {
        Ok(argument_manifest(&manifest)?.iter().map( |(k, v)| {
            Inlet::new(k.as_str(), v.get("default").unwrap().clone())
        }).collect::<Vec<Inlet>>())
    }

    pub fn inlet(&self, name: &str) -> JuizResult<&Inlet> {
        self.inlets.iter().find(|inlet| { (*inlet).name() == name }).ok_or_else(|| { anyhow::Error::from(JuizError::CanNotFindError { target: format!("Process::Inlet({name})") }) } )
    }
    
    pub fn inlet_mut(&mut self, name: &str) -> JuizResult<&mut Inlet> {
        self.inlets.iter_mut().find(|inlet| { (*inlet).name() == name }).ok_or_else(|| { anyhow::Error::from(JuizError::CanNotFindError { target: format!("Process::Inlet({name})") }) } )
    }

    pub fn new(manif: Value, func: FunctionType) -> JuizResult<Self> {
        Self::new_with_class(JuizObjectClass::Process("ProcessImpl"), manif, func)
    }

    pub(crate) fn clousure_new_with_class_name(class_name: JuizObjectClass, manif: Value, func: Box<FunctionTrait>) -> JuizResult<Self> {
        log::trace!("ProcessImpl::new(manifest={}) called", manif);
        let manifest = check_process_manifest(manif)?;
        let object_name = obj_get_str(&manifest, "name")?;
        let use_memo = obj_get_bool(&manifest, "use_memo").or::<bool>(Ok(true)).unwrap();
        Ok(Self{
            core: ObjectCore::create(class_name, obj_get_str(&manifest, "type_name")?, object_name),
            function: func, 
            identifier: identifier_from_manifest("core", "core", "Process", &manifest)?,
            outlet: Outlet::new(object_name, use_memo),
            inlets: Self::create_inlets(&manifest)?,
            manifest: manifest, 
        })
    }

    pub(crate) fn _clousure_new(manif: Value, func: Box<FunctionTrait>) -> JuizResult<Self> {
        ProcessImpl::clousure_new_with_class_name(JuizObjectClass::Process("ProcessImpl"), manif, func)
    }

        
    fn collect_values(&self) -> CapsuleMap {
        log::trace!("ProcessImpl({}).collect_values() called.", &self.identifier);
        self.inlets.iter().map(|inlet| { (inlet.name().clone(), inlet.collect_value() )} ).collect::<Vec<(String, CapsulePtr)>>().into()
    }

    /// invokeが起こった時にinletから入力データを収集する関数。
    fn collect_values_exclude(&self, arg_name: &str, arg_value: CapsulePtr) -> CapsuleMap {
        log::trace!("ProcessImpl({}).collect_values_exclude(arg_name={}) called.", &self.identifier, arg_name);
        // excludeされるべきarg_nameでないinletにはcollect_valueをそれぞれ呼び出す。
        self.inlets.iter().map(|inlet| {
            if inlet.name() == arg_name { return (arg_name.to_owned(), arg_value.clone()); }
            return (inlet.name().clone(), inlet.collect_value());
        }).collect::<Vec<(String, CapsulePtr)>>().into()
    }

}

impl JuizObjectCoreHolder for ProcessImpl {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl JuizObject for ProcessImpl {


    fn profile_full(&self) -> JuizResult<Value> {
        let mut v = self.core.profile_full()?;
        obj_merge_mut(&mut v, &jvalue!({
            "inlets": self.inlets.iter().map(|inlet| { inlet.profile_full().unwrap() }).collect::<Vec<Value>>(),
            "outlet": self.outlet.profile_full()?,
            "arguments": self.manifest.get("arguments").unwrap(),
        }))?;
        Ok(v.into())
    }

}

impl Process for ProcessImpl {
    
    fn manifest(&self) -> &Value { 
        &self.manifest
    }

    fn call(&self, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        log::trace!("ProcessImpl({})::call(args={:?}) called", self.identifier(), args);
        check_manifest_before_call(&(self.manifest), &args)?;
        Ok( (self.function)(args)?.into() )
    }

    fn is_updated(&self) -> JuizResult<bool> {
        self.is_updated_exclude("")
    }

    fn is_updated_exclude(&self, arg_name: &str) -> JuizResult<bool> {
        log::trace!("ProcessImpl({})::is_updated_exclude(arg_name='{}') called now", self.identifier(), arg_name);
        if self.outlet.memo().is_empty()? {
            log::trace!(" - MEMO is empty. Must be updated.");
            return Ok(true)
        }
        for inlet in self.inlets.iter() {
            if inlet.name() == arg_name { continue; }
            if inlet.is_updated()? {
                log::trace!(" - Inlet({}) is updated. Must be updated.", inlet.name());
                return Ok(true)
            }
        }
        log::trace!("ProcessImpl({})::is_updated_exclude(arg_name='{}') returned false", self.identifier(), arg_name);
        Ok(false)
    }

    /// invokeする
    /// 
    /// inletから入力を受け取ってcallをして、出力を得る。無事に出力が得られたらmemoに書き込む。
    fn invoke<'b>(&'b self) -> JuizResult<CapsulePtr> {
        log::trace!("Processimpl({})::invoke() called", self.identifier());
        if self.outlet.memo().is_empty()? || self.is_updated()? {
            return Ok(self.outlet.set_value(self.call(self.collect_values())?));
        }
        return Ok(self.outlet.memo().clone());
    }

    /// invokeをするが、inletのうちarg_nameで指定されるものに関してはデータ収集を行わずに引数valueとして受け取った値を入力として使う
    /// 
    /// これは後段のsource_connectionからpushされた場合に使う。
    /// 
    fn invoke_exclude<'b>(&self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        log::trace!("Processimpl({})::invoke_exclude(arg_name={}) called", self.identifier(), arg_name);
        if self.outlet.memo().is_empty()? {
            log::trace!("Processimpl({})::invoke_exclude() called. memo is empty.", self.identifier());
            return Ok(self.outlet.set_value(self.call(self.collect_values_exclude(arg_name, value))?));
        } else if self.is_updated_exclude(arg_name)? {
            log::trace!("Processimpl({})::invoke_exclude() called. inlet is updated.", self.identifier());
            return Ok(self.outlet.set_value(self.call(self.collect_values_exclude(arg_name, value))?));
        } else {
            log::trace!("Processimpl({})::invoke_exclude() called but memo is not empty and inlet is not updated.", self.identifier());
        }
        return Ok(self.outlet.memo().clone());
    }
    fn execute(&self) -> JuizResult<CapsulePtr> {
        log::trace!("Processimpl({})::execute() called", self.identifier());
        self.outlet.push(self.invoke()?)
    }

    fn push_by(&self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.outlet.push(self.invoke_exclude(arg_name, value.clone())?)
    }
    
    fn get_output(&self) -> CapsulePtr {
        self.outlet.memo().clone()
    }

    fn notify_connected_from(&mut self, source: ProcessPtr, connecting_arg: &str, connection_manifest: Value) -> JuizResult<Value> {
        log::trace!("ProcessImpl(id={:?}).notify_connected_from(source=Process()) called", self.identifier());
        let id = self.identifier().clone();
        self.inlet_mut(connecting_arg)?.insert(
                    Box::new(SourceConnectionImpl::new(id, source, connection_manifest.clone(), connecting_arg.to_owned())?));
        log::trace!("ProcessImpl(id={:?}).notify_connected_from(source=Process()) exit", self.identifier());
        Ok(connection_manifest.into())
    }

    fn try_connect_to(&mut self, destination: ProcessPtr, arg_name: &str, connection_manifest: Value) -> JuizResult<Value> {
        log::trace!("ProcessImpl(id={:?}).try_connect_to(destination=Process()) called", self.identifier());
        let destination_id = proc_lock(&destination).context("ProcessImpl::try_connect_to()")?.identifier().clone();
        self.outlet.insert(
            arg_name.to_owned(), 
            Box::new(DestinationConnectionImpl::new(
                &self.identifier(), 
                &destination_id,
                destination, 
                connection_manifest.clone(), 
                arg_name.to_owned())?));
        log::trace!("ProcessImpl(id={:?}).try_connect_to(destination=Process()) exit", self.identifier());
        Ok(connection_manifest.into())
    }

    
    fn source_connections(&self) -> JuizResult<Vec<&Box<dyn SourceConnection>>> {
        Ok(self.inlets.iter().map(|inlet| { inlet.source_connections() } ).flatten().collect::<Vec<&Box<dyn SourceConnection>>>())
    }
    

    fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn DestinationConnection>>> {
        self.outlet.destination_connections()
    }
    
    fn bind(&mut self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.inlet_mut(arg_name)?.bind(value)
    }
    
    fn purge(&mut self) -> JuizResult<()> {
        log::trace!("ProcessImpl({})::purge() called", self.identifier());
        Ok(())
    }
}

impl Drop for ProcessImpl {
    fn drop(&mut self) {
        log::info!("ProcessImpl({})::drop() called", self.identifier());
        log::trace!("ProcessImpl({})::drop() exit", self.identifier());
    }
}

unsafe impl Send for ProcessImpl {

}

unsafe impl Sync for ProcessImpl {

}