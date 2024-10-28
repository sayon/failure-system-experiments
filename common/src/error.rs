use crate::identifier::Identifier;

pub trait IUnifiedError: serde::Serialize + for<'de> serde::Deserialize<'de> {
    fn get_identifier(&self) -> Identifier;
    fn get_message(&self) -> String;
}

pub trait ICustomError<U>
where
    U: IUnifiedError,
{
    fn to_unified(&self) -> U;
}

pub trait CustomErrorMessage {
    fn get_message(&self) -> String;
}
