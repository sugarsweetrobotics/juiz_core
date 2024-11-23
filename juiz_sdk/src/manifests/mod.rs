//! プロセスやコンテナのバイナリの機能記述 (マニフェスト) に関する機能パッケージ
//! 

mod process_manifest;
mod container_manifest;
mod argument_manifest;
mod manifest_description;
// pub mod container_process_manifest;
mod component_manifest;
mod topic_manifest;

pub use container_manifest::ContainerManifest;
pub use process_manifest::ProcessManifest;
pub use component_manifest::ComponentManifest;
pub use topic_manifest::TopicManifest;
pub use argument_manifest::{ArgumentManifest, ArgumentType};
pub use manifest_description::Description;