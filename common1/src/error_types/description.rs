// use super::{identifier::ErrorIdentifier, packed::PackedError};

// // Unknown errors from the wild, that are not encoded in types. We explore them through JSON
// pub trait IErrorDescription: std::fmt::Debug + serde::Serialize {
//     fn get_identifier(&self) -> ErrorIdentifier;
//     fn get_message(&self) -> String;
//     fn get_data(&self) -> serde_json::Value {
//         serde_json::value::to_value(&self).unwrap()
//     }
// }

// pub fn pack(s: impl IErrorDescription) -> PackedError {
//     PackedError {
//         code: s.get_identifier().clone().encode(),
//         message: s.get_message(),
//         data: s.get_data(),
//     }
// }

// impl std::error::Error for PackedError {}
