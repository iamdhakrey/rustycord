# Contributing to Rustycord

Thank you for your interest in contributing to Rustycord! This document provides guidelines and instructions for contributing to this project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Pull Requests](#pull-requests)
- [Development Setup](#development-setup)
- [Coding Guidelines](#coding-guidelines)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Testing](#testing)
- [Documentation](#documentation)

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
   ```bash
   git clone https://github.com/YOUR-USERNAME/rustcord.git
   cd rustcord
   ```
3. Add the original repository as an upstream remote
   ```bash
   git remote add upstream https://github.com/iamdhakrey/rustcord.git
   ```
4. Create a new branch for your feature or bugfix
   ```bash
   git checkout -b feature/your-feature-name
   ```

## How to Contribute

### Reporting Bugs

Before submitting a bug report, please check that it hasn't already been reported. If you find a bug that hasn't been reported, create an issue on the repository with the following information:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Screenshots or code snippets if applicable
- Environment information (OS, Rust version, etc.)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. Create an issue with:

- A clear, descriptive title
- Detailed explanation of the proposed feature
- Any possible implementation details or ideas
- Relevant examples of how the feature would be used

### Pull Requests

1. Update your fork to the latest upstream version
   ```bash
   git fetch upstream
   git merge upstream/main
   ```
2. Create a new branch for your changes
3. Make your changes following the coding guidelines
4. Add or update tests as needed
5. Update documentation as needed
6. Commit your changes with clear commit messages
7. Push your branch to your fork
8. Submit a pull request to the main repository

For pull requests, please:

- Include a clear description of the changes
- Link any related issues
- Update relevant documentation
- Ensure all tests pass
- Follow the coding style guidelines

## Development Setup

1. Ensure you have Rust and Cargo installed (minimum version 1.60)
2. Install required dependencies
   ```bash
   cargo build
   ```
3. Set up pre-commit hooks (optional)
   ```bash
   cargo install cargo-husky
   cargo husky install
   ```

## Coding Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for code formatting (run `cargo fmt` before committing)
- Run `cargo clippy` to catch common mistakes and improve code quality
- Write clear comments for public API functions
- Maintain backward compatibility where possible

## Commit Message Guidelines

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line
- Consider starting the commit message with an applicable prefix:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation changes
  - `style:` for formatting changes
  - `refactor:` for code refactoring
  - `test:` for adding or modifying tests
  - `chore:` for changes to the build process or auxiliary tools

## Testing

- Write tests for all new features and bug fixes
- Ensure all existing tests pass before submitting a pull request
- Run tests with `cargo test`
- For performance-critical code, include benchmarks

## Documentation

- Document all public API functions, types, and modules
- Update the README.md with any necessary changes
- Update example code to reflect changes
- Consider adding examples for new features

Thank you for contributing to Rustycord!
