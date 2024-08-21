use std::{error::Error, fmt::Display, mem::discriminant};
use std::env::VarError;
use std::str::Utf8Error;
use std::time::SystemTimeError;
use axum::http::StatusCode;


use hmac::digest::InvalidLength;
use serde::Serialize;

use function_compose::FnError;

use crate::fnutils::ErrorType::EntityNotFound;

#[derive(Debug, Clone)]
pub enum ErrorType {
    //User Id
    UserNotFound(String),
    AuthError(String),
    // Role ID
    RoleNotFound(String),
    EntityNotFound(String),
    Unknown(String),
    DBInitError,
    DBError(String),
    InvalidInput(String),
    EmailAlreadyTaken(String),
}

#[derive( Serialize)]
pub struct ErrorObject{
    code: String,
    description: Option<String>,
    #[serde(skip_serializing)]
    pub error_type:ErrorType,
    
}

pub struct HttpErrorObject{
    pub status_code:Option<StatusCode>,
    error_object: ErrorObject,
}

impl HttpErrorObject{   
    pub fn new(error_object: ErrorObject)->Self{

        let status_code = match &error_object.error_type{
            ErrorType::UserNotFound(_s) => {
                StatusCode::BAD_REQUEST
            }
            ErrorType::AuthError(_s) => {
                 StatusCode::UNAUTHORIZED
            }
            ErrorType::RoleNotFound(_s) => {
                StatusCode::UNAUTHORIZED
            }
            EntityNotFound(_s) => {
                StatusCode::BAD_REQUEST
            }
            ErrorType::Unknown(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ErrorType::DBInitError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ErrorType::DBError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR

            }
            ErrorType::InvalidInput(_s) => {
                StatusCode::BAD_REQUEST
            }
            ErrorType::EmailAlreadyTaken(_s) => {
                StatusCode::BAD_REQUEST
            }

        };
        
        HttpErrorObject{
            status_code: Some(status_code),
            error_object:error_object,
        }
        
        
    }
}

impl ErrorObject{
    
    
    pub fn new(code:String, description:Option<String>, error_type:ErrorType)-> Self{
        ErrorObject{
            code,
            description,
            error_type
        }
    }

    /*pub fn new_with_error(code:String, description:Option<String>, status_code:StatusCode)-> Self{
        ErrorObject{
            code,
            description,
            status_code: Some(status_code),
        }
    }*/
}

impl From<FnError<ErrorType>> for ErrorObject{
    fn from(value: FnError<ErrorType>) -> Self {
        match value{
            FnError { underlying_error, error_code: _, description: _ } => {
                if underlying_error.is_some() {
                    underlying_error.unwrap().to_error_object()
                }else{
                    ErrorObject::new("E105".to_owned(),None, ErrorType::Unknown(String::new()))
                }
            }
        }
    }
}

pub fn map_to_error_object()-> fn(FnError<ErrorType>)->ErrorObject{
    |e|{
        match e{
            FnError { underlying_error, error_code: _, description: _ } => {
                if underlying_error.is_some() {
                    underlying_error.unwrap().to_error_object()
                }else{
                    ErrorObject::new("E105".to_owned(),None, ErrorType::Unknown(String::new()))
                }
                
            }
        }
    }
}


impl ErrorType{

    pub fn to_error_object(&self) -> ErrorObject {
        match self{
            ErrorType::UserNotFound(s) => {
                ErrorObject::new("E101".to_owned(),Some(s.clone()), self.clone())
            }
            ErrorType::AuthError(s) => {
                ErrorObject::new("E102".to_owned(), Some(s.clone()), self.clone()) }

            ErrorType::RoleNotFound(s) => {
                ErrorObject::new("E103".to_owned(),Some(s.clone()), self.clone())

            }
            EntityNotFound(s) => {
                ErrorObject::new("E104".to_owned(),Some(s.clone()), self.clone())
            }
            ErrorType::Unknown(_) => {
                ErrorObject::new("E105".to_owned(),None, self.clone())
            }
            ErrorType::DBInitError => {
                ErrorObject::new("E105".to_owned(),None, self.clone())
            }
            ErrorType::DBError(_) => {
                ErrorObject::new("E105".to_owned(),None, self.clone())

            }
            ErrorType::InvalidInput(s) => {
                ErrorObject::new("E106".to_owned(),Some(s.clone()), self.clone())
            }
            ErrorType::EmailAlreadyTaken(s) => {
                ErrorObject::new("E106".to_owned(),Some(s.clone()), self.clone())
            }
        }
    }
    /*pub fn to_error_object(&self) -> ErrorObject {
        match self{
            ErrorType::UserNotFound(s) => {
                ErrorObject::new_with_status_code("E101".to_owned(),Some(s.clone()), StatusCode::BAD_REQUEST)
            }
            ErrorType::AuthError(s) => {
                ErrorObject::new_with_status_code("E102".to_owned(), Some(s.clone()), StatusCode::UNAUTHORIZED) }

            ErrorType::RoleNotFound(s) => {
                ErrorObject::new_with_status_code("E103".to_owned(),Some(s.clone()), StatusCode::UNAUTHORIZED)

            }
            EntityNotFound(s) => {
                ErrorObject::new_with_status_code("E104".to_owned(),Some(s.clone()), StatusCode::BAD_REQUEST)
            }
            ErrorType::Unknown(_) => {
                ErrorObject::new_with_status_code("E105".to_owned(),None, StatusCode::INTERNAL_SERVER_ERROR)
            }
            ErrorType::DBInitError => {
                ErrorObject::new_with_status_code("E105".to_owned(),None, StatusCode::INTERNAL_SERVER_ERROR)
            }
            ErrorType::DBError(_) => {
                ErrorObject::new_with_status_code("E105".to_owned(),None, StatusCode::INTERNAL_SERVER_ERROR)

            }
            ErrorType::InvalidInput(s) => {
                ErrorObject::new_with_status_code("E106".to_owned(),Some(s.clone()), StatusCode::BAD_REQUEST)
            }
            ErrorType::EmailAlreadyTaken(s) => {
                ErrorObject::new_with_status_code("E106".to_owned(),Some(s.clone()), StatusCode::BAD_REQUEST)
            }
        }
    }*/
}



impl Into<AppError> for ErrorType {
    fn into(self) -> AppError {
        AppError {
            underlying_error: None,
            error_type: self,
        }
    }
}

//type ErrorTypeInfo = (ErrorType, String,);
pub struct ErrorTypeInfo{
    error_type: ErrorType,
    description: String
}

impl ErrorTypeInfo{
    pub fn new(info:(ErrorType, String,))->Self{
        ErrorTypeInfo{
            error_type: info.0,
            description: info.1,            
        }
    }
}


impl Into<FnError<ErrorType>> for ErrorType{
    fn into(self) -> FnError<ErrorType> {
        FnError {
            underlying_error: Some(self),
            description: None,
            error_code: None,
        }
    }
}

impl Into<FnError<ErrorType>> for ErrorTypeInfo{
    fn into(self) -> FnError<ErrorType> {
        FnError {
            underlying_error: Some(self.error_type),
            description: Some(self.description),
            error_code: None,
        }
    }
}

impl From<diesel::result::Error> for ErrorType{
    fn from(error: diesel::result::Error) -> Self {
        match &error{

            diesel::result::Error::NotFound => {
                ErrorType::EntityNotFound(error.to_string())
            },
            _e => {ErrorType::DBError(error.to_string())}
            
            
        }
    }
}

pub struct AppError {
    pub error_type: ErrorType,
    pub underlying_error: Option<Box<dyn Error>>,
}

pub type AppResult<T> = Result<T, AppError>;

pub type FnResult<T, E> = Result<T, FnError<E>>;


pub trait ToAppResult<T>{
    fn to_app_result(self)->AppResult<T>;
}

pub trait ToFnResult<T, E>{
    fn to_fn_result(self)->FnResult<T, E>;
}

impl<T, E> ToAppResult<T> for Result<T, E>
where
    E: Error + 'static,
{
    fn to_app_result(self) -> AppResult<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(AppError {
                error_type: ErrorType::Unknown(e.to_string()),
                underlying_error: Some(Box::new(e)),
            }),
        }
    }
}

impl<T, E> ToFnResult<T, E> for Result<T, E>
    where
        E: Error + Send + 'static,
{
    fn to_fn_result(self) -> FnResult<T, E> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(FnError {
                error_code: None,
                description: None,
                underlying_error: Some(e),
            }),
        }
    }
}
/* impl<T> From<T> for AppError where T:Display{

    fn from(value: T) -> Self {
        todo!()
    }

} */

/*impl From<BcryptError> for AppError {
    fn from(value:  BcryptError ) -> Self {
        println!("Diesel Error converted to AppError {}", value);
        AppError {
            errorType: ErrorType::Unknown(value.to_string()),
            underlyingError: Some(Box::new(value)),
        }
    }
}*/

pub struct ErrorMapper<K, F> {
    error_map: Vec<(K, F)>,
}

pub fn map_result_not_found_error<'a>(error_type: &'a ErrorType)-> impl Fn(diesel::result::Error) -> FnError<ErrorType> + 'a{
    move |error| {
        match error {
            diesel::result::Error::NotFound => {
                error_type.clone().into()
            }
            _ => {
                ErrorType::Unknown(error.to_string().clone()).into()
            }
        }
    }
}

pub fn map_to_unknown_error()-> fn(diesel::result::Error) -> FnError<ErrorType>{
    |error| {
        ErrorType::Unknown(error.to_string()).into()

    }
}

pub fn map_hmac_invalid_length_to_unknown_error()-> fn(InvalidLength) -> FnError<ErrorType>{
    |e| {
        ErrorType::Unknown(e.to_string()).into()
    }
}

pub fn map_jwt_error_to_unknown_error()-> fn(jwt::Error) -> FnError<ErrorType>{
    |e| {
        ErrorType::Unknown(e.to_string()).into()
    }
}

pub fn map_to_unknown_bcrypt_error()-> fn(bcrypt::BcryptError) -> FnError<ErrorType>{
    |error| {
        ErrorType::Unknown(error.to_string()).into()
    }
}

pub fn map_to_unknown_db_error()-> fn(diesel::result::Error) -> FnError<ErrorType>{
    |error| {
        ErrorType::Unknown(error.to_string()).into()
    }
}

pub fn map_to_unknown_var_error()-> fn(VarError) -> FnError<ErrorType>{
    |error| {
        ErrorType::Unknown(error.to_string()).into()
    }
}

pub fn map_to_unknown_utf_error()-> fn(Utf8Error) -> FnError<ErrorType>{
    |error| {
        ErrorType::Unknown(error.to_string()).into()
    }
}

pub fn map_to_unknown_system_time_error()-> fn(SystemTimeError) -> FnError<ErrorType>{
    |error| {
        ErrorType::Unknown(error.to_string()).into()
    }
}

impl<K, F> ErrorMapper<K, F>
where
    F: FnOnce() -> ErrorType,
{
    pub fn new() -> ErrorMapper<K, F> {
        ErrorMapper {
            error_map: Vec::new(),
        }
    }

    pub fn add(&mut self, k: K, f: F) -> &mut Self {
        self.error_map.push((k, f));
        self
    }

    fn get_error_type(&self, k: &K) -> Option<&(K, F)> {
        self.error_map
            .iter()
            .find(|item| discriminant(&item.0) == discriminant(k))
    }
}

pub fn convert_to_app_error<K, F>(e: &ErrorMapper<K, F>, err: K) -> AppError
where
    F: Fn() -> ErrorType,
    K: Display,
{
    let matching_error_type = e.get_error_type(&err);
    if matching_error_type.is_some() {
        let borrow_mut = matching_error_type.unwrap();
        let var_name = &borrow_mut.1;
        var_name().into()
    } else {
        ErrorType::Unknown(err.to_string()).into()
    }
}

pub fn convert_to_fn_error<K, F>(e: &ErrorMapper<K, F>, err: K) -> FnError<ErrorType>
    where
        F: Fn() -> ErrorType,
        K: Display,
{
    let matching_error_type = e.get_error_type(&err);
    if matching_error_type.is_some() {
        let borrow_mut = matching_error_type.unwrap();
        let var_name = &borrow_mut.1;
        var_name().into()
    } else {
        ErrorType::Unknown(err.to_string()).into()
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}



