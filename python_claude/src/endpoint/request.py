from fastapi import FastAPI
from src.models.models import ClaudeRequest, ClaudeResponse
from src.prompts.camping import CAMPING_SYSTEM_PROMPT
from src.services.chat_service import ChatService
app = FastAPI()

SYSTEM_PROMPT = CAMPING_SYSTEM_PROMPT

## Every User has its own ChatHistory/Service
sessions: dict[str, ChatService] = {}


@app.post("/ask-claude", response_model=ClaudeResponse)
def ask_claude(request: ClaudeRequest):

    print("Request je dobijen", request, flush=True)
    print(request)
    if request.conversation_id not in sessions:
        sessions[request.conversation_id] = ChatService(SYSTEM_PROMPT)
    chat = sessions[request.conversation_id]

    answer = chat.ask(question=request.question, use_mcp_weather=request.use_mcp_weather, weather_data=request.weather_data, system=request.system, temperature=request.temperature)

    return ClaudeResponse(answer=answer)