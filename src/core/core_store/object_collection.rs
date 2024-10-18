
use std::collections::HashMap;



use crate::prelude::*;





pub struct ObjectCollection<T, TF> {
    name: String,
    factories: HashMap<String, TF>,
    objects: HashMap<Identifier, T>,
    //objects: Rc<RefCell<HashMap<Identifier, Arc<Mutex<T>>>>>,
}

impl<T, TF> ObjectCollection<T, TF> {

    pub fn new(name: &str) -> Box<ObjectCollection<T, TF>> {
        Box::new(ObjectCollection { name: name.to_string(), 
            factories: HashMap::new(), 
            objects: HashMap::new() })
    }

    pub fn clear(&mut self) -> JuizResult<()> {
        self.objects.clear();
        self.factories.clear();
        Ok(())
    }

    pub fn objects(&self) -> &HashMap<String, T> {
        &self.objects
    }

    pub fn factories(&self) -> &HashMap<String, TF> {
        &self.factories
    }

    pub fn register_factory(&mut self, type_name: &str, pf: TF) -> JuizResult<()> {
        log::trace!("StoreWorker({})::registerfactory(Factory(type_name={:?})) called",self.name,type_name);
        if self.factories.contains_key(type_name) {
            return Err(anyhow::Error::from(JuizError::FactoryOfSameTypeNameAlreadyExistsError{type_name: type_name.to_owned()}));
        }
        let _opt_ref = self.factories.insert(type_name.to_owned(), pf);
        Ok(())
    }

    pub fn factory(&self, type_name: &str) -> JuizResult<&TF> {
        match self.factories.get(type_name) {
            None => return Err(anyhow::Error::from(JuizError::FactoryCanNotFoundError{type_name: type_name.to_string()})),
            Some(pf) => return Ok(pf)
        }
    }

    pub fn register(&mut self, id: &Identifier, p: T) -> JuizResult<&T> {
        log::trace!("StoreWorker({})::register(Object(id={:?})) called", self.name, id);
        self.objects.insert(id.clone(), p);
        self.get(&id)
    }

    // pub fn deregister(&mut self, p: Arc<Mutex<T>>) -> JuizResult<Arc<Mutex<T>>> {
    //     let id = p.lock().unwrap().identifier().clone();
    //     self.deregister_by_id(&id)
    // }

    pub fn deregister_by_id(&mut self, id: &Identifier) -> JuizResult<T> {
        log::trace!("StoreWorker({})::deregister(Object(id={:?})) called", self.name, id);
        match self.objects.remove(id) {
            Some(p) => Ok(p),
            None =>{
                log::trace!("StoreWorker({})::deregister(id={:}) failed. Not found.", self.name, id);
                Err(anyhow::Error::from(JuizError::ObjectCanNotFoundByIdError { id: id.clone() }))
            }
        }
    }

    pub fn get<'a>(&'a self, id: &Identifier) -> JuizResult<&'a T> {
        match self.objects.get(id) {
            Some(p) => Ok(p),
            None => {
                log::trace!("StoreWorker({})::get(id={:?}) failed.", self.name, id);
                log::trace!(" - CoreStore includes processes[");
                for (k, _v) in self.objects.iter() {
                    log::trace!("    - {:?}", k);
                }
                log::trace!("]");
                Err(anyhow::Error::from(JuizError::ObjectCanNotFoundByIdError{id: id.clone()}))
            }
        }
    }

    pub fn cleanup_objects(&mut self) -> JuizResult<()> {
        self.objects.clear();
        Ok(())
    }
}
