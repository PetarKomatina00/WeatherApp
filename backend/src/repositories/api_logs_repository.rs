use std::borrow::Cow;
use std::env;

use diesel::result::Error;
use diesel::{ExpressionMethods, PgConnection};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::uri::Query;
use rocket::Response;
use uuid::{uuid, Uuid};
use crate::models::{ApiLogs, NewApiLogs};
use crate::schema::api_logs;
use diesel::prelude::*;
use rocket::request::Request;
pub struct ApiLogsRepository;

impl ApiLogsRepository{
    pub fn find_multiple_api_logs(connection: &mut PgConnection, limit: i64) -> QueryResult<Vec<ApiLogs>>{
        api_logs::table.limit(limit)
        .order(api_logs::trace_id.asc())
        .load::<ApiLogs>(connection)
    }
    pub fn create_api_log(connection: &mut PgConnection, new_api_log: NewApiLogs) -> QueryResult<ApiLogs>{
        
        println!("NewApiLogs: {:?}", new_api_log);
        let result: Result<ApiLogs, Error> = diesel::insert_into(api_logs::table)
        .values(new_api_log)
        .get_result(connection);

        match result {
            Ok(res) => {Ok(res)}
            Err(e) => {Err(e)}
        }
    }
}


pub struct ApiLogger;
#[rocket::async_trait]
impl Fairing for ApiLogger{
    fn info(&self) -> Info{
        Info {
            name : "API Logs",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("API logged started");
        let method = request.method().to_string();
        let path = request.uri().to_string();
        let status = response.status().code.to_string();
        
        //Not ideal, but the only endpoint is /fetch/<city>
        //To extract the city i need to use fragments of a path


        let mut location: Option<String> = Some(String::from("/"));
        println!("Gledamo /fetch");
        if request.uri().path().starts_with("/fetch/"){
            println!("Ok usli smo");
            if let Some(city) = request.uri().path().segments().nth(1){
                println!("ide gas");
                location = Some(city.to_string());
            }
            else{
                return;
            }
        }
        // let origin_path = request.uri().path();
        // let origin_query: Option<rocket::http::uri::Query<'_>> = request.uri().query();

        // println!("origin query: {:?}", origin_query);
        // let query = match origin_query {
        //     Some(oq) => oq,
        //     None => {return;}
        // };

        //println!("{} {}", origin_path, query);
        let function_name = request.route()
            .map(|r: &rocket::Route| r.name.as_ref().unwrap()).unwrap_or(&Cow::from("unknown")).to_string();

        let user_id: String = request
            .local_cache(|| String::from("")).into();

        if user_id.is_empty(){
            println!("user_id is empty");
            return;
        }
        println!("{} {} {} {} {}", method, path, status, function_name, user_id);

        

        dotenv::dotenv().ok();

        let connection = env::var("DATABASE_URL").expect("Cannot get postgres url");
        let mut pgconn = PgConnection::establish(&connection).expect("Error");
        let api_log = ApiLogsRepository::create_api_log(&mut pgconn, NewApiLogs{
            trace_id: user_id, 
            func_call: function_name,
            status,
            location,
            error_message: None
        });
        match api_log{
            Ok(_) => println!("API LOG created!"),
            Err(e) => {println!("API LOG created! {}", e);}
        };
    }

}