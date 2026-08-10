from services.chat_service import ChatService
from .eval_prompt import get_eval_prompt
import json
from statistics import mean
class TestPrompt:
    def __init__(self, chatService: ChatService):
        self.chat = chatService
    def run_prompt(self, test_case):
        prompt = f"""
            Please solve the following task:
            {test_case["task"]}

            Weather data:
            {test_case["weather"]}
            """
        output = self.chat.ask(prompt)
        return output

    def run_test_case(self, test_case):
        output = self.run_prompt(test_case)

        # Grading
        ## TO DO
        model_grade = self.grade_by_model(test_case, output)
        score = model_grade["score"]
        reasoning = model_grade["reasoning"]

        return {
            "output" : output,
            "test_case" : test_case,
            "score" : score,
            "reasoning" : reasoning
        }
    def run_eval(self, dataset):
        results = []

        for test_case in dataset:
            result = self.run_test_case(test_case)
            results.append(result)
        average_score = mean([result["score"] for result in results])
        print(f"Average score : {average_score}")
        return results
    def grade_by_model(self, test_case, output):
        eval_prompt = get_eval_prompt(test_case, output)
        eval_text = self.chat.ask(eval_prompt)
        return json.loads(eval_text)