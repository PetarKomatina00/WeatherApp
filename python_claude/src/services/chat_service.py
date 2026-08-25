from src.claude_client import client
from src.config import ANTHROPIC_MODEL
import json;
class ChatService:
    def __init__(self, system_prompt: str):
        self.system = system_prompt
        self.message_history = []

    def add_message(self, role: str, text: str) -> None:
        self.message_history.append({
            "role": role,
            "content": text.rstrip(),
        })

    def ask(self, question: str, use_mcp_weather, weather_data = None, system=None, temperature = 0.2) -> str:


        question = question.strip()
        if not question:
            raise ValueError("Question cannot be empty")
        if weather_data:
            weather_data_model = weather_data.model_dump_json()
        
            user_message = f"""

            If the user asks  "Can I go on camping today" or "Da li mogu da idem na kampovanje danas?"
            use to provided weather_data if it is available

            <question>
            {question}
            </question>

            <weather_data>
            {weather_data_model}
            </weather_data>

            <weather_tool_enabled>
            {use_mcp_weather}
            </weather_tool_enabled>

            Use the provided weather data only if it belongs to the location the user is currently asking about.

            If the provided weather data belongs to a different location, do not use it to evaluate the user's current location.

            If relevant weather data is not available and the question requires current weather information, follow the weather tool rules from the 
            system prompt.
            """
        
            self.add_message("user", user_message)
        else:
            content = f"""
            Current user question:
            {question}
        
            <weather_data>
            No weather data is currently available.
            </weather_data>

            <weather_tool_enabled>
            {use_mcp_weather}
            </weather_tool_enabled>
        
            """
            self.add_message("user", question)
        
        params = {
            "model" : ANTHROPIC_MODEL,
            "max_tokens" : 1024,
            "messages" : self.message_history,
            "temperature" : temperature,
            "system" : system if system else self.system
        }
        # if weather_data is valid, claude will send a message with \n.
        # examplke messages = [ {"role": "user", "content": "Kakvo je vreme?"}, {"role": "assistant", "content": "Trenutno je sunčano.\n"},]
        # Error is: Exception in ASGI application
        # When creating message history we need to rstrip()

        if use_mcp_weather:
            pass
        else:
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
        self.message_history.append({"role": "assistant","content": message.rstrip()})