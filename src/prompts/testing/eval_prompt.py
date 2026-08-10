def get_eval_prompt(test_case, output):
    eval_prompt = f"""
        You are an expert AI Camping Assistant and outdoor safety evaluator. Your task is to evaluate the following AI-generated camping recommendation.

        Original Camping Scenario:

        {test_case["task"]}


        Weather Conditions:

        {test_case["weather"]}


        Solution to Evaluate:

        {output}


        Evaluation Criteria:
        Evaluate whether the proposed recommendation correctly determines if the weather and conditions are suitable and safe for camping.

        Consider factors such as:
        - Temperature and feels-like temperature
        - Precipitation and probability of rain
        - Wind speed and wind gusts
        - Thunderstorms or severe weather
        - Humidity
        - Overnight conditions
        - Any weather-related safety risks relevant to camping

        The recommendation should clearly determine whether camping is favorable, unfavorable, or possible with caution based on the provided scenario and weather data.


        Output Format
        Provide your evaluation as a structured JSON object with the following fields, in this specific order:

        - "strengths": An array of 1-3 key strengths
        - "weaknesses": An array of 1-3 key areas for improvement
        - "reasoning": A concise explanation of your overall assessment, including whether the camping recommendation is supported by the provided weather conditions
        - "score": A number between 1-10

        Strict output requirements:

        Do not use Markdown code fences.
        Do not include json or .
        Do not include any text, explanation, or commentary before or after the JSON object.
        The response must be parseable directly with Python's json.loads().
        Use exactly these fields, in this order:
        "strengths"
        "weaknesses"
        "reasoning"
        "score"
        Example response shape:
        {{
        "strengths": string[],
        "weaknesses": string[],
        "reasoning": string,
        "score": number
        }}
        """
    return eval_prompt