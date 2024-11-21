#![cfg(test)]
#![allow(dead_code)]


use zksync_error::serialized::SerializedError;
use zksync_error::serialized::unpack_untyped;
use zksync_error::serialized::unpack_typed;
use zksync_error::packed::PackedError;
use zksync_error::packed::serialized;
use zksync_error::packed::pack;
use zksync_error::kind::Kind;
use zksync_error::error::domains::Compiler;
use zksync_error::error::domains::ZksyncError;
use zksync_error::error::definitions::Zksolc;

pub fn thrower_known() -> Result<(), PackedError<ZksyncError>> {
    Err(pack(Zksolc::Generic {
        filename: "some_filename".to_string(),
        line: 10,
        column: 42,
    }))
}
pub fn thrower_known_serialized() -> Result<(), SerializedError> {
    Err(serialized(pack(Zksolc::Generic {
        filename: "some_filename".to_string(),
        line: 10,
        column: 42,
    })))
}

#[test]
pub fn handle_known() {
    let received_error: PackedError<ZksyncError> = thrower_known().unwrap_err();
    let typed_error = &received_error.data;
    match typed_error {
        ZksyncError::Compiler(compiler_error) => match &compiler_error {
            Compiler::Zksolc(zksolc_error) => match &zksolc_error {
                Zksolc::Generic { .. } => {
                    assert_eq!(
                        format!("{:#?}", &typed_error),
                        r#"Compiler(
    Zksolc(
        Generic {
            filename: "some_filename",
            line: 10,
            column: 42,
        },
    ),
)"#
                    );

                    assert_eq!(
                        format!("{:#?}", &received_error),
                        r#"PackedError {
    identifier: Identifier {
        kind: Compiler(
            Zksolc,
        ),
        code: 42,
    },
    message: "Some error in zksolc when processing  some_filename line 10 col 42",
    data: Compiler(
        Zksolc(
            Generic {
                filename: "some_filename",
                line: 10,
                column: 42,
            },
        ),
    ),
}"#
                    );
                }
                _ => todo!(),
            },
            Compiler::Solc(_) => todo!(),
        },
        ZksyncError::Tooling(_) => todo!(),
    }
}

pub fn handle_known_serialized(received_error: &SerializedError) {
    if let Ok(typed_error) = unpack_typed::<ZksyncError>(received_error) {
        match &typed_error {
            ZksyncError::Compiler(compiler_error) => match compiler_error {
                Compiler::Zksolc(zksolc_error) => match zksolc_error {
                    Zksolc::Generic { .. } => {
                        println!("Caught known error: {:#?}", &typed_error);
                        println!(
                            "Don't have to use json to work with this error: {:} ",
                            &received_error
                        );
                    }
                    _ => todo!(),
                },
                Compiler::Solc(_) => todo!(),
            },
            ZksyncError::Tooling(_) => todo!(),
        }
    } else {
        println!("Use json to work with this error: {:} ", &received_error);
    }
}

pub fn thrower_unknown() -> Result<(), SerializedError> {
    Err(SerializedError::new_custom(
        Kind::Tooling(zksync_error::error::domains::ToolingCode::RustSDK),
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
