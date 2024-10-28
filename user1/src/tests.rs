#![cfg(test)]
#![allow(dead_code)]
use strum_macros::EnumDiscriminants;

#[repr(u32)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(Code))]
enum StatusCode {
    Success { message: &'static str } = 200,
    BadRequest { message: &'static str } = 400,
    Unauthorized { message: &'static str } = 401,
    Forbidden { message: &'static str } = 403,
    NotFound { message: &'static str } = 404,
    InternalError { message: &'static str } = 500,
}

#[test]
fn test() {
    // Example usage of StatusCode with fields
    let success: Code = StatusCode::Success { message: "" }.into();
    let bad: Code = StatusCode::BadRequest { message: "" }.into();

    // Print each discriminant with its code
    assert_eq!(success as u32, 200);
    assert_eq!(bad as u32, 400);
}
