// @generated automatically by Diesel CLI.

diesel::table! {
    weather (id) {
        id -> Int4,
        city_name -> Text,
        temperature -> Float8,
        description -> Text,
        humidity -> Nullable<Int4>,
        wind_speed -> Nullable<Float8>,
        sunrise -> Nullable<Int8>,
        sunset -> Nullable<Int8>,
    }
}
