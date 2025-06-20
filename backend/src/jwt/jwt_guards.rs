use rocket::{http::Status, request::{FromRequest, Outcome}};
use crate::models::{Claims, Profile};
use rocket::request::Request;
use super::jwt_utility::{get_user_info, validate_token};
pub struct User(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let jar = req.cookies();
        //println!("From request started again");

        if let Some(cookie) = jar.get_private("access_token") {
            let jwt_from_auth0_callback = cookie.value();
            //println!("JWT from auth0: {}", jwt_from_auth0_callback);
            match validate_token(&jwt_from_auth0_callback).await {
                Ok(claims) => {
                    println!("Woohoo");
                    req.local_cache(|| claims.sub.clone());
                    Outcome::Success(User(claims))
                }
                Err(_) => Outcome::Forward(Status::Unauthorized),
            }
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

pub struct UserProfile(pub Profile);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserProfile {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let jar = req.cookies();
        //println!("From request started again");
        //println!("UserProfile Guard");
        if let Some(cookie) = jar.get_private("access_token") {
            let jwt_from_auth0_callback = cookie.value();
            //println!("JWT from auth0: {}", jwt_from_auth0_callback);
            match get_user_info(&jwt_from_auth0_callback).await {
                Ok(profile) => Outcome::Success(UserProfile(profile)),
                Err(_) => Outcome::Forward(Status::Unauthorized),
            }
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}