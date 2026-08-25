
from typing import Any

from mcp import Client

class MCPClient:

    def __init__(self, server_url: str):
        self.server_url = server_url

    async def get_claude_tools(self) -> list[dict[str, Any]]:
        """
        Get available MCP tools and convert them
        to Claude-compatible tool definitions.
        """

        async with Client(self.server_url) as client:
            print("Connected to the MCP server")

            tools_result = await client.list_tools()

            if not tools_result.tools:
                print("No tools registered")
                return []

            print("Available tools")

            claude_tools = []

            for tool in tools_result.tools:
                print(tool.name)

                claude_tools.append({
                    "name": tool.name,
                    "description": tool.description or "",
                    "input_schema": tool.input_schema,
                })

            return claude_tools

    async def call_tool(self,tool_name: str,arguments: dict[str, str],):
        """
        Call a tool exposed by the MCP server.
        """

        async with Client(self.server_url) as client:
            result = await client.call_tool(tool_name,arguments)
            return result