
use rocket::{http::{Cookie, CookieJar, SameSite, Status}, response::Redirect, serde::json::Json};
use rocket_oauth2::{OAuth2, TokenResponse};


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
    //println!("{:?}", ouath);
    ouath
}

#[get("/callback")]
pub fn callback(token: TokenResponse<Auth0>, jar: &CookieJar<'_>) -> Redirect {
    //println!("Callback: {:?}", jar);
    jar.add_private(
        Cookie::build(("access_token", token.access_token().to_owned()))
            .path("/")
              .same_site(SameSite::None)
              .build());
    //println!("Callback222: {:?}", jar);
    Redirect::to("http://127.0.0.1:8001")
}
#[get("/api/token")]
pub fn api_token(jar: &CookieJar<'_>) -> Result<Json<String>, Status>{
    //println!("{:?}", jar);
    let token = jar.get_private("access_token")
    .map(|c| c.value().to_string())
    .ok_or(Status::Unauthorized)?;

    println!("Token: {:?}", token);

    //let decoded = decode_only(&token).expect("Failed to decode JWT");

    
    //println!("Decoded: {:?}", decoded);
    Ok(Json(token))
}