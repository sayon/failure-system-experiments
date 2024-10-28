use crate::kind::{CompilerSubdomain, Domain, EraSubdomain, Kind, ToolingSubdomain};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identifier {
    pub kind: Kind,
    pub code: i32,
}

impl Identifier {
    pub fn new(kind: Kind, code: i32) -> Self {
        Self { kind, code }
    }

    pub fn encode(&self) -> i32 {
        let domain_code: i32 = self.kind.domain_code();
        let subdomain_code: i32 = self.kind.subdomain_code();
        domain_code * 10000 + subdomain_code * 1000 + self.code
    }

    pub fn decode(raw_code: i32) -> Option<Self> {
        let code = raw_code % 1000;
        let subdomain_code = (raw_code / 1000) % 10;
        let domain = Domain::from_repr((raw_code / 10000) % 10)?;
        let path: Kind = match domain {
            Domain::Era => Kind::Era(EraSubdomain::from_repr(subdomain_code)?),
            Domain::Compiler => Kind::Compiler(CompilerSubdomain::from_repr(subdomain_code)?),
            Domain::Tooling => Kind::Tooling(ToolingSubdomain::from_repr(subdomain_code)?),
        };
        Some(Identifier { kind: path, code })
    }
}
