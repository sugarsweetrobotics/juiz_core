
pub mod process;
pub mod process_proxy;
pub mod process_impl;
pub mod process_factory;
pub mod process_factory_impl;
pub mod python_process_factory_impl;
pub mod process_factory_wrapper;
pub mod argument;
pub mod output;
pub mod capsule;
pub mod inlet;
pub mod outlet;

pub use process::{Process, proc_lock, proc_lock_mut, process_ptr, process_ptr_clone};
pub use process_factory::{ProcessFactory, ProcessFactoryPtr};
pub use process_factory_impl::create_process_factory;
pub use process_factory_wrapper::ProcessFactoryWrapper;

pub use self::capsule::{capsule_to_value, value_to_capsule, CapsulePtr};
pub use self::argument::Argument;
pub use self::output::Output;

/*
pub struct ArgumentMap {
    map: HashMap<String, Argument>,
}

impl From<Value> for ArgumentMap {
    fn from(value: Value) -> Self {
        todo!("ここにArgumentMapへのValueからの変換を書く")
    }
}

impl ArgumentMap {

    pub(crate) fn get(&self, arg_name: &str) -> Option<Argument> {
        self.get(arg_name)
    }
    
    pub fn new() -> Self {
        ArgumentMap{
            map: HashMap::new()
        }
    }
    
    fn insert(&mut self, key: String, value: Argument) -> () {
        self.map.insert(key, value);
    }
}

pub fn arg_img<'t>(args: &'t Vec<Argument>, name: &str) -> JuizResult<&'t Mat> {
    todo!()
    
    for a in args.iter() {
        if a.name == name {
            //a.image();
            todo!("この関数で画像をかえす");
        }
    }
    Err(anyhow::Error::from(JuizError::ArgumentCanNotFoundByNameError{name: name.to_owned()}))
    
}

*/