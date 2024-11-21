//!
//! Immutable for user
//!
//!
use crate::error::ICustomError;
use crate::error::IUnifiedError;

use crate::kind::Kind;

use strum_macros::EnumDiscriminants;
use strum_macros::FromRepr;

use crate::error::definitions::RustSDK;
use crate::error::definitions::RustSDKCode;
use crate::error::definitions::Solc;
use crate::error::definitions::SolcCode;
use crate::error::definitions::Zksolc;
use crate::error::definitions::ZksolcCode;

#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ZksyncError {
    Compiler(Compiler),
    Tooling(Tooling),
}

impl ZksyncError {
    pub fn get_kind(&self) -> crate::kind::Kind {
        match self {
            ZksyncError::Compiler(Compiler::Zksolc(_)) => {
                Kind::Compiler(CompilerCode::Zksolc)
            }
            ZksyncError::Compiler(Compiler::Solc(_)) => {
                Kind::Compiler(CompilerCode::Solc)
            }
            ZksyncError::Tooling(Tooling::RustSDK(_)) => {
                Kind::Tooling(ToolingCode::RustSDK)
            }
        }
    }
    pub fn get_code(&self) -> i32 {
        match self {
            ZksyncError::Compiler(Compiler::Zksolc(error)) => {
                Into::<ZksolcCode>::into(error) as i32
            }
            ZksyncError::Compiler(Compiler::Solc(error)) => {
                Into::<SolcCode>::into(error) as i32
            }
            ZksyncError::Tooling(Tooling::RustSDK(error)) => {
                Into::<RustSDKCode>::into(error) as i32
            },
        }
    }
}
impl IUnifiedError<ZksyncError> for ZksyncError {}

#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(CompilerCode))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum Compiler {
    Zksolc(Zksolc),
    Solc(Solc),
}

#[repr(i32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ToolingCode))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum Tooling {
    RustSDK(RustSDK),
}

impl ICustomError<ZksyncError, ZksyncError> for Zksolc {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::Compiler(Compiler::Zksolc(self.clone()))
    }
}
impl ICustomError<ZksyncError, ZksyncError> for Solc {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::Compiler(Compiler::Solc(self.clone()))
    }
}
impl ICustomError<ZksyncError, ZksyncError> for RustSDK {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::Tooling(Tooling::RustSDK(self.clone()))
    }
}

impl std::fmt::Display for ZksyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
impl std::error::Error for ZksyncError {}
