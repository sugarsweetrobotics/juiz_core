
use juiz_core::prelude::*;

#[repr(Rust)]
pub struct CvFilesystem {
}

impl CvFilesystem {

    pub fn manifest() -> Value {
        ContainerManifest::new("cv_filesystem").into()
    }
}

fn create_cv_filesystem_container(_manifest: Value) -> JuizResult<Box<CvFilesystem>> {
    Ok(Box::new(CvFilesystem{}))
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_filesystem_factory() -> JuizResult<ContainerFactoryPtr> {
    ContainerFactoryImpl::create(CvFilesystem::manifest(), create_cv_filesystem_container)
}


