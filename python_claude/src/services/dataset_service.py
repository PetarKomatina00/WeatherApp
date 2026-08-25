import json
from services.chat_service import ChatService
from prompts.dataset import DATASET_SYSTEM_PROMPT, DATASET_GENERATION_PROMPT




class DatasetService:

    def __init__(self, chatService: ChatService):
        self.chat = chatService
        pass
    def generate_dataset(self):
        answer = self.chat.ask(DATASET_GENERATION_PROMPT)
        return json.loads(answer)