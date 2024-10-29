use crate::{
    error::{ICustomError, IUnifiedError},
    identifier::Identifier,
    serialized::SerializedError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackedError<U>
where
    U: serde::Serialize,
{
    pub identifier: Identifier,
    pub message: String,
    pub data: U, // U = specific instance of ZksyncError
}

pub fn pack_unified<T>(s: T) -> Result<PackedError<T>, serde_json::Error>
where
    T: serde::Serialize + IUnifiedError + Clone,
{
    Ok(PackedError {
        identifier: s.get_identifier(),
        message: s.get_message(),
        data: s,
    })
}

pub fn pack<T, U>(s: T) -> PackedError<U>
where
    T: serde::Serialize + ICustomError<U> + Clone,
    U: IUnifiedError + Clone,
{
    pack_unified(s.to_unified()).expect("Serialization error")
}

pub fn serialized<T>(p: PackedError<T>) -> SerializedError
where
    T: serde::Serialize + IUnifiedError,
{
    let data = serde_json::value::to_value(&p.data).expect("Serialization error");
    SerializedError {
        code: p.identifier.encode(),
        message: p.message,
        data,
    }
}
pub fn serialized_ref<T>(p: &PackedError<T>) -> SerializedError
where
    T: serde::Serialize + IUnifiedError,
{
    let data = serde_json::value::to_value(&p.data).expect("Serialization error");
    SerializedError {
        code: p.identifier.encode(),
        message: p.message.clone(),
        data,
    }
}

impl<T> std::fmt::Display for PackedError<T>
where
    T: serde::Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //FIXME dirty
        let value = serde_json::value::to_value(&self.data).expect("Serialization error");
        let value_pretty = serde_json::to_string(&value).expect("Serialization error");
        f.write_fmt(format_args!(
            "{{ code: {} ; message: \"{}\"; data: {} }}",
            self.identifier.encode(),
            self.message,
            value_pretty
        ))
    }
}
impl<T> std::error::Error for PackedError<T> where T: serde::Serialize + std::fmt::Debug {}
