use crate::identifier::Identifier;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntypedErrorObject {
    pub identifier: Identifier,
    pub name: String,
    pub fields: serde_json::Map<String, serde_json::Value>, // Specific value introduced by user; unpacked from the Domain/subdomain and error name
    pub raw: serde_json::Value, // Specific value introduced by user; unpacked from the Domain/subdomain.
}