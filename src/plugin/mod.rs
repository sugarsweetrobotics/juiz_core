


mod plugin;
mod rust_plugin;
mod cpp_plugin;
mod python_plugin;


pub use rust_plugin::RustPlugin;
pub use python_plugin::PythonPlugin;
pub use cpp_plugin::CppPlugin;
pub use plugin::{Plugin, JuizObjectPlugin, concat_dirname, plugin_name_to_file_name};
pub use python_plugin::{pyany_to_value, pydict_to_value};

use crate::prelude::*;
