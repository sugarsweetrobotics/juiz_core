//! juizで使うデータ型に関する機能パッケージ
//! 
//! 
//! juiz内部ではデータはCapsuleというデータ型で表現されます。これはserde_json::ValueとImageの直積です。


pub mod value;
pub mod value_converter;

pub mod converter_error;

pub mod capsule;
pub mod capsule_converter;

pub mod capsule_map;
pub mod capsule_map_converter;

pub mod capsule_ptr;
pub mod capsule_ptr_converter;


pub use value::*;
pub use capsule::*;
pub use capsule_ptr::*;
pub use capsule_map::*;