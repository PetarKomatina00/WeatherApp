import asyncio
import os

from mcp_weather.mcp_client import MCPClient


async def main():
    client = MCPClient(os.environ["MCP_SERVER_URL"])

    tools = await client.get_claude_tools()

    if len(tools) == 0:
        print("Tools list is empty")
        return
    
    print("TOOLS:")
    print(tools)

    result = await client.call_tool(
        "get_weather",
        {
            "city": "Madrid"
        }
    )
    if result.is_error:
        print("Could not call concrete tool")
        print(result)
        return

    print("RESULT:")
    print(result)


if __name__ == "__main__":
    asyncio.run(main())