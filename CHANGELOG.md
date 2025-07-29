# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### 💼 Other

- *(deps)* Bump tokio from 1.46.1 to 1.47.0

## [0.4.2] - 2025-07-15

### 💼 Other

- *(deps)* Bump stefanzweifel/git-auto-commit-action
- *(deps)* Bump tokio from 1.45.1 to 1.46.1
- *(deps)* Bump clap from 4.5.40 to 4.5.41

### ⚙️ Miscellaneous Tasks

- Include dependency updates in the release

## [0.4.1] - 2025-06-15

### 🐛 Bug Fixes

- *(ci)* Potential fix for code scanning alert no. 1: Workflow does not contain permissions
- *(ci)* Potential fix for code scanning alert no. 2: Workflow does not contain permissions
- Clippy::single_component_path_imports
- *(ci)* Include Cargo.lock in release preparation paths

### 💼 Other

- *(deps)* Bump clap from 4.5.39 to 4.5.40

## [0.4.0] - 2025-06-13

### 🚀 Features

- [**breaking**] Debug mode replaced verbose mode
- Add debug logging for before and after tick via osc

### 🐛 Bug Fixes

- Update debug logging message to reflect mode change
- *(ci)* Fix git-cliff config

### 💼 Other

- *(deps)* Bump clap from 4.5.36 to 4.5.37
- *(deps)* Bump astral-sh/setup-uv from 5 to 6 in /.github/workflows
- *(deps)* Bump chrono from 0.4.40 to 0.4.41
- *(deps)* Bump tokio from 1.44.2 to 1.45.0
- *(deps)* Bump clap from 4.5.37 to 4.5.38
- *(deps)* Add tracing, tracing_subscriber
- *(deps)* Bump tokio from 1.45.0 to 1.45.1
- *(deps)* Bump clap from 4.5.38 to 4.5.39

### 📚 Documentation

- Cli options updated with debug mode and demo mode

### 🚜 Refactor

- Remove verbose mode and integrate tracing for logging
- Enhance error handling and logging in time update functions

## [0.3.4] - 2025-04-14

### 🐛 Bug Fixes

- *(ci)* Avoid overwriting the changelog on release PRs

### 💼 Other

- *(deps)* Bump tokio from 1.44.1 to 1.44.2
- *(deps)* Bump clap from 4.5.35 to 4.5.36

### ⚙️ Miscellaneous Tasks

- *(changelog)* Update on pull requests

## [0.3.3] - 2025-04-06

### ⚙️ Miscellaneous Tasks

- Bump cargo.toml automatically
- Remove tag trigger from changelog generation
- Fix uploading rust binary workflow
- Update version in Cargo.lock alongside Cargo.toml
- Fix typo in script path for release preparation

## [0.3.2] - 2025-04-05

### 🐛 Bug Fixes

- *(release)* Update binary name to include branch name
- Moon phase is not working

### 💼 Other

- *(deps)* Bump clap from 4.5.32 to 4.5.34
- *(deps)* Bump clap from 4.5.34 to 4.5.35
- *(deps)* Skip changelog generation for build dependencies

### ⚙️ Miscellaneous Tasks

- *(release)* Simplify branch in release candidate
- Add clippy check
- Install sarif-fmt

## [0.3.1] - 2025-03-25

### 💼 Other

- *(deps)* Bump rosc from 0.11.3 to 0.11.4

## [0.3.0] - 2025-03-21

### 🚀 Features

- Add OSC address validation with tests

### 🐛 Bug Fixes

- Bump version in cargo.toml

### 💼 Other

- *(deps)* Bump orhun/git-cliff-action in /.github/workflows

### 🚜 Refactor

- Move OscSender implementation to a separate module
- Abstraction of OSC message sending

### ⚙️ Miscellaneous Tasks

- Update changelog workflow to trigger on release
- Add release preparation action to release automatically
- Git-cliff-action puts new changelog
- Fix release preparation branch
- Fix release preparation branch

## [0.2.0] - 2025-03-14

### 🚀 Features

- Send lunar phase

### 💼 Other

- *(deps)* Bump clap from 4.5.31 to 4.5.32
- *(deps)* Bump tokio from 1.44.0 to 1.44.1
- *(deps)* Bump rosc from 0.11.2 to 0.11.3
- Add moon-phase

### 📚 Documentation

- Add demo movie on README
- Add unofficial watch compatibility
- *(changelog)* Update CHANGELOG.md

### 🚜 Refactor

- Simplify time sending logic

### ⚙️ Miscellaneous Tasks

- Use Swatinem/rust-cache to cache dependencies
- Generate changelog by git-cliff
- Fix endless changelog update
- Fix git-cliff sorting order to ignore chore(changelog)

## [0.1.1] - 2025-03-09

### 💼 Other

- Version 0.1.0 -> 0.1.1
- Update dependencies

### 📚 Documentation

- Changelog for v0.1.1

### ⚙️ Miscellaneous Tasks

- Fix compilation platform for release

## [0.1.0] - 2025-03-09

### 🐛 Bug Fixes

- Hard-coded port number
- Use SocketAddrV4 for binding UDP socket
- Update issue templates for better clarity in Japanese

### 💼 Other

- Basic implementation
- Unused debug code
- Verbose logging for current time in CLI
- Readme
- Readme
- Build and unit test
- Dependabot settings
- Tokio for async
- Sleep with async/await
- Function `tick` to `tick_clock` for clarity
- *(deps)* Bump rosc from 0.10.1 to 0.11.1
- Rename demo mode to clarify the meaning
- Helps for options
- Rename functions
- Transfer ownership of the inputs
- Sigint handler
- Send osc message continuously on demo mode
- Exit message if Ctrl-C is pressed
- Code
- Contributing

### 📚 Documentation

- Update README
- Add system requirements to README
- Change authors
- Add changelog
- Add changelog for version 0.1.0

### ⚙️ Miscellaneous Tasks

- Clean up comments on dependabot.yml
- Add GitHub Actions updates to dependabot.yml
- Add issue templates
- Update Cargo.toml with license, description, and publish settings
- Add release workflow

<!-- generated by git-cliff -->
