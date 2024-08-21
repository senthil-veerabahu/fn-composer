
use hmac::{Hmac, Mac};
use jwt::Header;
use jwt::Token;
use jwt::VerifyWithKey;
use jwt::{Claims, RegisteredClaims, SignWithKey};
use sha2::Sha256;


use std::{collections::BTreeMap};
use std::time::{Duration};
use serde_json::Value;
use function_compose::FnError;
use crate::fnutils::{ErrorType, FnResult,  map_hmac_invalid_length_to_unknown_error, map_jwt_error_to_unknown_error};



/*

claims := &DtoModel.SignedDetails{

		Role:           role,
		Userid:         uid,
		Email:          email,
		IsemailVerfied: isemailVerfied,
		IsPhoneVerfied: isphoneverfied,
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: time.Now().Local().AddDate(0, 2, 0).Unix(),
		},
	}

*/

pub fn verify_token(jwt_token: &str, jwt_signing_key: String)->Result<BTreeMap<String, String>, FnError<ErrorType>>{
    let jwt_signing_key_and_algo:Hmac<Sha256> = Hmac::new_from_slice(jwt_signing_key.as_bytes()).map_err(map_hmac_invalid_length_to_unknown_error())?;    
    let token: Token<Header, BTreeMap<String, String>, _> = VerifyWithKey::verify_with_key(jwt_token, &jwt_signing_key_and_algo).unwrap();
    let claims = token.claims().clone();
    Ok(claims)
}

pub fn generate_jwt_token(name: &String, email: &String, jwt_signing_key: String, 
     roles:Vec<&str>, 
     is_email_verified: bool,
     is_phone_verified: bool, expires_in:Duration)->FnResult<String, ErrorType>{
    //let jwt_signing_key = env::var("BCRYPT_COST")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_signing_key.as_bytes()).map_err(map_hmac_invalid_length_to_unknown_error())?;
    let mut claims: BTreeMap<String, Value> = BTreeMap::new();    
    claims.insert("userId".to_owned(), Value::String(name.clone()));
    claims.insert("email".to_owned(), Value::String(email.clone()));
    claims.insert("jti".to_owned(), Value::String(email.clone()));
    claims.insert("role".to_owned(), Value::String(roles.join(" ")));
    claims.insert("isEmailVerfied".to_owned(), Value::String(is_email_verified.to_string()));
    claims.insert("isPhoneVerfied".to_owned(), Value::String(is_phone_verified.to_string()));
    claims.insert("exp".to_owned(), Value::String(expires_in.as_secs().to_string()));
    
    
    let registered_claims:RegisteredClaims = Default::default();
    //registered_claims.expiration = Some(expires_in.as_secs());
    let all_claims = Claims{
        private: claims,
        registered: registered_claims
    };
    let token = all_claims.sign_with_key(&key).map_err(map_jwt_error_to_unknown_error())?;
    Ok(token)
}





/* Role:           role,
		Userid:         uid,
		Email:          email,
		IsemailVerfied: isemailVerfied,
		IsPhoneVerfied: isphoneverfied,
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: time.Now().Local().AddDate(0, 2, 0).Unix(), */