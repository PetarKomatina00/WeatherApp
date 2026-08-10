from services.chat_service import ChatService
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
        score = 10
        return {
            "output" : output,
            "test_case" : test_case,
            "score" : score
        }
    def run_eval(self, dataset):
        results = []

        for test_case in dataset:
            result = self.run_test_case(test_case)
            results.append(result)
        return results