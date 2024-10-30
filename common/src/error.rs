use std::error::Error;

use crate::identifier::Identifier;

pub trait IError<ContainedType>: Error
where
    ContainedType: Clone,
{
    fn get_identifier(&self) -> Identifier;
    fn get_message(&self) -> String;
    fn get_data(&self) -> ContainedType;
}

pub trait IUnifiedError<ContainedType>:
    serde::Serialize + for<'de> serde::Deserialize<'de> + IError<ContainedType>
where
    ContainedType: Clone,
{
}

pub trait ICustomError<U, C>
where
    U: IUnifiedError<C>,
    C: Clone,
{
    fn to_unified(&self) -> U;
}

pub trait CustomErrorMessage {
    fn get_message(&self) -> String;
}
