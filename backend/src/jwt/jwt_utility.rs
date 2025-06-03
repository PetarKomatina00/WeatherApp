use std::{borrow::Cow, env};

use jsonwebtoken::decode_header;
use percent_encoding::percent_decode_str;
use rocket::{http::Status, request::{FromRequest, Outcome, Request}, serde::{self, json::Json}};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct TokenRequest<'a>{
    grant_type: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    audience: &'a str,
    code: &'a str
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse{
    access_token: String, 
    #[allow(dead_code)]
    token_type: String, 
    #[allow(dead_code)]
    expires_in: u64
}
#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct JsonWebKey{
    pub kty: String, 
    #[serde(rename = "use")]
    pub key_use: String, 
    pub n: String, 
    pub e: String, 
    pub kid: String,
    pub x5t: String,
    pub x5c: String, 
    pub alg: String
}

#[derive(Debug, Deserialize)]
pub struct Jwks{
    pub keys: Vec<JsonWebKey>
}

#[derive(Debug)]
pub struct MyError;


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Claims {
    sub: String,
    iss: String,
    aud: String,
    exp: usize,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile{
    pub name: String,
    pub sub: String,
    pub email: String, 
    pub family_name: String, 
    pub given_name: String, 
    pub email_verified: bool,

}
pub struct User(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User{
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let jar = req.cookies();
        println!("From request started again");
        // let token = match jar.get_private("access_token") {
        //     Some(x) => x.value().to_owned(),
        //     None => return Outcome::Error((Status::Unauthorized, ())),
        // };

        let code_from_auth0_callback = jar.get_private("access_token")
        .map(|c| c.value().to_string())
        .ok_or(Status::Unauthorized)
        .expect("Error getting token");
        
        println!("Code from auth0: {}", code_from_auth0_callback);
        let code = code_from_auth0_callback.as_str();

        let decoded = decode_one_time_callback_string(code);
        let access_token = get_access_token(&decoded).await.expect("Error");

        println!("Access Token from request: {}", access_token);
        //let access_token = get_access_token(code_from_auth0_callback).await;
        //println!("Token accessed");
        //println!("Token: {}", token);

        //let x = get_user_info(token).await;

        //println!("Body JSON{:?}:", x);
        Outcome::Success(User(Claims::default()))
        // match validate(&token).await{
        //     Ok(claims) => Outcome::Success(User(claims)),
        //     Err(_) => Outcome::Forward(Status::Unauthorized)
        // }
    }
}

#[get("/validate/<token>")]
pub async fn validate(token: &str){
    println!("Validation started");
    //println!("Token is: {}", token);

    let kid = decode_header(&token).expect("Could not decode header");
    println!("KID is: {:?}", kid);
    
    // let json_web_key: JsonWebKey = get_concrete_web_key(kid).await.expect("Failed to get concrete web key");
    // println!("JSON WEB KEY IS: {:?}", json_web_key);

    // let public_key = DecodingKey::from_rsa_components
    // (&json_web_key.n, &json_web_key.e)
    // .expect("Trouble extracting public key");
    // let mut validation = Validation::new(Algorithm::RS256);

    // let audience = env::var("AUDIENCE").expect("Cannot get auth0 audience");
    // let audience_url = format!("https://{}", audience);
    // validation.set_audience(&[audience_url]);

    // let auth0_domain = env::var("AUTH0_DOMAIN").expect("Cannot get auth0 DOMAIN");
    // let iss: HashSet<String> = HashSet::from([format!("https://{}", auth0_domain)]);
    // validation.iss = Some(iss);

    // let claims = decode::<Claims>(token, &public_key, &validation).expect("Error validating JWT").claims;
    
    // let claim = Claims::default();
}

// kid is what type of algorithm is used
// see the url link below of what the jsonwebkeys are
async fn get_concrete_web_key(kid: String) -> Option<JsonWebKey>{

    let tenant = env::var("AUTH0_DOMAIN").expect("Cannot get Tenant");
    let url = format!("https://{}/.well-known/jwks.json", tenant);

    let jwk: Jwks = reqwest::get(url).await.expect("Error getting JSON WEB TOKENS").json().await.unwrap();

    let mut json_web_key: JsonWebKey = Default::default();
    for key in jwk.keys{
        if kid == key.kid{
            json_web_key = key;
        }
    }
    if json_web_key == JsonWebKey::default(){
        None
    }
    else{
        Some(json_web_key)
    }
}

#[get("/private")]
pub fn get_user_claim(user: User){
    println!("Hello, {}", user.0.sub)
}

pub async fn get_access_token(code: &str) -> Result<String, String>{
    dotenv::dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("Cannot get client id");
    let client_secret = env::var("CLIENT_SECRET").expect("Cannot get client secret");
    let audience = env::var("AUDIENCE").expect("Cannot get audience");
    
    let client = reqwest::Client::builder().cookie_store(true)
        .build().expect("Could not create client to send HTTP");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/x-www-form-urlencoded".parse().expect("Could not parse http"));

    let body = TokenRequest {
        grant_type: "client_credentials",
        client_id: client_id.as_str(),
        client_secret: client_secret.as_str(),
        audience: audience.as_str(),
        code: &code
    };
    let request = client
    .request(reqwest::Method::POST, "https://dev-kr7vi67c2vo4vs3w.eu.auth0.com/oauth/token")
    .header("content-type", "application/x-www-form-urlencoded")
    .form(&body);
    let response = request.send().await.expect("Could not send HTTP");

    //println!("Response : {:?}", response);

    let token_response: TokenResponse = response.json().await.expect("Trouble converting Token Response");
    //println!("Access Token: {:?}", token_response.access_token);
    Ok(token_response.access_token)
}

#[get("/userinfo/<access_token>")]
pub async fn get_user_info(access_token: String) -> Json<UserProfile> {
    dotenv::dotenv().ok();
    let tenant = env::var("AUTH0_DOMAIN").expect("Cannot get Tenant");
    //let url: String = format!("https://{}/api/v2/users", tenant);
    let url: String = format!("https://{}/userinfo", tenant);

    //let access_token = get_access_token_private().await.expect("Something went wrong with the access token");

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("Accept", "application/json")
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Error with HTTP request");
    println!("Response: {:?}", response);

    let body: UserProfile = response.json::<UserProfile>().await.expect("Could not convert into UserProfile");

    println!("Body: {:?}", body);

    Json(body)
}
fn decode_one_time_callback_string(one_time_string: &str) -> Cow<'_, str>{

    let decode: std::borrow::Cow<'_, str> = percent_decode_str(one_time_string)
        .decode_utf8()
        .expect("Invalid");

    decode
}