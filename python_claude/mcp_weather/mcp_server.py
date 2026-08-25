import os;

import httpx;
from mcp.server import MCPServer
from dotenv import load_dotenv

load_dotenv()

mcp = MCPServer("mcp-weather")

OPENWEATHER_API_KEY=os.getenv("WEATHER_API_KEY", "Cannot get openweather api key")

if not OPENWEATHER_API_KEY:
    raise RuntimeError("Weather api key not found")

OPENWEATHER_URL = "http://api.openweathermap.org/data/2.5/weather"

@mcp.tool()
async def get_weather(city: str):
    """
    Get current weather information for a city.

    Use this tool when current weather informations are needed,
    for camping or other outdoor activities.

    """
    async with httpx.AsyncClient() as client:
        response = await client.get(
            OPENWEATHER_URL,
            params={
                "q":city,
                "appid" : OPENWEATHER_API_KEY,
                "units": "metric"
            }
        )

        response.raise_for_status()

        data = response.json()

        return {
        "city": data["name"],
        "temperature": data["main"]["temp"],
        "feels_like": data["main"]["feels_like"],
        "humidity": data["main"]["humidity"],
        "weather": data["weather"][0]["description"],
        "wind_speed": data["wind"]["speed"],
    }

if __name__ == "__main__":
    mcp.run(
        transport="streamable-http",
        host="127.0.0.1",
        port=8003
    )