
//!
//! AUTOGENERATED BASED ON A SET OF JSON FILES, DO NOT EDIT MANUALLY
//!

use strum_macros::{EnumDiscriminants, FromRepr};

use crate::error::domains::CompilerCode;
use crate::error::domains::CoreCode;

#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[strum_discriminants(name(DomainCode))]
#[strum_discriminants(derive(FromRepr))]
#[strum_discriminants(vis(pub))]
#[repr(i32)]
pub enum Kind {
   Compiler(CompilerCode) = 1,
   Core(CoreCode) = 2,
}

impl Kind {
    pub fn domain_code(&self) -> i32 {
        let domain: DomainCode = self.clone().into();
        domain as i32
    }
    pub fn component_code(&self) -> i32 {
        match self {
         Kind::Compiler(component) => component.clone() as i32,
         Kind::Core(component) => component.clone() as i32,

        }
    }
}
