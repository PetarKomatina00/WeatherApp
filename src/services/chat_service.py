from claude_client import client
from config import ANTHROPIC_MODEL


class ChatService:
    def __init__(self):
        self.message_history = []
        self.system = """
            You are an AI Camping and Weather Assistant integrated into a WeatherDashboard application.
            Your primary goal is to help users decide whether current or forecast weather conditions are suitable for camping, hiking, and other outdoor activities.
            You will receive weather data from the application. Base your recommendations primarily on this provided data.
        """

    def add_message(self, role: str, text: str) -> None:
        self.message_history.append({
            "role": role,
            "content": text,
        })

    def ask(self, question: str, system=None) -> str:

        params = {
            "model" : ANTHROPIC_MODEL,
            "max_tokens" : 1024,
            "messages" : self.message_history,
            "system" : self.system
        }
        if system:
            params["system"] = system
        self.add_message("user", question)

        response = client.messages.create(**params)

        answer = response.content[0].text

        self.add_message("assistant", answer)

        return answer