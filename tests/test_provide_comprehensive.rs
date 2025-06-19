#![feature(error_generic_member_access)]

use std::error::request_value;
use thiserror::Error;

#[derive(Error, Debug)]
#[provide(String => format!("Error code: {}", self.code))]
#[provide(i32 => self.code)]
#[error("Custom error with code {code}")]
struct CustomError {
    code: i32,
    message: String,
}

#[derive(Error, Debug)]
enum NetworkError {
    #[provide(String => format!("Connection failed to {}", host))]
    #[provide(u16 => *port)]
    #[error("Connection failed to {host}:{port}")]
    ConnectionFailed { host: String, port: u16 },

    #[provide(String => "Timeout occurred".to_string())]
    #[error("Request timed out")]
    Timeout,
}

#[derive(Error, Debug)]
#[provide(String => format!("Struct with backtrace: {}", self.message))]
#[error("Error with backtrace: {message}")]
struct ErrorWithBacktrace {
    message: String,
    #[backtrace]
    backtrace: std::backtrace::Backtrace,
}

#[test]
fn test_struct_provide_with_field_access() {
    let error = CustomError {
        code: 404,
        message: "Not found".to_string(),
    };

    // Test that we can retrieve the provided values
    let provided_string: Option<String> = request_value(&error);
    let provided_code: Option<i32> = request_value(&error);

    assert!(provided_string.is_some());
    assert!(provided_code.is_some());
    assert_eq!(provided_code, Some(404));
    assert_eq!(provided_string.as_ref().unwrap(), "Error code: 404");
}

#[test]
fn test_enum_provide_with_field_access() {
    let error = NetworkError::ConnectionFailed {
        host: "example.com".to_string(),
        port: 8080,
    };

    let provided_string: Option<String> = request_value(&error);
    let provided_port: Option<u16> = request_value(&error);

    assert!(provided_string.is_some());
    assert!(provided_port.is_some());
    assert_eq!(provided_port, Some(8080));
    assert_eq!(
        provided_string.as_ref().unwrap(),
        "Connection failed to example.com"
    );
}

#[test]
fn test_enum_provide_without_fields() {
    let error = NetworkError::Timeout;

    let provided_string: Option<String> = request_value(&error);
    assert!(provided_string.is_some());
    assert_eq!(provided_string.as_ref().unwrap(), "Timeout occurred");
}

#[test]
fn test_provide_with_backtrace() {
    let error = ErrorWithBacktrace {
        message: "Test message".to_string(),
        backtrace: std::backtrace::Backtrace::capture(),
    };

    // Should provide the custom string (backtrace testing is more complex)
    let provided_string: Option<String> = request_value(&error);

    assert!(provided_string.is_some());
    assert_eq!(
        provided_string.as_ref().unwrap(),
        "Struct with backtrace: Test message"
    );

    // Note: Backtrace testing requires more complex setup and may not work in all environments
    // The important thing is that our custom provide works alongside existing backtrace functionality
}
