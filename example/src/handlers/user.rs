

use axum::Json;
use axum_macros::debug_handler;

use diesel_async::AsyncPgConnection;
use function_compose::*;
use futures::{FutureExt, future::BoxFuture};
use serde::{Serialize, Deserialize};

use crate::{axumutils::{DBConnectionHolder, AppState}, db::{DBConnection, DBConnProvider}, model::{User}, repository::{repository::RepositoryDB, user_repository::{  AuthData, NewUser, UserRepository}}, utils::secutils::generate_jwt_token}; 

use std::{env, ops::Add, time::{Duration, SystemTime, UNIX_EPOCH}};
use function_compose::composeable;
use crate::fnutils::{ErrorObject, ErrorType, map_to_error_object, map_to_unknown_system_time_error, map_to_unknown_var_error};


#[composeable()]
pub  fn authenticate(_auth_request: AuthRequest, _conn: &mut DBConnection) ->BoxFuture<Result<AuthData , FnError<ErrorType>>>{
    async{
        let value: &mut AsyncPgConnection = _conn.current_connection().await?;
        let mut user_repository = RepositoryDB::from(value);
        let auth_data:AuthData = user_repository
                .auth(String::from(_auth_request.user), String::from(_auth_request.pass)).await?;
        
        Ok(auth_data)
    }.boxed()
}

#[composeable()]
pub  fn generate_token<'a>(user_data: AuthData) ->BoxFuture<'a, Result<(User,String), FnError<ErrorType>>>{
    async move{
        let jwt_signing_key = env::var("JWT_SIGNING_KEY").map_err(map_to_unknown_var_error())?;
        let user = user_data.0;
        let role = &user_data.2;
        let role_names:Vec<&str> = vec![&role.name];
        let start = SystemTime::now();
        let mut since_the_epoch = start.duration_since(UNIX_EPOCH).map_err(map_to_unknown_system_time_error())?;
        since_the_epoch = since_the_epoch.add(Duration::from_secs(60*60));
        let token = generate_jwt_token(&user.email, &user.email, jwt_signing_key,
             role_names, (&user.isemail_verfied).clone().is_some_and(|r| r), 
             (&user.is_phone_verfied).clone().is_some_and(|r|r), since_the_epoch)?; 
        let result = (user, token);
        Ok(result)
    }.boxed()
}


#[derive(Serialize, Debug)]
pub struct AuthResponse{
    token: String,
    email_verified:        bool,
    email:                String,
    mobile:               String,
    mobile_verified_status: bool,
    is_active:             bool,
    location_identified:   bool,
}

#[composeable()]
pub fn  pack_auth_result(user_data: (User, String)) -> Result<AuthResponse, FnError<ErrorType>>{
    Ok(AuthResponse{
        token: user_data.1,
        email_verified: user_data.0.isemail_verfied.is_some_and(|e|e),
        email: user_data.0.email,
        mobile: user_data.0.phone_number.unwrap_or_default(),
        mobile_verified_status: user_data.0.is_phone_verfied.is_some_and(|e|e),
        is_active: user_data.0.is_active.is_some_and(|e|e),
        location_identified:false,
    })
}


impl From<CreateUserRequest> for NewUser{
    fn from(create_user_request: CreateUserRequest) -> Self {
        NewUser{
            name: create_user_request.name,
            first_name: create_user_request.first_name,
            last_name: create_user_request.last_name,
            email: create_user_request.email,
            password: create_user_request.password
        }
    }
} 

#[debug_handler(state=AppState)]
pub async  fn create_mobile_user_handler(mut db_conn1: DBConnectionHolder, Json(payload): Json<CreateUserRequest>) ->Result<Json<User>, ErrorObject>{
    let user = compose!(create_mobile_user.provide(&mut db_conn1) -> with_args(payload)).await.map_err(map_to_error_object())?;
    //.await?;
    Ok(Json(user))
}

#[composeable()]
pub fn create_mobile_user(create_user_request:CreateUserRequest,_conn: &mut DBConnection)->BoxFuture<Result<User, FnError<ErrorType>>>{
    async{
        let value: &mut AsyncPgConnection = _conn.current_connection().await?;
        let mut user_repository = RepositoryDB::from(value);
        let user = user_repository.create_mobile_user(NewUser::from(create_user_request)).await?;
        Ok(user)
    }.boxed()
}

pub fn auth_request_from(user:String, pass: String) ->AuthRequest{
    AuthRequest{
        user: user,
        pass: pass
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest{
    user:String,
    pass:String
}


#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest{
    name:String,
    first_name:String,
    last_name: String,
    email: String,
    password: String,
}

