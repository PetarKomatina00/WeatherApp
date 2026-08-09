from services.chat_service import ChatService


def main() -> None:
    chat = ChatService()

    answer = chat.ask(
        "Da li mogu da idem na kampovanje danas?"
    )

    print(answer)

    answer = chat.ask(
        "Kamp je dostupan i putevi su dobri. "
        "Ne očekuje se kiša. Vreme je super."
    )

    print(answer)


if __name__ == "__main__":
    main()