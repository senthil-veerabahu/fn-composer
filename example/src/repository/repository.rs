use axum::async_trait;
use diesel_async::AsyncPgConnection;

use crate::fnutils::{ErrorType, FnError};


pub struct RepositoryDB<'a>{
    pub connection: &'a mut AsyncPgConnection,
}

impl<'a> RepositoryDB<'a>{
    pub fn from(connection: &'a mut AsyncPgConnection)->Self{
        RepositoryDB{ connection: connection }
    }
}

#[async_trait]
pub trait CrudRepository<E, ID>{
    async fn find_by_id(id: ID) ->Result<E, FnError<ErrorType>>;
    async fn update(e:E)->Result<bool, FnError<ErrorType>>;
    async fn delete(e:E)->Result<bool, FnError<ErrorType>>;
    async fn create(e:E)->Result<E, FnError<ErrorType>>;
}
