use std::env;

use rocket::{
    futures::FutureExt, http::{Cookie, CookieJar, SameSite, Status}, response::Redirect, serde::json::Json
};
use rocket_oauth2::{OAuth2, TokenResponse};

use crate::jwt::jwt_utility::User;

pub struct Auth0;

#[get("/login")]
pub fn login(oauth2: OAuth2<Auth0>, jar: &CookieJar<'_>) -> Redirect {
    //println!("Login: {:?}", jar);
    dotenv::dotenv().ok();
    //("prompt", "login"),
    let audience = env::var("AUDIENCE").expect("Could not get Audience");
    let ouath = oauth2
        .get_redirect_extras(
            jar,
            &["openid", "profile", "email"],
            &[
                ("prompt", "login"),
                ("screen_hint", "signup"),
                ("audience", &audience),
            ],
        )
        .unwrap();
    //println!("{:?}", ouath);
    ouath
}

#[get("/callback?<code>&<state>")]
pub fn callback(
    code: String,
    state: String,
    token: TokenResponse<Auth0>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, String> {
    //println!("Callback: {:?}", jar);
    println!("Code from callback: {}", code);
    println!("Token from callback: {}", token.access_token());
    jar.add_private(
        Cookie::build(("access_token", token.access_token().to_owned()))
            .path("/")
            .secure(true)
            .same_site(SameSite::None)
            .build(),
    );
    //println!("Callback222: {:?}", jar);
    Ok(Redirect::to("http://127.0.0.1:8001"))
}
#[get("/api/token")]
pub fn api_token(jar: &CookieJar<'_>) -> Result<Json<String>, Status> {
    //println!("{:?}", jar);
    let token = jar
        .get_private("access_token")
        .map(|c| c.value().to_string())
        .ok_or(Status::Unauthorized)?;

    println!("Token: {:?}", token);

    //let decoded = decode_only(&token).expect("Failed to decode JWT");

    //println!("Decoded: {:?}", decoded);
    Ok(Json(token))
}


#[get("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Redirect{
    dotenv::dotenv().ok();

    println!("Removing cookie");
    jar.remove_private(Cookie::from("access_token"));
    println!("Cookie removed");
    let auth0_domain = env::var("AUTH0_DOMAIN").expect("Cannot get auth0 domain");
    let client_id = env::var("CLIENT_ID").expect("Cannot get CLIENT ID");
    let return_to = format!("http://127.0.0.1:8001");
    let url = format!("https://{}/v2/logout?client_id={}&returnTo={}", 
    auth0_domain, client_id, return_to);

    Redirect::to(url)
}