import asyncio

from mcp import Client
from mcp_server import mcp


claude_tools = []
async def main():
    async with Client(mcp) as client:
        print("Connected to the MCP server")

        tools_result = await client.list_tools()

        if len(tools_result.tools) == 0:
            print("No tools registered")
        else:
            print("Available tools")

        for tool in tools_result.tools:
            print(f"{tool.name}")
            claude_tools.append({
                "name" : tool.name,
                "description" : tool.description or "",
                "input_schema" : tool.input_schema
            })

        result = await client.call_tool(
            "get_weather", 
            {
                "city" : "Madrid"
            }
        )

        print("Result")
        print(result)
if __name__ == "__main__":
    asyncio.run(main())
            
            
