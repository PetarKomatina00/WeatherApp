use diesel::{ExpressionMethods, PgConnection};
use crate::models::{ApiLogs, NewApiLogs};
use crate::schema::api_logs;
use diesel::prelude::*;
pub struct ApiLogsRepository;

impl ApiLogsRepository{
    pub fn find_multiple_api_logs(connection: &mut PgConnection, limit: i64) -> QueryResult<Vec<ApiLogs>>{
        api_logs::table.limit(limit)
        .order(api_logs::trace_id.asc())
        .load::<ApiLogs>(connection)
    }
    pub fn create_api_log(connection: &mut PgConnection, new_api_log: NewApiLogs) -> QueryResult<ApiLogs>{
        diesel::insert_into(api_logs::table)
        .values(new_api_log)
        .get_result(connection)
    }
}