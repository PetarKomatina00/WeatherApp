import asyncio
import json
import os

from anthropic import AsyncAnthropic

from src.claude_client import MCPClient


async def main():
    claude = AsyncAnthropic(
        api_key=os.environ["ANTHROPIC_API_KEY"]
    )

    mcp_client = MCPClient(
        os.environ["MCP_WEATHER_URL"]
    )

    # 1. Preuzmi toolove sa MCP servera
    tools = await mcp_client.get_claude_tools()

    print("AVAILABLE TOOLS:")
    print(tools)

    messages = [
        {
            "role": "user",
            "content": "Da li mogu da idem na kampovanje u Madrid?"
        }
    ]

    # 2. Prvi Claude poziv
    response = await claude.messages.create(
        model=os.environ["CLAUDE_MODEL"],
        max_tokens=1024,
        messages=messages,
        tools=tools,
    )

    print("\nFIRST CLAUDE RESPONSE:")
    print(response)

    tool_use = next(
        (
            block
            for block in response.content
            if block.type == "tool_use"
        ),
        None,
    )

    # Claude je odlučio da MCP nije potreban
    if tool_use is None:
        print("\nClaude did not use a tool.")

        for block in response.content:
            if block.type == "text":
                print(block.text)

        return

    print("\nTOOL SELECTED BY CLAUDE:")
    print("name:", tool_use.name)
    print("arguments:", tool_use.input)

    # 3. Izvrši ono što je Claude tražio
    result = await mcp_client.call_tool(
        tool_use.name,
        tool_use.input,
    )

    print("\nMCP RESULT:")
    print(result)

    # 4. Pretvori MCP rezultat u sadržaj pogodan za Claude
    if result.structured_content is not None:
        tool_result_content = json.dumps(
            result.structured_content,
            ensure_ascii=False,
        )
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
        model=os.environ["CLAUDE_MODEL"],
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