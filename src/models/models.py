from fastapi import FastAPI
from pydantic import BaseModel
class Main(BaseModel):
    humidity: int
    temp: float
    feels_like: float
    temp_min: float
    temp_max: float
class Sys(BaseModel): 
    sunrise: int
    sunset: int

class Coords(BaseModel):
    lon: float
    lat: float

class WeatherData(BaseModel):
    id: int
    name: str
    main: Main
    sys: Sys
    timezone: int
    coord: Coords
    cod: int

class ClaudeRequest(BaseModel):
    conversation_id: str
    question: str
    temperature: float = 0.2
    system: str | None = None
    weather_data : WeatherData | None = None
    use_mcp_weather: bool
class ClaudeResponse(BaseModel):
    answer: str