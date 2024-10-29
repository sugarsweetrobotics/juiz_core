
use juiz_base::prelude::*;

#[repr(Rust)]
pub struct CvFilesystem {
}

impl CvFilesystem {

    pub fn manifest() -> ContainerManifest {
        ContainerManifest::new("cv_filesystem")
            .factory("cv_filesystem_factory")
    }
}

fn create_cv_filesystem_container(_manifest: ContainerManifest) -> JuizResult<Box<CvFilesystem>> {
    Ok(Box::new(CvFilesystem{}))
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_filesystem_factory() -> JuizResult<ContainerFactoryStruct> {
    Ok(juiz_base::container_factory(CvFilesystem::manifest(), create_cv_filesystem_container))
}


