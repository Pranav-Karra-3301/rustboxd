# Rustboxd Contributing Guide

Thank you for considering contributing to Rustboxd! This guide will help you get started.

## Development Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Pranav-Karra-3301/rustboxd.git
   cd rustboxd
   ```

2. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Install dependencies and run tests**:
   ```bash
   cargo test
   ```

## Project Structure

```
rustboxd/
├── src/
│   ├── core/           # Core functionality (client, errors, constants)
│   ├── models/         # Data models (User, Movie, Search, etc.)
│   ├── pages/          # Page-specific parsers
│   ├── utils/          # Utility functions
│   └── lib.rs         # Library entry point
├── examples/           # Usage examples
├── tests/             # Integration and unit tests
└── docs/              # Documentation
```

## Making Changes

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes and add tests**:
   - All new functionality should include tests
   - Follow the existing code style
   - Run `cargo fmt` to format your code
   - Run `cargo clippy` to check for issues

3. **Test your changes**:
   ```bash
   cargo test
   cargo check --examples
   ```

4. **Submit a pull request**:
   - Describe your changes clearly
   - Include any relevant issue numbers
   - Make sure all CI checks pass

## Code Style

- Use `cargo fmt` to format code
- Follow Rust naming conventions
- Write clear, descriptive variable and function names
- Add documentation comments for public APIs
- Keep functions small and focused

## Testing

- Write unit tests for utility functions
- Write integration tests for main functionality
- Mock external HTTP calls when possible
- Test error conditions

## Documentation

- Update README.md if adding new features
- Add examples for new functionality
- Document public APIs with rustdoc comments
- Keep examples up to date

## Reporting Issues

- Use the GitHub issue tracker
- Include a clear description of the problem
- Provide steps to reproduce
- Include relevant error messages

## Questions?

Feel free to open an issue for questions or join discussions in existing issues.
