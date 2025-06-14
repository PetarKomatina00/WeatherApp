use std::{borrow::Cow, collections::HashSet, env};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use percent_encoding::percent_decode_str;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    serde::{self, json::Json},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    audience: &'a str,
    redirect_uri: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    #[allow(dead_code)]
    token_type: String,
    #[allow(dead_code)]
    expires_in: u64,
    #[allow(dead_code)]
    id_token: String,
}
#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct JsonWebKey {
    pub kty: String,
    #[serde(rename = "use")]
    pub key_use: String,
    pub n: String,
    pub e: String,
    pub kid: String,
    pub x5t: String,
    pub x5c: Vec<String>,
    pub alg: String,
}

#[derive(Debug, Deserialize)]
pub struct VecJsonWebKey {
    #[serde(rename = "keys")]
    pub jwk_vec: Vec<JsonWebKey>,
}

#[derive(Debug)]
pub struct MyError;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Claims {
    sub: String,
    iss: String,
    aud: Vec<String>,
    exp: usize,
    iat: usize,
    scope: Option<String>,
    #[serde(rename = "https://rust-weather-api.com/roles")]
    user_type: Vec<String>,
    permissions: Option<Vec<String>>,
}

pub struct User(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let jar = req.cookies();
        println!("From request started again");

        if let Some(cookie) = jar.get_private("access_token") {
            let jwt_from_auth0_callback = cookie.value();
            println!("JWT from auth0: {}", jwt_from_auth0_callback);
            match validate_token(&jwt_from_auth0_callback).await {
                Ok(claims) => Outcome::Success(User(claims)),
                Err(_) => Outcome::Forward(Status::Unauthorized),
            }
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

pub async fn validate_token(access_token: &str) -> Result<Claims, String> {
    println!("Validation started");
    println!("Access token from validate_token: {}", access_token);
    //println!("Token is: {}", token);
    dotenv::dotenv().ok();
    let kid: String = decode_header(&access_token)
        .expect("Could not decode header")
        .kid
        .expect("Could not get KID");
    //println!("KID is: {}", kid);

    let json_web_key: JsonWebKey = get_concrete_web_key(kid)
        .await
        .expect("Failed to get concrete web key");
    //println!("JSON WEB KEY IS: {:?}", json_web_key);

    let public_key = DecodingKey::from_rsa_components(&json_web_key.n, &json_web_key.e)
        .expect("Trouble creating a RSA DecoingKey");
    let mut validation = Validation::new(Algorithm::RS256);

    let audience = env::var("AUDIENCE").expect("Cannot get auth0 audience");
    let audience_url = format!("https://{}", audience);
    println!("audience: {}", audience_url);
    validation.set_audience(&[audience]);

    let auth0_domain = env::var("AUTH0_DOMAIN").expect("Cannot get auth0 DOMAIN");
    let iss: HashSet<String> = HashSet::from([format!("https://{}/", auth0_domain)]);
    println!("audience: {:?}", iss);
    validation.iss = Some(iss);
    println!("Validaton: {:?}", validation);
    let claims: Claims = decode::<Claims>(access_token, &public_key, &validation)
        .expect("Error validating JWT")
        .claims;

    println!("CLaims {:?}", claims);
    // TODO()
    // Give the concrete custom claims inside the payload.
    Ok(claims)
}

// KID represents what public keys are used during encryption.
// AUTH0 has a well known JSON set of (n,e)
// See the url link below of what the jsonwebkeys are
async fn get_concrete_web_key(kid: String) -> Option<JsonWebKey> {
    dotenv::dotenv().ok();
    let tenant = env::var("AUTH0_DOMAIN").expect("Cannot get Tenant");
    println!("Tenant: {}", tenant);
    let url = format!("https://{}/.well-known/jwks.json", tenant);

    let jwks_vec: VecJsonWebKey = reqwest::get(url)
        .await
        .expect("Error getting JSON WEB TOKENS")
        .json::<VecJsonWebKey>()
        .await
        .expect("Could Convert Well-Known JWKS");

    for jwk in jwks_vec.jwk_vec {
        if jwk.kid == kid {
            return Some(jwk);
        }
    }
    None
}

#[get("/private")]
pub fn get_user_claim(user: User) {
    println!("Hello, {}", user.0.sub);
    println!("User role: {:?}", user.0.user_type);
}

pub async fn get_access_token(decoded_jwe: &str) -> Result<String, String> {
    dotenv::dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("Cannot get client id");
    let client_secret = env::var("CLIENT_SECRET").expect("Cannot get client secret");
    let audience = env::var("AUDIENCE").expect("Cannot get audience");

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Could not create client to send HTTP");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        "application/x-www-form-urlencoded"
            .parse()
            .expect("Could not parse http"),
    );

    let redirect_uri = format!("http://127.0.0.1:8000/auth0/callback");
    let body = TokenRequest {
        grant_type: "client_credentials",
        client_id: client_id.as_str(),
        client_secret: client_secret.as_str(),
        audience: audience.as_str(),
        redirect_uri: redirect_uri.as_str(),
    };
    println!("Code: {}", decoded_jwe);
    let request = client
        .request(
            reqwest::Method::POST,
            "https://dev-kr7vi67c2vo4vs3w.eu.auth0.com/oauth/token",
        )
        .header("content-type", "application/x-www-form-urlencoded")
        .form(&body);
    let response = request.send().await.expect("Could not send HTTP");

    println!("Response  Status: {:?}", &response.status());
    println!("Body: {:?}", &response.text().await.unwrap());

    //let token_response: TokenResponse = response.json().await.expect("Trouble converting Token Response");
    //println!("Access Token: {:?}", token_response.access_token);
    Ok(String::from("a"))
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Profile {
    pub name: String,
    pub sub: String,
    pub email: String,
    pub family_name: String,
    pub given_name: String,
    pub email_verified: bool,
}

pub struct UserProfile(Profile);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserProfile {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let jar = req.cookies();
        //println!("From request started again");
        println!("UserProfile Guard");
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
pub async fn get_user_info(access_token: &str) -> Result<Profile, String> {
    dotenv::dotenv().ok();
    let tenant = env::var("AUTH0_DOMAIN").expect("Cannot get Tenant");
    //let url: String = format!("https://{}/api/v2/users", tenant);
    let url: String = format!("https://{}/userinfo", tenant);

    //let access_token = get_access_token_private().await.expect("Something went wrong with the access token");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Error");
    //println!("Response: {:?}", response);

    match response.status().is_success() {
        true => {
            let body: Profile = response
            .json::<Profile>()
            .await
            .expect("Could not convert into UserProfile");
            Ok(body)
        }
        false => {
            Err(String::from("Something went wrong with the get user info"))
        }
    }
        
}

#[get("/whoami")]
pub async fn who_am_i(user_profile: UserProfile) -> Json<Profile> {
    println!("Times called");
    Json(user_profile.0)
}

fn decode_one_time_callback_string(one_time_string: &str) -> Cow<'_, str> {
    let decode: std::borrow::Cow<'_, str> = percent_decode_str(one_time_string)
        .decode_utf8()
        .expect("Invalid");

    decode
}
