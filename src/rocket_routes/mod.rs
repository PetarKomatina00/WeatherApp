use rocket::get;


#[get("/")]
pub fn get() -> (){
    println!("Hello from the weather app");
}