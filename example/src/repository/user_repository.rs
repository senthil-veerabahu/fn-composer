use std::env;


use axum::async_trait;

use bcrypt::verify;
use bcrypt::hash;

use diesel::ExpressionMethods;

use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel::insert_into;
use diesel::prelude::Insertable;
use diesel::result::Error;
use diesel::sql_function;
use diesel::sql_types::Text;
use diesel_async::RunQueryDsl;
use diesel::JoinOnDsl;

use function_compose::{FnError};
use serde::Deserialize;
use uuid::Uuid;

use crate::model::*;


use crate::schema::role_entities;
use crate::schema::roles;
use crate::schema::role_entities::logger_id;

use crate::schema::users::{email, user_id};
use crate::schema::roles::columns as RoleTable;


use crate::{ schema::users::dsl::*, schema::roles::dsl::*, schema::role_entities::dsl::*};
use crate::fnutils::{convert_to_fn_error, ErrorMapper, ErrorType, map_result_not_found_error, map_to_unknown_bcrypt_error, map_to_unknown_db_error, map_to_unknown_var_error};



use super::repository::RepositoryDB;
sql_function!(fn lower(x: Text) -> Text);

#[async_trait]
pub trait UserRepository/* :CrudRepository<User,i64> */{
     async fn auth(&mut self, userName: String, pass: String)->Result<AuthData, FnError<ErrorType>>;

     async fn create_mobile_user(&mut self, user:NewUser)->Result<User, FnError<ErrorType>>;

    //async fn get_user_role_entities(&mut self, user_id:Uuid)->Result<Vec<RoleEntity>, AppError>;
}


pub type AuthData = (User, RoleEntity, Role);

#[async_trait]
impl<'a> UserRepository for RepositoryDB<'a>{    

    async fn auth(&mut self, userName: String, pass: String) -> Result<AuthData, FnError<ErrorType>> {
        println!("request user name is {}", userName);
        let userResult: Result<Vec<AuthData>, Error> = users.filter(email.eq(userName.to_lowercase()))
        .inner_join(role_entities::table.on(user_id.eq(logger_id)))
        .inner_join(roles::table.on(role_entities::role_id.eq(roles::id)))
        .select((User::as_select(), RoleEntity::as_select(), Role::as_select()))
        .load(&mut self.connection).await;
        let var_name = || ErrorType::UserNotFound(userName.clone());
        let result = userResult.map_err(|e| convert_to_fn_error(ErrorMapper::new().add(Error::NotFound, var_name), e));
        let user = result?;
        let valid = verify(pass, (&user[0]).0.password.as_ref()).map_err(map_to_unknown_bcrypt_error())?;
        if !valid || user.is_empty() {
            return Err(ErrorType::AuthError(userName).into());
        }
        Ok(user.into_iter().next().unwrap())
    }

    async fn create_mobile_user(&mut self, mut user: NewUser) -> Result<User, FnError<ErrorType>> {
        let role: Role = roles.filter(RoleTable::name.eq("customer"))
            .select(Role::as_select())
            .first(&mut self.connection).await.map_err(map_result_not_found_error(&ErrorType::RoleNotFound("customer".to_owned())))?;
        let user_count:i64 = users.filter(email.eq(user.email.clone())).count()
            .get_result(&mut self.connection).await.map_err(map_to_unknown_db_error())?;
        
        if user_count >  0 
        { 
            Err(ErrorType::EmailAlreadyTaken(user.email).into())
        }else { 
            let bcrypt_cost_string:String = env::var("BCRYPT_COST").map_err(map_to_unknown_var_error())?;
            let bcrypt_cost = bcrypt_cost_string.parse::<u32>().unwrap();
            user.password = hash(user.password, bcrypt_cost).map_err(map_to_unknown_bcrypt_error())?;
            let user:User = insert_into(users)
                .values(user)
                .get_result(self.connection).await.map_err(map_to_unknown_db_error())?;

            let new_role_entity = NewRoleEntity{
                    logger_id:Some(user.user_id),
                    role_id:Some(role.id),
                    entity_id: None,
                    entity_name:None,
            };

            let role_entityCount = insert_into(role_entities)
                .values(new_role_entity)
                .execute(&mut self.connection).await.map_err(map_result_not_found_error(&ErrorType::RoleNotFound(role.id.to_string())))?;
            
            assert_eq!(1, role_entityCount);
            println!("Insert data into role entities {}", role_entityCount);
            Ok(user)
        }
    }
}




#[derive( Debug, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::users)]
#[derive( Deserialize)]
pub struct NewUser{
    pub name:String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password:String,
}

#[derive( Debug, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::role_entities)]
#[derive( Deserialize)]
pub struct NewRoleEntity {
    pub entity_id: Option<String>,
    pub entity_name: Option<String>,
    pub role_id: Option<i64>,
    pub logger_id: Option<Uuid>,
}

