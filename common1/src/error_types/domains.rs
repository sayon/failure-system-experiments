// use strum_macros::{EnumDiscriminants, FromRepr};

// #[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq)]
// #[strum_discriminants(name(Domain))]
// #[strum_discriminants(derive(FromRepr))]
// #[strum_discriminants(vis(pub))]
// #[repr(u32)]
// pub enum DomainPath {
//     Era(EraSubdomain) = 1,
//     Compiler(CompilerSubdomain) = 2,
// }

// #[derive(Clone, Debug, Eq, PartialEq, FromRepr)]
// #[repr(u32)]
// pub enum CompilerSubdomain {
//     Solc = 1,
//     Zksolc = 2,
//     Vyper = 3,
//     LLVM = 4,
// }

// #[derive(Clone, Debug, Eq, PartialEq, FromRepr)]
// #[repr(u32)]
// pub enum EraSubdomain {
//     VM = 1,
//     Sequencer = 2,
// }

// impl DomainPath {
//     pub fn domain_code(&self) -> u32 {
//         let domain: Domain = self.clone().into();
//         domain as u32
//     }
//     pub fn subdomain_code(&self) -> u32 {
//         match self {
//             DomainPath::Era(subdomain) => subdomain.clone() as u32,
//             DomainPath::Compiler(subdomain) => subdomain.clone() as u32,
//         }
//     }
// }
