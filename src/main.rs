use validator::{Validate, ValidationErrors};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Validate)]
struct LoginRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "username length must be between 3 and 20"
    ))]
    username: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "password length must be between 3 and 20"
    ))]
    password: String,
}

#[derive(Debug, Validate)]
struct AddressRequest {
    #[validate(length(min = 1, max = 100))]
    street: String,
    #[validate(length(min = 1, max = 100))]
    city: String,
    #[validate(length(min = 1, max = 100))]
    country: String,
}

#[derive(Debug, Validate)]
struct RegisterUserRequest {
    #[validate(length(min = 3, max = 20))]
    username: String,
    #[validate(length(min = 3, max = 20))]
    password: String,
    #[validate(length(min = 3, max = 100))]
    name: String,
    #[validate(nested)]
    address: AddressRequest,
}

#[test]
fn test_nested_struct_success() {
    let request = RegisterUserRequest {
        username: "ekoatro".to_string(),
        password: "passwortaro".to_string(),
        name: "ekotaro".to_string(),
        address: AddressRequest{
            street: "jalan".to_string(),
            city: "kota".to_string(),
            country: "negara japantaro".to_string(),
        }
    };

    assert!(request.validate().is_ok());
}

#[test]
fn test_nested_struct_error() {
    let request = RegisterUserRequest {
        username: "ekoatro".to_string(),
        password: "passwortaro".to_string(),
        name: "ekotaro".to_string(),
        address: AddressRequest{
            street: "".to_string(),
            city: "".to_string(),
            country: "".to_string(),
        }
    };

    assert!(request.validate().is_err());

    let errors: ValidationErrors = request.validate().err().unwrap();
    println!("{:#?}", errors.errors());
}

#[test]
fn test_validate_success() {
    let login = LoginRequest {
        username: "eko".to_string(),
        password: "rahasia".to_string(),
    };

    assert!(login.validate().is_ok());
}

#[test]
fn test_validate_failed() {
    let login = LoginRequest {
        username: "ek".to_string(),
        password: "rahasia".to_string(),
    };

    assert!(login.validate().is_err());

    let errors: ValidationErrors = login.validate().err().unwrap();
    println!("errors: {:?}", errors);
}
