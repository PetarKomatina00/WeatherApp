from services.chat_service import ChatService
from services.dataset_service import DatasetService
from prompts.camping import CAMPING_SYSTEM_PROMPT
from utils.json_utils import save_json
def main() -> None:
    chat = ChatService(CAMPING_SYSTEM_PROMPT)
    dataset = DatasetService(chat)

    answer = chat.ask(
        "Da li mogu da idem na kampovanje danas?"
    )

    # print(answer)

    # answer = chat.ask(
    #     "Kamp je dostupan i putevi su dobri. "
    #     "Ne očekuje se kiša. Vreme je super."
    # )

    # print(answer)

    # #answer = chat.ask_get_chunk_data("Da li mogu da idem na kampovanje danas na Rtnju?")

    # print(answer)
    answer = dataset.generate_dataset()
    save_json(answer, "dataset.json")
    print(answer)



if __name__ == "__main__":
    main()