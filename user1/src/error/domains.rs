//!
//! Immutable for user
//!
//!
use zksync_error::error::ICustomError;

use zksync_error::error::IUnifiedError;

use zksync_error::kind::CompilerSubdomain;

use zksync_error::kind::Kind;
use zksync_error::kind::ToolingSubdomain;

use strum_macros::EnumDiscriminants;

use crate::RustSDKError;
use crate::RustSDKErrorCode;
use crate::SolcError;
use crate::SolcErrorCode;
use crate::ZksolcError;
use crate::ZksolcErrorCode;

#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ZksyncError {
    CompilerError(CompilerError),
    ToolingError(ToolingError),
}

impl ZksyncError {
    pub fn get_kind(&self) -> zksync_error::kind::Kind {
        match self {
            ZksyncError::CompilerError(compiler_error) => Kind::Compiler(match compiler_error {
                CompilerError::Zksolc(_) => CompilerSubdomain::Zksolc,
                CompilerError::Solc(_) => CompilerSubdomain::Solc,
            }),
            ZksyncError::ToolingError(tooling_error) => Kind::Tooling(match tooling_error {
                ToolingError::RustSDK(_) => ToolingSubdomain::RustSDK,
            }),
        }
    }
    pub fn get_code(&self) -> i32 {
        match self {
            ZksyncError::CompilerError(compiler_error) => match compiler_error {
                CompilerError::Zksolc(zksolc_error) => {
                    Into::<ZksolcErrorCode>::into(zksolc_error) as i32
                }
                CompilerError::Solc(solc_error) => Into::<SolcErrorCode>::into(solc_error) as i32,
            },
            ZksyncError::ToolingError(tooling_error) => match tooling_error {
                ToolingError::RustSDK(rust_sdkerror) => {
                    Into::<RustSDKErrorCode>::into(rust_sdkerror) as i32
                }
            },
        }
    }
}
impl IUnifiedError<ZksyncError> for ZksyncError {}

#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(CompilerSubdomainCode))]
#[strum_discriminants(vis(pub))]
pub enum CompilerError {
    Zksolc(ZksolcError),
    Solc(SolcError),
}

#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ToolingSubdomainCode))]
#[strum_discriminants(vis(pub))]
pub enum ToolingError {
    RustSDK(RustSDKError),
}

impl ICustomError<ZksyncError, ZksyncError> for ZksolcError {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::CompilerError(CompilerError::Zksolc(self.clone()))
    }
}
impl ICustomError<ZksyncError, ZksyncError> for SolcError {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::CompilerError(CompilerError::Solc(self.clone()))
    }
}
impl ICustomError<ZksyncError, ZksyncError> for RustSDKError {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::ToolingError(ToolingError::RustSDK(self.clone()))
    }
}

impl std::fmt::Display for ZksyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
impl std::error::Error for ZksyncError {}
