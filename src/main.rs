use serde::Serialize;
use validator::{Validate, ValidateArgs, ValidationErrors};

pub mod pzn {
    pub mod validator {
        use crate::{DatabaseContext, RegisterUserRequest};
        use std::borrow::Cow;
        use validator::ValidationError;

        pub fn not_blank(value: &str) -> Result<(), ValidationError> {
            if value.trim().is_empty() {
                return Err(ValidationError::new("not_blank")
                    .with_message(Cow::from("Value cannot be blank")));
            }

            Ok(())
        }

        pub fn password_equals_confirm_password(
            request: &RegisterUserRequest,
        ) -> Result<(), ValidationError> {
            if request.password != request.confirm_password {
                return Err(ValidationError::new("password_equals_confirm_password")
                    .with_message(Cow::from("Password and confirm password must be same")));
            }

            Ok(())
        }

        pub fn can_register(
            request: &RegisterUserRequest,
            context: &DatabaseContext,
        ) -> Result<(), ValidationError> {
            if context.total >= context.max_data {
                return Err(
                    ValidationError::new("can_register").with_message(Cow::from(format!(
                        "cannot register user {}, database is full",
                        request.username
                    ))),
                );
            }

            Ok(())
        }
    }
}

pub struct DatabaseContext {
    total: i32,
    max_data: i32,
}

#[derive(Debug, Validate, Serialize)]
struct CreateCategoryRequest {
    #[validate(custom(function = "crate::pzn::validator::not_blank"))]
    id: String,
    #[validate(custom(function = "crate::pzn::validator::not_blank"))]
    name: String,
}

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
#[validate(context= DatabaseContext,
    schema(
        function = "pzn::validator::password_equals_confirm_password",
        skip_on_field_errors = false,
        code = "password",
        message = "password != confirm password"
    ),
    schema(
        function = "crate::pzn::validator::can_register",
        skip_on_field_errors = false,
        code = "username",
        use_context
    )
)]
pub struct RegisterUserRequest {
    #[validate(length(min = 3, max = 20, code = "username"))]
    username: String,
    #[validate(length(min = 3, max = 20, code = "password"))]
    password: String,
    #[validate(length(min = 3, max = 20, code = "confirm_password"))]
    confirm_password: String,
    #[validate(length(min = 3, max = 100, code = "name"))]
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
    #[validate(nested, length(min = 1))]
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
fn test_custom_validation() {
    let request = CreateCategoryRequest {
        id: "".to_string(),
        name: "        ".to_string(),
    };

    let errors: ValidationErrors = request.validate().err().unwrap();
    println!("{:?}", errors.errors());
}

#[test]
fn test_validate_vector_success() {
    let request = Product {
        id: "product-1".to_string(),
        name: "product-1".to_string(),
        variants: vec![
            ProductVariant {
                name: "variant-1".to_string(),
                price: 1000,
            },
            ProductVariant {
                name: "variant-2".to_string(),
                price: 2000,
            },
        ],
    };

    assert!(request.validate().is_ok());
}

#[test]
fn test_validate_vector_error() {
    let request = Product {
        id: "product-1".to_string(),
        name: "product-1".to_string(),
        variants: vec![
            ProductVariant {
                name: "".to_string(),
                price: -1000,
            },
            ProductVariant {
                name: "".to_string(),
                price: -2000,
            },
        ],
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
        confirm_password: "passwortaro".to_string(),
        name: "ekotaro".to_string(),
        address: AddressRequest {
            street: "jalan".to_string(),
            city: "kota".to_string(),
            country: "negara japantaro".to_string(),
        },
    };

    let context = DatabaseContext{
        total: 100,
        max_data: 1000,
    };

    assert!(request.validate_with_args(&context).is_ok());
}

#[test]
fn test_nested_struct_error() {
    let request = RegisterUserRequest {
        username: "o".to_string(),
        password: "passwortaro".to_string(),
        confirm_password: "salah".to_string(),
        name: "".to_string(),
        address: AddressRequest {
            street: "".to_string(),
            city: "".to_string(),
            country: "".to_string(),
        },
    };

    let context = DatabaseContext{
        total: 100,
        max_data: 100,
    };

    assert!(request.validate_with_args(&context).is_err());

    let errors: ValidationErrors = request.validate_with_args(&context).err().unwrap();
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
    println!("errors: {:#?}", errors);
}
