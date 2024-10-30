#![cfg(test)]
#![allow(dead_code)]

use common::{
    kind::{EraSubdomain, Kind},
    packed::{pack, serialized, PackedError},
    serialized::{unpack_typed, unpack_untyped, SerializedError},
};

use crate::{
    error::domains::{CompilerError, ZksyncError},
    ZksolcError,
};

pub fn thrower_known() -> Result<(), PackedError<ZksyncError>> {
    Err(pack(ZksolcError::Generic {
        filename: "some_filename".to_string(),
        line: 10,
        column: 42,
    }))
}
pub fn thrower_known_serialized() -> Result<(), SerializedError> {
    Err(serialized(pack(ZksolcError::Generic {
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
        ZksyncError::CompilerError(compiler_error) => match &compiler_error {
            CompilerError::Zksolc(zksolc_error) => match &zksolc_error {
                ZksolcError::Generic { .. } => {
                    assert_eq!(
                        format!("{:#?}", &typed_error),
                        r#"CompilerError(
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
    data: CompilerError(
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
            CompilerError::Solc(_) => todo!(),
        },
        ZksyncError::ToolingError(_) => todo!(),
    }
}

pub fn handle_known_serialized(received_error: &SerializedError) {
    if let Ok(typed_error) = unpack_typed::<ZksyncError>(received_error) {
        match &typed_error {
            ZksyncError::CompilerError(compiler_error) => match compiler_error {
                CompilerError::Zksolc(zksolc_error) => match zksolc_error {
                    ZksolcError::Generic { .. } => {
                        println!("Caught known error: {:#?}", &typed_error);
                        println!(
                            "Don't have to use json to work with this error: {:} ",
                            &received_error
                        );
                    }
                    _ => todo!(),
                },
                CompilerError::Solc(_) => todo!(),
            },
            ZksyncError::ToolingError(_) => todo!(),
        }
    } else {
        println!("Use json to work with this error: {:} ", &received_error);
    }
}

pub fn thrower_unknown() -> Result<(), SerializedError> {
    Err(SerializedError::new_custom(
        Kind::Era(EraSubdomain::VM),
        242,
        "Message does not matter -- except for a possible prefix.",
        serde_json::json!(
            { "EraError" : { "VM" : { "SomeVMError" : { "somefield" : "somevalue", "intfield": 42 } } } }
        ),
    ))
}

#[test]
pub fn handle_unknown_serialized() {
    let received_error = thrower_unknown().unwrap_err();
    if let Ok(_) = unpack_typed::<ZksyncError>(&received_error) {
        unreachable!()
    } else {
        let error_object = unpack_untyped(&received_error).unwrap();

        assert_eq!(
            format!("{:#?}", error_object),
            r#"UntypedErrorObject {
    identifier: Identifier {
        kind: Era(
            VM,
        ),
        code: 242,
    },
    name: "SomeVMError",
    fields: {
        "intfield": Number(42),
        "somefield": String("somevalue"),
    },
    raw: Object {
        "EraError": Object {
            "VM": Object {
                "SomeVMError": Object {
                    "intfield": Number(42),
                    "somefield": String("somevalue"),
                },
            },
        },
    },
}"#
        );

        assert_eq!(
            error_object.fields.get("somefield"),
            Some(&serde_json::json!("somevalue"))
        );
        assert_eq!(
            error_object.fields.get("intfield"),
            Some(&serde_json::json!(42))
        );
    }
}
