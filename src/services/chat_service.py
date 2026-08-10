from claude_client import client
from config import ANTHROPIC_MODEL

class ChatService:
    def __init__(self, system_prompt: str):
        self.system = system_prompt
        self.message_history = []

    def add_message(self, role: str, text: str) -> None:
        self.message_history.append({
            "role": role,
            "content": text,
        })

    def ask(self, question: str, system=None, temperature = 0.2) -> str:

        params = {
            "model" : ANTHROPIC_MODEL,
            "max_tokens" : 1024,
            "messages" : self.message_history,
            "temperature" : temperature,
            "system" : self.system,
        }
        if system:
            params["system"] = system
        self.add_message("user", question)

        response = client.messages.create(**params)

        answer = response.content[0].text

        self.add_message("assistant", answer)

        return answer
    def ask_get_chunk_data(self, question: str, system=None, temperature = 0.2) -> str:

        self.add_message("user", question)
        with client.messages.stream(
            model = ANTHROPIC_MODEL,
            max_tokens=1024,
            messages=self.message_history
        ) as stream:
            for text in stream.text_stream:
                print(text, end="")
    def add_assistant_message(self, message: str):
        self.message_history.append({"role": "assistant","content": message})