// use super::domains::{CompilerSubdomain, Domain, DomainPath, EraSubdomain};

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct ErrorIdentifier {
//     pub path: DomainPath,
//     pub code: u32,
// }

// impl ErrorIdentifier {
//     pub fn encode(&self) -> u32 {
//         let domain_code: u32 = self.path.domain_code();
//         let subdomain_code: u32 = self.path.subdomain_code();
//         domain_code * 10000 + subdomain_code * 1000 + self.code
//     }

//     pub fn decode(raw_code: u32) -> Option<Self> {
//         let code = raw_code % 1000;
//         let subdomain_code = (raw_code / 1000) % 10;
//         let domain = Domain::from_repr((raw_code / 10000) % 10)?;
//         let path: DomainPath = match domain {
//             Domain::Era => DomainPath::Era(EraSubdomain::from_repr(subdomain_code)?),
//             Domain::Compiler => DomainPath::Compiler(CompilerSubdomain::from_repr(subdomain_code)?),
//         };
//         Some(ErrorIdentifier { path, code })
//     }
// }
