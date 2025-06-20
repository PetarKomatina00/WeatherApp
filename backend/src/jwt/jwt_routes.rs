use rocket::serde::json::Json;

use crate::models::{Claims, Profile};

use super::jwt_guards::{User, UserProfile};



#[get("/get/user/claim")]
pub fn get_user_claim(user: User) -> Json<Claims>{
    //println!("Hello, {}", user.0.sub);
    //println!("User role: {:?}", user.0.user_type);
    println!("Super duper");
    Json(user.0)
}

#[get("/whoami")]
pub async fn who_am_i(user_profile: UserProfile) -> Json<Profile> {
    //println!("Times called");
    Json(user_profile.0)
}
