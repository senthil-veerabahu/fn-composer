use futures::{future::BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use function_compose::composeable;

use crate::{db::{DBConnProvider, DBConnection}, repository::{product_repository::{ProductCategoryData, ProductData, ProductRepository}, repository::RepositoryDB}};
use crate::fnutils::ErrorType;


#[composeable()]
pub fn find_product_by_ids(ids:Vec<Uuid>, dbConn: &mut DBConnection)->BoxFuture<Result<Vec<ProductData>, FnError<ErrorType>>>{
    async{
        let current_connection = dbConn.currentConnection().await?;
        let mut productRepository = RepositoryDB::from(current_connection);
        let result = productRepository.get_products_by_ids(ids).await?;
        Ok(result)
    }.boxed()
}


#[derive(Serialize, Deserialize)]
pub struct ProductDTO{
    id: String,
    name: String,
    generic: String,
    variant: String,
    imgUrl: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductListDTO{
    items: Vec<ProductDTO>
}

#[derive(Serialize, Deserialize)]
pub struct ProductFilterDTO{
    id: String,
    name: String,
    generic: String,
    variant: String,    
    categoryName: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFilterListDTO{
    productFilterDtoList: Vec<ProductFilterDTO>
}

#[composeable()]
pub  fn pack_product_data(product_list:Vec<ProductData>) -> Result<ProductListDTO, FnError<ErrorType>>{
    
    let products = product_list.into_iter().map(|w| {
        ProductDTO{
            id: w.0.id.to_string(),
            name: w.0.product_name.to_string(),
            generic: w.0.family.unwrap_or_default(),
            variant:w.0.variant.to_string(),
            imgUrl: w.1.image_one.unwrap(),
        }
    }).collect();

    Ok(ProductListDTO{
        items:products
    })
}

#[composeable()]
pub  fn pack_product_category_data(product_list:Vec<ProductCategoryData>) -> Result<ProductFilterListDTO, FnError<ErrorType>>{
    
    let products = product_list.into_iter().map(|w| {
        ProductFilterDTO{
            id: w.0.id.to_string(),
            name: w.0.product_name.to_string(),
            generic: w.0.family.unwrap_or_default(),
            variant:w.0.variant.to_string(),
            categoryName: w.1.catogery_name,
        }
    }).collect();

    Ok(ProductFilterListDTO{
        productFilterDtoList:products
    })
}