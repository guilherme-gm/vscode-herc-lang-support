# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2019-05-02

### Changed
- Removed dev dependencies from release package. (reduced its size)

## [0.3.0] - 2019-05-02

### Added
- Auto-generated docs of all script commands in Herc's script_commands.txt
- Variable autocomplete

### Changed
- Recoded the script parser in Tree-Sitter instead of ANTLR, this improved code analysis a lot.

## [0.2.0] - 2019-04-24
- Split into Client and Language Server.

### Added
- License (MIT)

### Changed
- Big structural change to run in client/server mode
- Fixed `package.json` information

## [0.1.0] - 2019-04-23
- Initial release
    * Syntax Highlighting
    * A small set of commands included in autocomplete and signature helper

[0.3.0]: https://github.com/guilherme-gm/vscode-herc-lang-support/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/guilherme-gm/vscode-herc-lang-support/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/guilherme-gm/vscode-herc-lang-support/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/guilherme-gm/vscode-herc-lang-support/releases/tag/v0.1.0
