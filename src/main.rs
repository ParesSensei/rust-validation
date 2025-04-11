use serde::Serialize;
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

#[derive(Debug, Validate)]
struct Product {
    #[validate(length(min = 3, max = 200))]
    id: String,
    #[validate(length(min = 3, max = 200))]
    name: String,
    #[validate(nested, length(min =1))]
    variants: Vec<ProductVariant>,
}

#[derive(Debug, Validate, Serialize)]
struct ProductVariant {
    #[validate(length(min = 3, max = 100))]
    name: String,
    #[validate(range(min = 12, max = 100000000))]
    price: i32,
}

#[test]
fn test_validate_vector_success() {
    let request = Product{
        id: "product-1".to_string(),
        name: "product-1".to_string(),
        variants: vec![
            ProductVariant{
                name: "variant-1".to_string(),
                price: 1000
            },
            ProductVariant{
                name: "variant-2".to_string(),
                price: 2000
            },
        ]
    };

    assert!(request.validate().is_ok());
}

#[test]
fn test_validate_vector_error() {
    let request = Product{
        id: "product-1".to_string(),
        name: "product-1".to_string(),
        variants: vec![
            ProductVariant{
                name: "".to_string(),
                price: -1000
            },
            ProductVariant{
                name: "".to_string(),
                price: -2000
            },
        ]
    };

    assert!(request.validate().is_err());
    let error = request.validate().err().unwrap();
    println!("{:?}", error.errors());
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
