-- Your SQL goes here


CREATE TABLE weather(
    id SERIAL PRIMARY KEY,
    city_name TEXT NOT NULL,
    temperature FLOAT NOT NULL,
    description TEXT NOT NULL,
    humidity INT,
    wind_speed FLOAT,
    sunrise BIGINT,
    sunset BIGINT
)