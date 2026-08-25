import asyncio
import json
import os

from anthropic import AsyncAnthropic

from mcp_weather.mcp_client import MCPClient


async def main():
    claude = AsyncAnthropic(api_key=os.environ["ANTHROPIC_API_KEY"])

    mcp_client = MCPClient(os.environ["MCP_SERVER_URL"])

    tools = await mcp_client.get_claude_tools()

    if len(tools) == 0:
        print("No available tools")
        return
    
    print("AVAILABLE TOOLS:")
    print(tools)

    messages = [
        {
            "role": "user",
            "content": "Da li mogu da idem na kampovanje u Madrid?"
        }
    ]

    # First we need to call claude
    response = await claude.messages.create(
        model=os.environ["ANTHROPIC_MODEL"],
        max_tokens=1024,
        messages=messages,
        tools=tools,
    )

    print("\nFIRST CLAUDE RESPONSE:")
    print(response)

    tool_use = None

    for block in response.content:
        if block.type == "tool_use":
            tool_use = block
            break

    # Get the tool use block
    if tool_use is None:
        print("\nClaude did not use a tool.")

        for block in response.content:
            if block.type == "text":
                print(block.text)
        return

    print("\nTOOL SELECTED BY CLAUDE:")
    print("name:", tool_use.name)
    print("arguments:", tool_use.input)

    # Call tool
    result = await mcp_client.call_tool(
        tool_use.name,
        tool_use.input,
    )

    print("\nMCP RESULT:")
    print(result)

    # 4. Pretvori MCP rezultat u sadržaj pogodan za Claude
    if result.structured_content is not None:
        tool_result_content = json.dumps(result.structured_content)
    else:
        tool_result_content = "\n".join(
            block.text
            for block in result.content
            if hasattr(block, "text")
        )

    # 5. Sačuvaj Claude-ov tool_use
    messages.append({
        "role": "assistant",
        "content": response.content,
    })

    # 6. Vrati rezultat tool-a Claude-u
    messages.append({
        "role": "user",
        "content": [
            {
                "type": "tool_result",
                "tool_use_id": tool_use.id,
                "content": tool_result_content,
            }
        ],
    })

    # 7. Claude sada pravi finalni odgovor
    final_response = await claude.messages.create(
        model=os.environ["ANTHROPIC_MODEL"],
        max_tokens=1024,
        messages=messages,
        tools=tools,
    )

    print("\nFINAL CLAUDE RESPONSE:")

    for block in final_response.content:
        if block.type == "text":
            print(block.text)


if __name__ == "__main__":
    asyncio.run(main())