DATASET_GENERATION_PROMPT = """
        Generate an evaluation dataset for a prompt evaluation. The dataset will be used to evaluate prompts
        that determine whether weather and outdoor conditions are suitable for camping.

        Generate an array of JSON objects, where each object represents a camping scenario that the AI should
        evaluate and decide whether camping is recommended.

        Return only valid JSON.

        Do not include Markdown code fences.
        Do not include ```json.
        Do not include explanations before or after the JSON.
        Your response must begin with [ and end with ].

        Example output:

        ```json
        [
            {
                "task": "Should the user go camping today?",
                "weather": {
                    "temperature": 22,
                    "feels_like": 21,
                    "rain_probability": 10,
                    "wind_speed": 8,
                    "humidity": 55,
                    "condition": "Partly cloudy"
                }
            },
            ...additional
        ]
        Please generate 3 objects.
        """
DATASET_SYSTEM_PROMPT = """
    You are an evaluation dataset generator.

    Your task is to generate realistic test cases for evaluating
    an AI camping and weather assistant.

    Follow the requested JSON format exactly.
    Do not answer the camping scenarios yourself.
    """
