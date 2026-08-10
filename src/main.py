from services.chat_service import ChatService
from services.dataset_service import DatasetService
from prompts.camping import CAMPING_SYSTEM_PROMPT
from utils.json_utils import save_json,load_json
from pathlib import Path
from prompts.testing.test_prompt import TestPrompt
import json
def main() -> None:
    chat = ChatService(CAMPING_SYSTEM_PROMPT)
    prompt_test = TestPrompt(chat)

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
    
    #These gets called only once

    file_path = Path("dataset.json")
    if not file_path.exists():
        dataset = DatasetService(chat)
        answer = dataset.generate_dataset()
        save_json(answer, "dataset.json")
        print("Dataset created")
    else:
        print("Dataset already exists. Good.")

    if file_path.exists():
        dataset = load_json(file_path)
        results = prompt_test.run_eval(dataset)
        print(json.dumps(results, indent=2))


if __name__ == "__main__":
    main()