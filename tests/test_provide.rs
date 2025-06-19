#![feature(error_generic_member_access)]

use thiserror::Error;

#[derive(Error, Debug)]
#[provide(String => format!("test string"))]
#[error("test error")]
struct TestError;

#[derive(Error, Debug)]
enum TestEnum {
    #[provide(i32 => 42)]
    #[error("variant error")]
    Variant,
}

#[test]
fn test_provide_compiles() {
    let _err = TestError;
    let _enum_err = TestEnum::Variant;
}

// Test multiple provide attributes on the same struct
#[derive(Error, Debug)]
#[provide(String => format!("first string"))]
#[provide(i32 => 123)]
#[error("multiple provides")]
struct MultipleProvides;

#[test]
fn test_multiple_provides() {
    let _err = MultipleProvides;
}
