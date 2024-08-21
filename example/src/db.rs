use std::env;


use axum::async_trait;
use dotenv::dotenv;

use diesel_async::{AsyncPgConnection, pooled_connection::{deadpool::{Pool, Object}, AsyncDieselConnectionManager}};
use function_compose::FnError;
use crate::fnutils::{ErrorType, ErrorTypeInfo};



pub struct DBConnection{
    pub c_pool: Pool<diesel_async::AsyncPgConnection>,
    pub connection: Option<Object<AsyncPgConnection>>,
}

#[async_trait]
pub trait DBConnProvider{
    async fn currentConnection<'a>(&'a mut self)->Result<&mut AsyncPgConnection, FnError<ErrorType>>;
}

impl DBConnection{

    pub fn new(pool: Pool<AsyncPgConnection>)->DBConnection {
        DBConnection { c_pool: pool, connection: None }
    }
    async fn getConnection(&mut self) -> Result<&mut AsyncPgConnection, FnError<ErrorType>>{
        if self.connection.is_none() {
            let result = self.c_pool.get().await;
            match result {
                Ok(conn) => {                    
                    self.connection = Some(conn);
                    let  a = &mut self.connection;
                    let conRef = a.as_deref_mut().unwrap();
                    Ok(conRef)
                }
                Err(error) => {                    
                    let from = ErrorTypeInfo::new((ErrorType::DBError(error.to_string()), error.to_string())).into();
                    Err(from)
                }
            }
        } else {
            let c =self.connection.as_deref_mut().unwrap();
            Ok(c)
        }
    }
}

#[async_trait]
impl DBConnProvider for DBConnection {    
    async fn currentConnection<'a>(&'a mut self)->Result<&mut AsyncPgConnection, FnError<ErrorType>>{
        let connection = self.getConnection().await?;
        Ok(connection)
    }
}

pub async fn createConnectionPool() -> Result<Pool<AsyncPgConnection>, FnError<ErrorType>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");    
    let config: AsyncDieselConnectionManager<_> = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let result = Pool::builder(config).build();    
     match result{
        Ok(r) => Ok(r),
        Err(e) => {
            let from = ErrorTypeInfo::new((ErrorType::DBError(e.to_string()), e.to_string())).into();
            Err(from)
            
        }
    }
}