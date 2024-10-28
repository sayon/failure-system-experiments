use strum_macros::{EnumDiscriminants, FromRepr};

#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq)]
#[strum_discriminants(name(Domain))]
#[strum_discriminants(derive(FromRepr))]
#[strum_discriminants(vis(pub))]
#[repr(i32)]
pub enum Kind {
    Era(EraSubdomain) = 1,
    Compiler(CompilerSubdomain) = 2,
    Tooling(ToolingSubdomain) = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, FromRepr)]
#[repr(i32)]
pub enum CompilerSubdomain {
    Solc = 1,
    Zksolc = 2,
    Vyper = 3,
    LLVM = 4,
}

#[derive(Clone, Debug, Eq, PartialEq, FromRepr)]
#[repr(i32)]
pub enum ToolingSubdomain {
    RustSDK = 4,
}

#[derive(Clone, Debug, Eq, PartialEq, FromRepr)]
#[repr(i32)]
pub enum EraSubdomain {
    VM = 1,
    Sequencer = 2,
}

impl Kind {
    pub fn domain_code(&self) -> i32 {
        let domain: Domain = self.clone().into();
        domain as i32
    }
    pub fn subdomain_code(&self) -> i32 {
        match self {
            Kind::Era(subdomain) => subdomain.clone() as i32,
            Kind::Compiler(subdomain) => subdomain.clone() as i32,
            Kind::Tooling(subdomain) => subdomain.clone() as i32,
        }
    }
}
