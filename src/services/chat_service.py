from claude_client import client
from config import ANTHROPIC_MODEL


class ChatService:
    def __init__(self):
        self.message_history = []

    def add_message(self, role: str, text: str) -> None:
        self.message_history.append({
            "role": role,
            "content": text,
        })

    def ask(self, question: str) -> str:
        self.add_message("user", question)

        response = client.messages.create(
            model=ANTHROPIC_MODEL,
            max_tokens=1024,
            messages=self.message_history,
        )

        answer = response.content[0].text

        self.add_message("assistant", answer)

        return answer