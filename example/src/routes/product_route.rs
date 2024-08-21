use axum::Json;
use axum_macros::debug_handler;
use futures::FutureExt;
use serde::{Deserialize};
use uuid::Uuid;

use function_compose::*;
use crate::handlers::product_handler::*;


use crate::axumutils::{AppState, AuthUserData, DBConnectionHolder, Qs};
use crate::fnutils::ErrorObject;
use crate::handlers::product_handler::ProductListDTO;


#[derive(Deserialize)]
pub struct GetProductRequest {
    ids: Vec<Uuid>,
}


#[debug_handler(state=AppState)]
pub async fn get_product_by_ids(Qs(get_product_request_data): Qs<GetProductRequest>, mut dbConn1: DBConnectionHolder, _auth_user_data:AuthUserData) -> Result<Json<ProductListDTO>, ErrorObject> {
    let result:ProductListDTO = compose!(find_product_by_ids.provide(&mut dbConn1) -> pack_product_data -> with_args(get_product_request_data.ids)).await?;
    Ok(Json(result))
}


