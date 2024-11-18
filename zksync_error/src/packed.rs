use std::fmt::Debug;

use crate::{
    error::{ICustomError, IError, IUnifiedError},
    identifier::Identifier,
    serialized::SerializedError,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PackedError<U>
where
    U: Clone + Debug,
{
    pub identifier: Identifier,
    pub message: String,
    pub data: U, // U = specific instance of ZksyncError
}

impl<T> IError<T> for PackedError<T>
where
    T: Clone + Debug + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    fn get_identifier(&self) -> Identifier {
        self.identifier.clone()
    }

    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_data(&self) -> T {
        self.data.clone()
    }
}

pub fn pack_unified<T, C>(s: T) -> Result<PackedError<T>, serde_json::Error>
where
    T: serde::Serialize + IUnifiedError<C> + Clone + Debug,
    C: Clone,
{
    Ok(PackedError {
        identifier: s.get_identifier(),
        message: s.get_message(),
        data: s,
    })
}

pub fn pack<T, U, C>(s: T) -> PackedError<U>
where
    T: serde::Serialize + ICustomError<U, C> + Clone,
    U: IUnifiedError<C> + Clone + Debug,
    C: Clone,
{
    pack_unified(s.to_unified()).expect("Serialization error")
}

pub fn serialized<T, C>(p: PackedError<T>) -> SerializedError
where
    T: Clone + Debug + serde::Serialize + IUnifiedError<C>,
    C: Clone,
{
    let data = serde_json::value::to_value(&p.data).expect("Serialization error");
    SerializedError {
        code: p.identifier.encode(),
        message: p.message,
        data,
    }
}
pub fn serialized_ref<T, C>(p: &PackedError<T>) -> SerializedError
where
    T: Clone + Debug + serde::Serialize + IUnifiedError<C>,
    C: Clone,
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
    T: Clone + Debug + serde::Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::value::to_value(&self.data).expect("Serialization error");
        let value_pretty = serde_json::to_string(&value).expect("Serialization error");
        let code = self.identifier.encode();
        let message = &self.message;
        let data = value_pretty;
        f.write_fmt(format_args!(
            r#"{{ code: {code} ; message: "{message}"; data: {data} }}"#,
        ))
    }
}
impl<T> std::error::Error for PackedError<T> where T: serde::Serialize + Debug + Clone {}
