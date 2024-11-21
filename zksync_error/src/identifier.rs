use crate::error::domains::CompilerCode;
use crate::error::domains::ToolingCode;
use crate::kind::DomainCode;
use crate::kind::Kind;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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
        let component_code: i32 = self.kind.component_code();
        domain_code * 10000 + component_code * 1000 + self.code
    }

    pub fn decode(raw_code: i32) -> Option<Self> {
        let code = raw_code % 1000;
        let component_code = (raw_code / 1000) % 10;
        let domain = DomainCode::from_repr((raw_code / 10000) % 10)?;
        let kind: Kind = match domain {
            DomainCode::Compiler => {
                Kind::Compiler(CompilerCode::from_repr(component_code)?)
            }
            DomainCode::Tooling => {
                Kind::Tooling(ToolingCode::from_repr(component_code)?)
            }
        };
        Some(Identifier { kind, code })
    }
}
