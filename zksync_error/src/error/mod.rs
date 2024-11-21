pub mod definitions;
pub mod domains;

use std::error::Error;

use crate::error::domains::Compiler;
use crate::error::domains::Tooling;
use crate::error::domains::ZksyncError;
use crate::identifier::Identifier;

pub trait IError<ContainedType>: Error
where
    ContainedType: Clone,
{
    fn get_identifier(&self) -> Identifier;
    fn get_message(&self) -> String;
    fn get_data(&self) -> ContainedType;
}

pub trait IUnifiedError<ContainedType>:
    serde::Serialize + for<'de> serde::Deserialize<'de> + IError<ContainedType>
where
    ContainedType: Clone,
{
}

pub trait ICustomError<U, C>
where
    U: IUnifiedError<C>,
    C: Clone,
{
    fn to_unified(&self) -> U;
}

pub trait CustomErrorMessage {
    fn get_message(&self) -> String;
}

impl IError<ZksyncError> for ZksyncError {
    fn get_identifier(&self) -> Identifier {
        Identifier {
            kind: self.get_kind(),
            code: self.get_code(),
        }
    }

    fn get_message(&self) -> String {
        match self {
            ZksyncError::Compiler(Compiler::Zksolc(error)) => error.get_message(),
            ZksyncError::Compiler(Compiler::Solc(error)) => error.get_message(),
            ZksyncError::Tooling(Tooling::RustSDK(error)) => error.get_message(),
        }
    }

    fn get_data(&self) -> ZksyncError {
        self.clone()
    }
}
