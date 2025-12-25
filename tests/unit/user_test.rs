use super::*;
use validator::ValidationErrors;

#[test]
fn test_user_validation_valid() {
    let user = CreateUser {
        email: String::from("test@example.com"),
        password: String::from("securepassword"),
    };
    assert!(user.validate().is_ok());
}

#[test]
fn test_user_validation_invalid_email() {
    let user = CreateUser {
        email: String::from("invalid-email"),
        password: String::from("securepassword"),
    };
    let result = user.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.field_errors().contains_key("email"));
    }
}

#[test]
fn test_user_validation_short_password() {
    let user = CreateUser {
        email: String::from("test@example.com"),
        password: String::from("short"),
    };
    let result = user.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.field_errors().contains_key("password"));
    }
}

#[test]
fn test_user_hashing() {
    let password = "securepassword";
    let hashed = argon2::hash_encoded(password.as_bytes(), b"randomsalt", &Config::default()).unwrap();
    assert!(!hashed.is_empty());
}