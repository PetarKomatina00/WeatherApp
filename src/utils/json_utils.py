import json
from pathlib import Path
from typing import Any

def save_json(data: Any, file_path: str) -> None:
    path = Path(file_path)
    with path.open("w", encoding="utf-8") as file:
        json.dump(data, file, indent=2)
def load_json(file_path: str | Path):
    path = Path(file_path)

    with path.open("r", encoding="utf-8") as file:
        return json.load(file)