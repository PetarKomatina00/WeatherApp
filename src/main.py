from claude_client import client
from config import ANTHROPIC_MODEL


def add_user_message(messageHistory, text):
    user_message = {
        "role" : "user",
        "content" : text
    }
    messageHistory.append(user_message)
def add_assistant_message(messageHistory,text):
    assistant_message = {
        "role" : "assistant",
        "content" : text
    }
    messageHistory.append(assistant_message)

def chat(messagesHistory):
    message = client.messages.create(
        model=ANTHROPIC_MODEL,
        max_tokens=1024,
        messages=messagesHistory
        )
    return message.content[0].text

def main() -> None:
    messageHistory = []

    add_user_message(messageHistory, "Da li mogu da idem na kampovanje danas")
    answer = chat(messageHistory)
    add_assistant_message(messageHistory, answer)
    add_user_message(messageHistory, "Kamp je dostupan i putevi su dobri. Ne ocekuje se kisa. Vreme je super. Znaju svi gde ide.")
    answer = chat(messageHistory)
main()