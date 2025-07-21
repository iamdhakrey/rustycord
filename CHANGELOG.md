# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2025-07-21

### 🐛 Bug Fixes
- fix: update docs.rs build configuration and add development notice
- fix: allow publishing to crates.io with uncommitted changes

### 📝 Other Changes
- chore: update rustycord version to 0.1.3 in Cargo.lock
## [0.1.3] - 2025-07-21

### 📝 Other Changes
- docs: update documentation links to use angle brackets
- docs: update links in documentation to use angle brackets
- docs: update README and index files with project badges and title
- chore: update rustycord version to 0.1.2 in Cargo.lock
## [0.1.2] - 2025-07-21

### ✨ Features
- feat: add LICENSE file and update project license information in PROGRESS.md

### 🐛 Bug Fixes
- fix: ensure text messages are returned as strings in DiscordWebSocket

### 📝 Other Changes
- docs: add contributing guidelines and project roadmap
- refactor: improve ChannelMessage struct for better readability
- Add comprehensive logging and message handler examples
- chore: update Cargo.lock for v0.1.1 release
## [0.1.1] - 2025-07-19

### ✨ Features
- feat: add RustyCord release script
- feat: add initial changelog for project documentation
- feat: add Makefile for development commands
- feat: add comprehensive documentation and development warnings
- feat: enhance documentation with usage examples and development warnings
- feat: add package metadata and documentation assets
- feat: add troubleshooting guide and prefix command implementation
- feat: add comprehensive documentation for installation and usage of RustCord
- feat: update README.md with comprehensive bot setup and logging configuration
- feat: Implement ShardManager for managing Discord shards
- feat: update async-trait dependency to version 0.1.88 and increment Cargo.lock version
- feat: enhance logging in BotBase methods for better traceability
- feat: enhance Client struct with event dispatcher and message sending methods
- feat: implement builder methods for Embed struct
- feat: enhance HTTP client with message sending functionality
- feat: enhance Discord WebSocket handling with improved logging and client integration
- feat: add handlers module to the project
- feat: add message handler and event dispatcher modules
- feat: implement event dispatcher and message handler registry

### 🐛 Bug Fixes
- fix: Fix doctest compilation errors and naming convention
- fix: replace hardcoded Discord token with placeholder
- fix: correct capitalization in development notice
- fix: clone client for shard tasks to ensure proper task execution
- fix: change bot field to optional in User model

### 📝 Other Changes
- refactor: remove unused test cases and related structures from prefix.rs
- docs: add development commands section to README.md
- chore: remove obsolete source files and Cargo.toml
- chore: replace old images with new branding assets for rustycord
- Rename project from RustCord to rustycord, updating all references in documentation, examples, and source code. Adjusted package metadata, logos, and URLs to reflect the new name. Ensured consistency across all files, including Cargo.toml, README.md, and various documentation files. Updated example projects to use the new library name and verified that all instances of the old name were replaced.
- chore: update mkdocs configuration for development version
- chore: update .gitignore to include .env and log files
- refactor: remove unused imports from gateway module
- refactor: comment out event handling methods in Manager implementation
- Add prefix commands example with custom commands and setup instructions
- docs: update logger setup documentation and fix parameter typo
- initiate project
## [Unreleased]

### Added
- Initial RustyCord Discord bot library
- Basic bot functionality and message handling
- Prefix command system
- Comprehensive logging system
- WebSocket gateway connection
- HTTP client for Discord REST API
- Rich embed support
- Event handling and dispatching

### Development Status
- ⚠️  **This library is in heavy development and NOT ready for production use**
- APIs may change without notice
- Features are incomplete and experimental
- Breaking changes occur frequently

---

*Note: This changelog will be automatically updated by the release script based on conventional commit messages.*
