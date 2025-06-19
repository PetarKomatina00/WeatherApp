// @generated automatically by Diesel CLI.

diesel::table! {
    api_logs (logs_id) {
        logs_id -> Uuid,
        trace_id -> Text,
        func_call -> Text,
        created_at -> Timestamp,
        status -> Text,
        location -> Nullable<Text>,
        error_message -> Nullable<Text>,
    }
}
