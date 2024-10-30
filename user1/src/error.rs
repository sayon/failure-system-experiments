use common::error::CustomErrorMessage;

use common::error::IError;

use common::identifier::Identifier;

use crate::domains::CompilerError;
use crate::domains::ToolingError;
use crate::domains::ZksyncError;

impl IError<ZksyncError> for ZksyncError {
    fn get_identifier(&self) -> Identifier {
        Identifier {
            kind: self.get_kind(),
            code: self.get_code(),
        }
    }

    fn get_message(&self) -> String {
        match self {
            ZksyncError::CompilerError(compiler_error) => match compiler_error {
                CompilerError::Zksolc(zksolc_error) => zksolc_error.get_message(),
                CompilerError::Solc(solc_error) => solc_error.get_message(),
            },
            ZksyncError::ToolingError(tooling_error) => match tooling_error {
                ToolingError::RustSDK(rust_sdkerror) => rust_sdkerror.get_message(),
            },
        }
    }

    fn get_data(&self) -> ZksyncError {
        self.clone()
    }
}
