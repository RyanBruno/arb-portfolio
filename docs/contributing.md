# Contributing Guide

Thank you for considering a contribution to **arb-portfolio**. This guide explains the workflow and conventions expected for patches.

## Workflow

1. Fork the repository and create your feature branch.
2. Make your changes and ensure they follow the coding conventions below.
3. Run `cargo fmt` and `cargo test` to verify formatting and ensure tests pass.
4. Commit your changes with a descriptive message.
5. Open a pull request describing your changes.

## Coding Conventions

- Code is formatted with `rustfmt`. Run `cargo fmt --all` before committing.
- Keep functions and modules focused. Prefer small, composable functions over large ones.
- Include unit tests for new features when possible.

## Commit Messages

Use present tense and keep the first line under 50 characters, e.g.:

```
docs: add contributing guide
```

## Questions

If you have any questions, open an issue or start a discussion in the repository.
