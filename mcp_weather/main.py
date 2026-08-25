
## This code is written only for testing purposes
## It shows a successfull connection has been made between MCP client, server and claude

# This now does not work because MCPClient is in another folder: python_claude
# This is used only for testing purposes in development.


# import asyncio
# from python_claude.src.mcp_client.mcp_client import MCPWeatherClient
# from mcp_weather.mcp_server import mcp_server

# async def main():
#     weather_mcp = MCPWeatherClient(mcp_server)

#     tools = await weather_mcp.get_claude_tools()

#     print("Cladue tools")
#     print(tools)

#     result = await weather_mcp.call_tool("get_weather", {
#         "city" : "Madrid"
#     })

#     print("Weather results")
#     print(result)

# if __name__ == "__main__":
#     asyncio.run(main())