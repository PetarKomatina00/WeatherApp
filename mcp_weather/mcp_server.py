import os;

import httpx;
from mcp.server import MCPServer
from dotenv import load_dotenv



class MCPWeatherServer:
    def __init__(self):
        load_dotenv()

        self.open_weather_api_key = os.getenv("WEATHER_API_KEY", "Cannot get openweather api key")

        if not self.open_weather_api_key:
            raise RuntimeError("Weather API Key not found")

        self.open_weather_url = "http://api.openweathermap.org/data/2.5/weather"

        self.mcp_server = MCPServer("mcp-weather")

        self._register_tools()

        
    def _register_tools(self):
        self.mcp_server.add_tool(self.get_weather)


    async def get_weather(self, city: str):
        """
        Get current weather information for a city.

        Use this tool when current weather informations are needed,
        for camping or other outdoor activities.

        """
        async with httpx.AsyncClient() as client:
            response = await client.get(
                self.open_weather_url,
                params={
                    "q":city,
                    "appid" : self.open_weather_api_key,
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

    def run (self, host: str = "0.0.0.0", port: int = 8003):
        self.mcp_server.run(transport="streamable-http", host=host, port=port)

if __name__ == "__main__":
    server = MCPWeatherServer()

    server.run()