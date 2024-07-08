use axum::async_trait;
use diesel::{associations::HasTable, ExpressionMethods, NullableExpressionMethods, QueryDsl, result::Error, SelectableHelper};
use diesel::JoinOnDsl;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use function_compose::FnError;

use crate::{schema::product_attributes::dsl::*, schema::products::dsl::*};
use crate::fnutils::{ErrorType, map_result_not_found_error};
use crate::model::{Category, Product, ProductAttribute};

use super::repository::RepositoryDB;

pub type ProductData = (Product, ProductAttribute);

pub type ProductCategoryData = (Product, Category);
#[async_trait]
pub trait ProductRepository{
    async fn get_products_by_ids(&mut self, ids:Vec<Uuid>)->Result<Vec<ProductData>, FnError<ErrorType>>;    
}

#[async_trait]
impl<'a> ProductRepository for RepositoryDB<'a>{ 
    async fn get_products_by_ids(&mut self, ids:Vec<Uuid>)->Result<Vec<ProductData>, FnError<ErrorType>>{
        let product_result:Result<Vec<ProductData>, Error> = products.filter(product_uuid.eq_any(ids.clone()))
            .inner_join(product_attributes::table().on(product_id.eq(crate::schema::products::dsl::id.nullable())))
            .select((Product::as_select(), ProductAttribute::as_select()))
            .load(&mut self.connection).await;
        let product_not_found_error = format!("One or more Product ids not found {:?}", ids);
        product_result.map_err(map_result_not_found_error(&ErrorType::EntityNotFound(product_not_found_error)))
    }

    
}