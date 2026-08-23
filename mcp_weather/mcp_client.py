import asyncio

from mcp import Client
from mcp_weather.mcp_server import mcp_server

#Testing
from python_claude.src.config import ANTHROPIC_MODEL
from python_claude.src.prompts.camping import CAMPING_SYSTEM_PROMPT
from anthropic import Anthropic
from python_claude.src.config import ANTHROPIC_API_KEY


class MCPWeatherClient:

    def __init__(self, server):
        self.server = server

    async def get_claude_tools(self) -> list[dict]:
        """
        Get MCP tools
        """
        async with Client(mcp_server) as mcp_client:
            print("Connected to the MCP server")

            tools_result = await mcp_client.list_tools()

            if len(tools_result.tools) == 0:
                print("No tools registered")
                return
            else:
                print("Available tools")

            claude_tools = []
            for tool in tools_result.tools:
                print(f"{tool.name}")
                claude_tools.append({
                    "name" : tool.name,
                    "description" : tool.description or "",
                    "input_schema" : tool.input_schema
                })

            return claude_tools


    async def call_tool(self, tool_name, arguments: dict[str, str]):
        """
        Execute MCP tool
        """

        async with Client(self.server) as client:
            result = await client.call_tool(tool_name, arguments)

            return result

