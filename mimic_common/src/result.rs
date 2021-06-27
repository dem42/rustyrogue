use std::{ffi::OsString, io};

use crate::propagate;
use thiserror::Error;
//////////////////////// Types ///////////////////////
pub type Result<T> = std::result::Result<T, MimicCommonError>;
//////////////////////// Enums ///////////////////////
#[derive(Error, Debug)]
pub enum MimicCommonError {
    #[error("Failed to get base directory from executable")]
    ExecutableBaseDirError,
    #[error(transparent)]
    IOError(io::Error),
    #[error("Resource {0:?} failed to resolve")]
    ResourceFailedToResolve(OsString),
}
//////////////////////// Impls ///////////////////////
propagate!(MimicCommonError, IOError as io::Error, using_panic_feature);
