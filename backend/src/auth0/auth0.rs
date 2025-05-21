use std::env;

use reqwest::Error;
use rocket::{figment::Profile, http::{Cookie, CookieJar, SameSite, Status}, response::Redirect, serde::json::Json};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};

use crate::{models::UserInfo, rocket_routes::weather_route::fetch_auth0_userinfo};


pub struct Auth0;

#[get("/login")]
pub fn login(oauth2: OAuth2<Auth0>, jar: &CookieJar<'_>) -> Redirect{
    //println!("Login: {:?}", jar);
    let ouath = oauth2
        .get_redirect_extras(
            jar,
            &["openid", "profile", "email"],
            &[("prompt", "login")],         
            )
        .unwrap();
    println!("{:?}", ouath);
    ouath
}

#[get("/callback")]
pub fn callback(token: TokenResponse<Auth0>, jar: &CookieJar<'_>) -> Redirect {
    println!("Callback: {:?}", jar);
    jar.add_private(
        Cookie::build(("access_token", token.access_token().to_owned()))
            .path("/")
              .same_site(SameSite::None)
              .secure(true)
              .build());
    Redirect::to("/")
}

#[get("/userinfo")]
pub async fn get_user_info(jar: &CookieJar<'_>) -> Result<Json<UserInfo>, Status>{
    println!("UserInfo: {:?}", jar);
    let token = jar.get_private("access_token")
        .map(|c| c.value().to_owned())
        .ok_or(Status::Unauthorized);

    println!("Token: {:?}", token);
    let token = token.expect("SOmething went wrong with the token");
    println!("Token: {}", token);
    match fetch_auth0_userinfo(&token).await {
        Ok(profile) => Ok(Json(profile)),
        Err(_) => Err(Status::Unauthorized),
    }
}