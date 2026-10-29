---
applyTo: "**/*"
---

# GitHub Copilot Instructions - DataBank Development Standards

## Purpose
This file defines how GitHub Copilot should generate code for DataBank IMX development teams.
All generated code must align with internal standards for quality, maintainability, security, and consistency.

## Scope
This repository is a **Rust training repository**. Follow the **Common Standards** section and the **Rust Project Instructions** section for code generation in this repository.

---

## Common Standards

### General Principles
- Prioritize **readability, maintainability, and clarity** over brevity.
- Follow **DataBank coding standards** at all times.
- Assume code will be **reviewed, tested, and deployed to client environments**.
- Avoid unsafe, experimental, or non-production-ready patterns.
- Do not introduce unnecessary dependencies or frameworks.
- Third-party packages **must** have licenses that allow closed-source commercial use.
- Prefer existing project patterns over introducing new architectural or library choices.

### Architecture and Design
- Prefer clear separation of concerns.
- Avoid tightly coupled designs.
- Design for reuse and extensibility.
- Keep controllers, endpoints, and entry points thin; place business logic in services or domain components.
- Match the structure and conventions already used by the repository unless the user explicitly requests a redesign.

### Error Handling and Security
- Validate all inputs.
- Avoid exposing sensitive information in exceptions, logs, API responses, or console output.
- Follow least-privilege principles.
- Do not generate insecure defaults, hardcoded secrets, or bypass validation/authentication/authorization checks.
- Exception and error messages must be clear, concise, grammatically correct, and end with a period.

### Comments and Documentation
- Write comments only when necessary.
- Use clear, concise US-English.
- Document complex logic, non-obvious decisions, and important operational behavior.
- Avoid comments that merely restate the code.

### Source Control and Workflow
- All code must be tracked in source control.
- Commit early and often.
- Use meaningful commit messages.
- Link commits to tasks, issues, bugs, or enhancements where appropriate.
- Use branches and pull requests for review; do not commit directly to protected mainline branches.

### Code Reviews and Quality Gates
- All code must be reviewed before merge or delivery.
- Address correctness, readability, performance, security, and maintainability.
- Where applicable, code must pass SonarQube or other required quality gates before merge.

### Testing and QA Expectations
- Follow a **test-driven mindset** where practical.
- Ensure new code is testable.
- Include unit tests and integration tests where appropriate.
- Consider edge cases, invalid inputs, failure scenarios, and regression risk.

### Prohibited Practices
- Do NOT generate hardcoded credentials, secrets, tokens, or connection strings.
- Do NOT introduce unapproved third-party libraries.
- Do NOT expose internal implementation details unnecessarily.
- Do NOT produce code that violates DataBank IP, security, or delivery policies.

---

## Rust Project Instructions

### General Style
- Write idiomatic, readable Rust and follow the conventions of the surrounding lesson or crate.
- Prefer clear ownership and borrowing over unnecessary cloning or complex lifetime workarounds.
- Use descriptive names and keep functions focused.
- Runnable lesson examples use Cargo and the 2024 edition; preserve the repository's established project layout.
- Format Rust code with `cargo fmt` and address relevant `cargo clippy` warnings when practical.

### Errors and Safety
- Use `Result` and the `?` operator for recoverable errors where appropriate.
- Use `panic!`, `unwrap`, or `expect` only when the lesson is teaching those behaviors or the failure is genuinely unrecoverable.
- Avoid unsafe Rust unless it is required by the task; explain and minimize any unsafe block.
- Do not suppress compiler warnings without a lesson-specific reason.

### Testing
- Use Rust's built-in test support and run `cargo test` for the affected crate when practical.
- Keep unit tests in the module or `src/lib.rs` as appropriate; put integration tests in `tests/`.
- Runnable lesson steps should compile cleanly and follow the checks in `AUTHORING.md`.

### Training Repository Notes
- This repository contains lesson examples and a lesson runner. Follow `AUTHORING.md` when creating or changing lessons, quizzes, homework, and runnable examples.
- Keep examples appropriate for the concept being taught; do not replace intentional teaching examples with unrelated production abstractions.
- The lesson runner is a separate Tauri application. Keep its Rust changes within its existing crate structure.

---

## Copilot Behavior Guidelines

When generating code, Copilot should:
- Follow all applicable rules in this document automatically.
- Prefer existing project patterns over new ones.
- Match surrounding code style closely.
- Avoid speculative implementations.
- Ask for clarification through comments or generated TODO notes only when requirements are genuinely unclear.

---

## Definition of Done

Generated code is acceptable only if it:
- Runs successfully without warnings in the target project context.
- Passes required static analysis and quality gates where applicable.
- Follows the Rust standards in this file.
- Includes appropriate validation, error handling, and logging.
- Is readable, maintainable, and testable.
