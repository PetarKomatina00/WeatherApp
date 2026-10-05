use std::env;

use rocket::{http::{Cookie, CookieJar, SameSite, Status}, response::Redirect, serde::json::Json};
use rocket_oauth2::{OAuth2, TokenResponse};
pub struct Auth0;

#[get("/login")]
pub fn login(oauth2: OAuth2<Auth0>, jar: &CookieJar<'_>) -> Redirect {
    //println!("Login: {:?}", jar);
    println!("Logged in");
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
    println!("Callback: {:?}", jar);
    println!("Code from callback: {}", code);
    println!("Token from callback: {}", token.access_token());
    println!("=== CALLBACK ENTERED ===");

    jar.add_private(
        Cookie::build(("access_token", token.access_token().to_owned()))
            .path("/")
            .secure(false)
            .same_site(SameSite::Lax)
            .build(),
    );
    jar.add(
        Cookie::build(("test_cookie", "hello"))
            .path("/")
            .secure(false)
            .same_site(SameSite::Lax)
            .build(),
    );
    println!("=== ACCESS TOKEN COOKIE ADDED ===");
    //println!("Callback222: {:?}", jar);

    let frontend_kuber_url = env::var("FRONTEND_KUBER_URL").expect("Could not get frontend url");
    let frontend_dev_url = env::var("FRONTEND_URL").expect("Could not get frontend dev url");
    Ok(Redirect::to(frontend_dev_url))
}
#[get("/api/token")]
pub fn api_token(jar: &CookieJar<'_>) -> Result<Json<String>, Status> {
    //println!("{:?}", jar);
    let token = jar
        .get_private("access_token")
        .map(|c| c.value().to_string())
        .ok_or(Status::Unauthorized)?;

    //println!("Token: {:?}", token);

    //let decoded = decode_only(&token).expect("Failed to decode JWT");

    //println!("Decoded: {:?}", decoded);
    Ok(Json(token))
}


#[get("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Redirect{
    dotenv::dotenv().ok();

    //println!("Removing cookie");
    jar.remove_private(Cookie::from("access_token"));
    //println!("Cookie removed");
    let auth0_domain = env::var("AUTH0_DOMAIN").expect("Cannot get auth0 domain");
    let client_id = env::var("CLIENT_ID").expect("Cannot get CLIENT ID");
    dotenv::dotenv().ok();
    let FRONTEND_KUBER_URL = env::var("FRONTEND_KUBER_URL").expect("Could not get frontend url");
    let frontend_dev_url = env::var("FRONTEND_URL").expect("Could not get frontend dev url");
    let return_to = format!("{}", frontend_dev_url);
    let url = format!("https://{}/v2/logout?client_id={}&returnTo={}", 
    auth0_domain, client_id, return_to);

    Redirect::to(url)
}