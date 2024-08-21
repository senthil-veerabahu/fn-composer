use axum::{Json, Router};
use axum::routing::{get, post};
use axum_macros::debug_handler;
use diesel_async::AsyncPgConnection;
use futures::future::BoxFuture;
use futures::FutureExt;
use tower_http::cors::CorsLayer;

use example::axumutils::{AppDBConnectionPool, AppState, DBConnectionHolder};
use example::db::{DBConnection, DBConnProvider};
use example::fnutils::{ErrorObject, ErrorType, map_to_error_object};
use example::handlers::user::*;
use example::model::User;
use example::repository::repository::RepositoryDB;
use example::repository::user_repository::{NewUser, UserRepository};
use example::routes::product_route::get_product_by_ids;
use function_compose::{compose, composeable, FnError};

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

pub async  fn create_mobile_user_handler(mut dbConn1: DBConnectionHolder, Json(payload): Json<CreateUserRequest>)->Result<Json<User>, ErrorObject>{
    let user = compose!(create_mobile_user.provide(&mut dbConn1) -> with_args(payload)).await.map_err(map_to_error_object())?;
    //.await?;
    Ok(Json(user))
}



#[debug_handler(state=AppState)]
pub async fn user_auth_handler( mut dbConn1: DBConnectionHolder, Json(auth_request): Json<AuthRequest>)-> Result<Json<AuthResponse>, ErrorObject>{
        let r:AuthResponse = compose!(
            authenticate.provide(&mut dbConn1) -> 
            generate_token -> 
            pack_auth_result -> 
            with_args(auth_request)).await?;                
        Ok(Json(r))
}


#[tokio::main]
async fn main() {    
    let appState:AppState = createAppState().await;
    let app = Router::new()        
        .route("/user", post(create_mobile_user_handler))
        .route("/auth", post(user_auth_handler))
        .route("/products", get(get_product_by_ids))
        .layer(CorsLayer::permissive())
        .with_state(appState);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}




#[composeable()]
pub fn create_mobile_user(create_user_request:CreateUserRequest,_conn: &mut DBConnection)->BoxFuture<Result<User, FnError<ErrorType>>>{
    async{
        let value: &mut AsyncPgConnection = _conn.currentConnection().await?;
        let mut userRepository = RepositoryDB::from(value);
        let user = userRepository.create_mobile_user(NewUser::from(create_user_request)).await?;
        Ok(user)
    }.boxed()
}


