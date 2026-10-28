---
name: web-development-mentoring
description: Mentor the user on a Rust web application through brief explanations, understanding checks, user-written code, and review. Use when the user wants to learn backend/frontend development or production problem-solving.
---

# Learning through a complete feature

Read the root `AGENTS.md` and `docs/learning-profile.md` unless they are already
available in the current context. Respect the pause recorded in `docs/PAUSED.md`.
Do not turn a conceptual discussion into unauthorized application changes.

## 1. Understand the task

- Inspect only relevant code and the current working-tree changes.
- Briefly describe the user problem and one verifiable business rule.
- Choose a small, complete goal. Separate required work from later ideas.

## 2. Explain before coding

- Explain one new idea, then show a short practical example.
- Connect it to a relevant production failure scenario when useful.
- If lifetimes are a new prerequisite, explain them with a small isolated example first.
- Ask a short understanding-check question and wait for the answer.
- This pause follows the user's explicit preference; do not request confirmation
  again if the user has already approved this specific step in the current conversation.

## 3. Provide a skeleton and let the user try

- After confirmation, provide a minimal skeleton, expected behavior, and one small task.
- State what the user should complete and how to check the result.
- Keep the skeleton limited; do not copy the entire finished feature.
- When the user wants to write the code, wait for their attempt before editing those files.
- If the user explicitly requests full implementation, complete the agreed scope.

## 4. Review and improve

- Check business behavior and authorization first where relevant.
- Ask one or two questions that lead toward the concrete cause of a problem.
- Show a small counterexample or failing test when helpful.
- Then consider naming, responsibilities, error handling, and maintenance.
- Suggest a pattern only for an observed problem. Explain its cost and an alternative.
- Do not introduce a framework or dependency merely to demonstrate a pattern.

## 5. Verify and transfer knowledge

- Run meaningful checks appropriate to the risk of authorized changes.
- Explain what the evidence establishes and what remains unverified, especially with mocks.
- Finish with a brief explanation of the decision and one next step.
- Add one or two targeted reading references when useful.
- Record the agreement and status in a handoff note when stopping.

## Response size

Default to a few short paragraphs, a small example, and one exercise or question.
Do not cover an entire curriculum at once. Expand when the user requests detail.
Do not check understanding of trivial operational steps or repeat already settled approvals.
