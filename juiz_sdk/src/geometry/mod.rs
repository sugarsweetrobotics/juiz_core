//! すでに発表済みの位置管理機能に関する実装途中のモジュール
//! 

pub mod pose;

pub use pose::{
    Pose3D,
    Point3D,
    Transform3D, 
    Vec3, 
    Quaternion, 
    Orientation3D,
    quaternion_from_euler_xyz, euler_xyz_from_quaternion
};