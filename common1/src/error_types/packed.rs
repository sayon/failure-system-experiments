// use super::{
//     description::IErrorDescription,
//     domains::{DomainPath, EraSubdomain},
//     identifier::ErrorIdentifier,
// };

// pub type ErrorCode = u32;

// #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
// pub struct PackedError {
//     pub code: ErrorCode,
//     pub message: String,
//     pub data: serde_json::Value,
// }

// impl std::fmt::Display for PackedError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!(
//             "{{ code: {}; message: \"{}\"; data: \"{}\"}}",
//             self.code, self.message, self.data
//         ))
//     }
// }
// impl IErrorDescription for PackedError {
//     fn get_identifier(&self) -> ErrorIdentifier {
//         ErrorIdentifier::decode(self.code).expect("INTERNAL ERROR")
//     }

//     fn get_message(&self) -> String {
//         self.message.clone()
//     }

//     fn get_data(&self) -> serde_json::Value {
//         serde_json::value::to_value(&self).unwrap()
//     }
// }
