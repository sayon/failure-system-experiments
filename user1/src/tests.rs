#![cfg(test)]
#![allow(dead_code)]

use zksync_error::error::definitions::Zksolc;
use zksync_error::error::domains::Compiler;
use zksync_error::error::domains::ZksyncError;
use zksync_error::kind::Kind;
use zksync_error::packed::pack;
use zksync_error::packed::serialized;
use zksync_error::packed::PackedError;
use zksync_error::serialized::unpack_typed;
use zksync_error::serialized::unpack_untyped;
use zksync_error::serialized::SerializedError;

pub fn thrower_known() -> Result<(), PackedError<ZksyncError>> {
    Err(pack(Zksolc::Umbrella {
        inner: serde_json::json! { "null" },
    }))
}
pub fn thrower_known_serialized() -> Result<(), SerializedError> {
    Err(serialized(pack(Zksolc::FileNotFound {
        path: "e".into(),
        file_index: 1,
    })))
}

#[test]
pub fn handle_known() {
    let received_error: PackedError<ZksyncError> = thrower_known().unwrap_err();
    let typed_error = &received_error.data;
    match typed_error {
        ZksyncError::Compiler(compiler_error) => match &compiler_error {
            Compiler::Zksolc(zksolc_error) => match &zksolc_error {
                Zksolc::Umbrella { inner } => {
                    assert_eq!(
                        format!("{:#?}", &typed_error),
  "Compiler(\n    Zksolc(\n        Umbrella {\n            inner: String(\"null\"),\n        },\n    ),\n)");

                    assert_eq!(
                        format!("{:#?}", &received_error),
                        "PackedError {
    identifier: Identifier {
        kind: Compiler(
            Zksolc,
        ),
        code: 42,
    },
    message: \"Any error!\",
    data: Compiler(
        Zksolc(
            Umbrella {
                inner: String(\"null\"),
            },
        ),
    ),
}"
                    );
                }
                Zksolc::FileNotFound { .. } => todo!(),
                Zksolc::SolcNotFound { path, payload } => todo!(),
                _ => todo!(),
            },
        },
        ZksyncError::Core(_) => todo!(),
    }
}

pub fn handle_known_serialized(received_error: &SerializedError) {
    if let Ok(typed_error) = unpack_typed::<ZksyncError>(received_error) {
        match &typed_error {
            ZksyncError::Compiler(compiler_error) => match compiler_error {
                Compiler::Zksolc(zksolc_error) => match zksolc_error {
                    Zksolc::FileNotFound { .. } => {
                        println!("Caught known error: {:#?}", &typed_error);
                        println!(
                            "Don't have to use json to work with this error: {:} ",
                            &received_error
                        );
                    }
                    _ => todo!(),
                },
            },
            ZksyncError::Core(_) => todo!(),
        }
    } else {
        println!("Use json to work with this error: {:} ", &received_error);
    }
}

pub fn thrower_unknown() -> Result<(), SerializedError> {
    Err(SerializedError::new_custom(
        Kind::Compiler(zksync_error::error::domains::CompilerCode::Zksolc),
        242,
        "Message does not matter -- except for a possible prefix.",
        serde_json::json!(
            { "Tooling" : { "RustSDK" : { "VeryWrongTool" : { "info" : "somevalue" } } } }
        ),
    ))
}

#[test]
pub fn handle_unknown_serialized() {
    let received_error = thrower_unknown().unwrap_err();
    if let Ok(e) = unpack_typed::<ZksyncError>(&received_error) {
        println!("{e:?}");
        unreachable!()
    } else {
        let error_object = unpack_untyped(&received_error).unwrap();

        assert_eq!(
            format!("{:#?}", error_object),
            "UntypedErrorObject {\n    identifier: Identifier {\n        kind: Tooling(\n            RustSDK,\n        ),\n        code: 242,\n    },\n    name: \"VeryWrongTool\",\n    fields: {\n        \"info\": String(\"somevalue\"),\n    },\n    raw: Object {\n        \"Tooling\": Object {\n            \"RustSDK\": Object {\n                \"VeryWrongTool\": Object {\n                    \"info\": String(\"somevalue\"),\n                },\n            },\n        },\n    },\n}"
        );

        assert_eq!(
            error_object.fields.get("info"),
            Some(&serde_json::json!("somevalue"))
        );
    }
}
