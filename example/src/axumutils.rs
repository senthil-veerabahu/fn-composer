use std::{collections::BTreeMap, convert::Infallible, env, ops::{DerefMut, Deref}};

use axum::{extract::{FromRef, FromRequest, FromRequestParts, State}, async_trait, http::request::Parts, RequestPartsExt, Json};
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use function_compose::FnError;
use jwt::{Header, Token, VerifyWithKey};
use crate::fnutils::*;
use crate::{db::{createConnectionPool, DBConnection}};
use crate::fnutils::ErrorType::EntityNotFound;
use crate::utils::secutils::verify_token;


#[derive(Clone)]
pub struct AppDBConnectionPool{
    pub connectionPool: Option<Pool<AsyncPgConnection>>
}

#[derive(Clone)]
pub struct AppState{
    pub connectionPool: AppDBConnectionPool
}


unsafe impl Send for AppState{}
unsafe impl Sync for AppState{}

#[async_trait]
impl<S> FromRequestParts<S> for AppState 
    where Self: FromRef<S>, S: Send + Sync{
    type Rejection = AppError;

    async fn from_request_parts(_req:  &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(AppState::from_ref(state))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_type = self.error_type.to_string();
        let r = (StatusCode::INTERNAL_SERVER_ERROR,
                 error_type).into_response();
        r
    }
}

struct AppErrorResponse<ErrorType>{
    pub underlying_error: Option<ErrorType>,
    pub error_code:Option<String>,
    pub description: Option<String>
}

impl From<FnError<ErrorType>> for AppErrorResponse<ErrorType>{
    fn from(value: FnError<ErrorType>) -> Self {
        AppErrorResponse{
            underlying_error: value.underlying_error,
            error_code: value.error_code,
            description:value.description,
        }
    }
}

/*impl IntoResponse for AppErrorResponse<ErrorType> {
    fn into_response(self) -> axum::response::Response {
        let error_type = self.underlying_error.to_string();
        let r = (StatusCode::INTERNAL_SERVER_ERROR,
                 error_type).into_response();
        r
    }
}*/

 
 impl FromRef<State<AppState>> for AppState{
    fn from_ref(input: &State<AppState>) -> AppState {
        input.0.clone()
        
    }
}

impl AppState {
    pub async fn initConnection(&mut self){
        let connectionPoolResult = createConnectionPool().await;
        if connectionPoolResult.is_err() {
            panic!("Failed to start app");
        }else {            
            self.connectionPool  = AppDBConnectionPool {
                connectionPool:Some(connectionPoolResult.ok().unwrap())
            };
        }
    }
}

pub async fn createAppState() ->AppState{
    let mut appState:AppState = AppState{
        connectionPool: AppDBConnectionPool{
            connectionPool: None
        }
    };
    appState.initConnection().await;
    println!("appstate connection pool {}", appState.connectionPool.connectionPool.is_none());
    appState
}

pub struct DBConnectionHolder {
    dbConnection: DBConnection
}

impl DerefMut for DBConnectionHolder{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dbConnection
    }
}

impl Deref for DBConnectionHolder{
    type Target = DBConnection;

    fn deref(&self) -> &Self::Target {
        &self.dbConnection
    }
}

struct RequestUser{
    userId: String,
    email:String,
    role:String,
    is_email_verified: bool,
    is_phone_verified: bool
}

pub struct AuthUserData{
    auth_user:Option<RequestUser>
}

trait ToHttpStatusCode {
    fn to_http_status_code(&self) -> StatusCode;
}


impl ToHttpStatusCode for ErrorObject{
    fn to_http_status_code(&self) -> StatusCode{
        match &self.error_type{
            ErrorType::UserNotFound(s) => {
                StatusCode::BAD_REQUEST
            }
            ErrorType::AuthError(s) => {
                StatusCode::UNAUTHORIZED
            }
            ErrorType::RoleNotFound(s) => {
                StatusCode::UNAUTHORIZED
            }
            EntityNotFound(s) => {
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
            ErrorType::InvalidInput(s) => {
                StatusCode::BAD_REQUEST
            }
            ErrorType::EmailAlreadyTaken(s) => {
                StatusCode::BAD_REQUEST
            }
        }
    }
}

impl IntoResponse for ErrorObject{
    fn into_response(self) -> Response {
        match &self{
            &_ => {}
        }
        (self.to_http_status_code(),   Json(self)).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUserData where AppState: FromRef<S>, S:Send+Sync{

    type Rejection = ErrorObject;

    async fn from_request_parts(req:  &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let authOption = req.headers.get(axum::http::header::AUTHORIZATION);
        let r:Result<AuthUserData, ErrorObject> = match authOption{
            Some(authHeaderValue) => {
                let data = std::str::from_utf8(authHeaderValue.as_bytes()).map_err(map_to_unknown_utf_error()).map_err(map_to_error_object())?;
                let mut tokens = data.split_whitespace();
                
                let has_bearer_string: Option<&str> = tokens.next();
                match has_bearer_string{
                    Some(_) => {
                        let has_bearer_token: Option<&str> = tokens.next();
                        match has_bearer_token{
                            Some(jwtToken) => {
                                let jwt_signing_key = env::var("JWT_SIGNING_KEY").map_err(map_to_unknown_var_error()).map_err(map_to_error_object())?;
                                let claims = verify_token(jwtToken, jwt_signing_key).map_err(map_to_error_object())?;
                                Ok(AuthUserData{
                                    auth_user: Some(RequestUser{
                                        userId: claims["userId"].clone(),
                                        email:claims["email"].clone(),
                                        role:claims["role"].clone(),
                                        is_email_verified: claims["isEmailVerfied"].parse().unwrap(),
                                        is_phone_verified: claims["isPhoneVerfied"].parse().unwrap(),
                                    }),
                                })        
                            }
                            None => Err(ErrorType::AuthError("User Not found in request".to_owned()).to_error_object())
                        }
                    }
                    None => Err(ErrorType::AuthError("User Not found in request".to_owned()).to_error_object())
                }
            },
            None => Err(ErrorType::AuthError("User Not found in request".to_owned()).to_error_object())
        };
        let r = r?;
        Ok(r)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for DBConnectionHolder where AppState: FromRef<S>, S: Send + Sync{
    
    type Rejection = AppError;

    async fn from_request_parts(req:  &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = req.extract_with_state::<AppState, _>(state);
        //let x = state;
        let connection_pool = state.await?.connectionPool.connectionPool.as_ref().unwrap().clone();
        let r:DBConnection  = DBConnection::new(connection_pool);        
        Ok(DBConnectionHolder { dbConnection: r })
        //Ok(Arc::new(r))
    }
}

pub struct Qs<T>(pub T);

#[axum::async_trait]
impl<S,T> FromRequestParts<S> for Qs<T>
where
    T: serde::de::DeserializeOwned,
{
    type Rejection = Infallible;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // TODO: error handling
        let query = req.uri.query().unwrap();        
        let from_str = serde_qs::from_str(query);
        Ok(Self(from_str.unwrap()))
    }
}

