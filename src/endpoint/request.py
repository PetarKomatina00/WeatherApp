from fastapi import FastAPI
from pydantic import BaseModel
from src.prompts.camping import CAMPING_SYSTEM_PROMPT
from src.services.chat_service import ChatService
app = FastAPI()

SYSTEM_PROMPT = CAMPING_SYSTEM_PROMPT

## Every User has its own ChatHistory/Service
sessions: dict[str, ChatService] = {}

class AskRequest(BaseModel):
    conversation_id: str
    question: str
    temperature: float = 0.2
    system: str | None = None
class AskResponse(BaseModel):
    answer: str

@app.post("/ask-claude", response_model=AskResponse)
def ask_claude(request: AskRequest):
    if request.conversation_id not in sessions:
        sessions[request.conversation_id] = ChatService(SYSTEM_PROMPT)
    chat = sessions[request.conversation_id]

    answer = chat.ask(question=request.question, system=request.system, temperature=request.temperature)

    return AskResponse(answer=answer)