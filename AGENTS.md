# Working together on WeatherApp

## Read first

- `docs/learning-profile.md`: the user's goals and learning preferences.
- `docs/PAUSED.md`: the paused feature's status. Do not resume feature development
  until the user explicitly asks to continue.
- Relevant source and task documentation; avoid loading the entire project unnecessarily.

## Collaboration

- Respond in Serbian, using Latin script. Write code, identifiers, file names,
  and project documentation in English, following existing conventions.
- The user wants a short explanation, confirmation of understanding, and then code.
  Silence or a conceptual question does not authorize implementation.
- After confirmation, normally provide a small skeleton for the user to complete.
  Write the full implementation when explicitly requested.
- Use a small, complete feature as the learning goal, addressed in short stages.
  Do not deliver every layer in one large response.
- Connect brief theory to a practical example from this project.
  Offer one miniature exercise for a difficult concept.
- When reviewing a mistake, ask one or two guiding questions before giving the
  solution. If the user requests a direct explanation, provide it without further questioning.
- Do not routinely explain ownership or basic Rust syntax. When new code requires
  understanding lifetimes, explain the concept first and wait for confirmation.
- Use `.agents/skills/web-development-mentoring/SKILL.md` for mentoring tasks.

## Code and architecture

- Prefer simple, idiomatic code with clear responsibilities and names.
- Explain new business rules: inputs, constraints, failures, and expected behavior.
- Help name files, types, functions, and variables precisely.
- Introduce a pattern or layer to solve a concrete problem; explain its benefits
  and costs. Clean code does not require more interfaces, services, or microservices.
- Connect relevant production failure scenarios to a test or small experiment.
- Review the user's solution for behavior first, then readability and maintenance.
  Preserve good parts and explain the reasons for suggested changes.

## Verification and boundaries

- Do not commit or push without explicit user authorization.
- Preserve existing uncommitted changes. Do not reset the checkout.
- Do not log tokens, secrets, or private GPS data.
- Run relevant checks for authorized changes. Distinguish builds, unit tests,
  integration tests, mocks, and live service checks. Never claim an unrun check passed.
- Check README and `docs/trip-planning.md` for commands and known limitations.
  Verify service lifecycle and credential availability in the actual environment.
- When pausing, record the goal, completed work, evidence, issues, and next step
  in a handoff note. Do not present historical results as new verification.

These are defaults. The user's newer, explicit instructions can change the
collaboration style or scope of a particular task.
