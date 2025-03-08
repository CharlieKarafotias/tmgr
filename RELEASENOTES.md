# Release Notes

## 2.3.0

- Refactor of code base to improve readability and maintainability
  - All `pub(crate)` visibilities are now `pub(super)` to ensure proper visibility
  - `TaskPriority` struct now implements `From` trait for `String` type
  - New `model` module for struct definitions to replace models existing in commands and cli modules
  - Replaced all legacy code for obtaining the id of a task in favor of `id` method on `Task` struct
  - Added getters for all fields on `Task` struct
  - Proper error handling - no more `unwrap` calls in code base
  - `Task` struct now implements function to retrieve fields required for building tables in `list` and `view` commands
- Improved test suites for all commands
- Dependency updates
  - Bumped `tempfile` from 3.17.1 to 3.18.0
  - Bumped `tokio` from 1.43.0 to 1.44.0

## 2.2.6

- Several improvements to `note` commands behavior
  - Updates default behavior of note open to use `$EDITOR` environment variable
  - New note files will have proper markdown specifications on creation
- Upgraded project from Rust edition 2021 to 2024
- Dependency updates
  - Bumped `clap` from 4.5.27 to 4.5.31
  - Bumped `serde` from 1.0.217 to 1.0.218
  - Bumped `surrealdb` from 2.1.4 to 2.2.1
  - Bumped `semver` from 1.0.25 to 1.0.26
  - Bumped `comfy-table` from 7.1.3 to 7.1.4
  - Bumped `chrono` from 0.4.39 to 0.4.40
  - Bumped `tempfile` from 3.15.0 to 3.17.1

## 2.2.5

- Bumped `clap` from 4.5.23 to 4.5.27
- Bumped `serde` from 1.0.216 to 1.0.217
- Bumped `surrealdb` from 2.1.3 to 2.1.4
- Bumped `tokio` from 1.42.0 to 1.43.0
- Bumped `reqwest` from 0.12.9 to 0.12.12
- Bumped `semver` from 1.0.24 to 1.0.25
- Bumped `directories` from 5.0.1 to 6.0.0
- Bumped `colored` from 2.2.0 to 3.0.0
- Bumped `tempfile` from 3.14.0 to 3.15.0

## 2.2.4

- Bumped `clap` from 4.5.21 to 4.5.23
- Bumped `serde` from 1.0.215 to 1.0.216
- Bumped `surrealdb` from 2.1.0 to 2.1.3
- Bumped `tokio` from 1.41.1 to 1.42.0
- Bumped `semver` from 1.0.23 to 1.0.24
- Bumped `colored` from 2.1.0 to 2.2.0
- Bumped `chrono` from 0.4.38 to 0.4.39
- Removed unused dependency `serde_json`

## 2.2.3

- Updated GitHub release script
- Bumped `surrealdb` dependency from `2.0.4` to `2.1.0`

## 2.2.2

- Reduced duplicate code throughout project by refactoring
- Updated wording from `task starting with <id>` to `task <id>`
- Updated output of `view` command to improve readability

## 2.2.1

- Improved test file structure
- Bumped `serde_json` dependency from `1.0.132` to `1.0.133`

## 2.2.0

- Added a `note` command which allows you to add notes to a task using Markdown
- Bumped `tempfile` dependency from `3.13.0` to `3.14.0`
- Bumped `comfy-table` dependency from `7.1.1` to `7.1.3`
- Bumped `tokio` dependency from `1.41.0` to `1.41.1`
- Bumped `serde` dependency from `1.0.214` to `1.0.215`
- Bumped `clap` dependency from `4.5.20` to `4.5.21`

## 2.1.4

- Bumped `serder` dependency from `1.0.210` to `1.0.214`
- Bumped `tokio` dependency from `1.40.0` to `1.41.0`
- Bumped `serde_json` dependency from `1.0.128` to `1.0.132`
- Bumped `reqwest` dependency from `0.12.8` to `0.12.9`


## 2.1.3

- Addressed open issues reported by GitHub vulnerability scanner
- Bumped `surrealdb` dependency from `2.0.2` to `2.0.4`
- Bumped `clap` dependency from `4.5.18` to `4.5.20`
- Bumped `reqwest` dependency from `0.12.7` to `0.12.8`

## 2.1.2

- Bumped `surrealdb` dependency from `2.0.1` to `2.0.2`
- Added pre-commit hook for automating release

## 2.1.1

- The `list` and `view` commands now utilize ascii_table crate for output. This improves readability.

## 2.1.0

- Improved result handling experience by standardizing approach to errors in `tmgr`
- Fixed issue where commands were printing results where String was derived from `Debug`
  trait. [See here](https://github.com/CharlieKarafotias/tmgr/issues/73#issuecomment-2365190468) for details.
- Fixed exit codes for project:
    - `0` for success
    - `1` for general errors
    - `2` for CLI usage errors
- `clap` dependency bumped to v4.5.18
- Updated readme

## 2.0.0

- Updated to use version 2 of SurrealDB
- Breaking change: Tasks are now based off ID instead of name. This improves the user experience with CLI interactions.

## 1.0.0

- Migration from SqLite3 to SurrealDB
- Reworking of CLI commands
    - Removal of `database` command and subcommands
    - Removal of `init` command
    - Removal of `todo` command. These commands were refactored to top level commands.
- Addition of `view` command

## 0.4.4

- Dependency updates

## 0.4.3

- Bug fix for `todo update` command. When updating a task, it is no longer required to provide all fields.
- Error logs are now red
- When `tmgr` errors, the exit code is now 1 instead of 0
- Bumping versions of several dependencies

## 0.4.2

- Improved the error handling experience by standardizing approach to errors in tmgr
- Refactored `init` command.
- Removed old error pattern using Box

## 0.4.1

- Bug fix for new `update` command. There was an issue with the request model not matching api response from GitHub

## 0.4.0

- Added in new `update` command to facilitate updating tmgr to the latest version

## 0.3.0

- Updated outputs of the `database list` and `status` command to improve readability

## 0.2.9

- Updated list command to support arg called `all`. By default, calling list will only print in progress tasks.

## 0.2.8

- Improved listing of todos
- Clap version bump

## 0.2.7

- Implemented the status command

## 0.2.6

- Improved file structure of project
- Updated readme with details on the status command

## 0.2.5

- Improved error messages for todo commands
- Addressed suggestion from compiler which prevents a potential bug in state manager
- Version bumps for chrono, clap, and tempfile dependencies

## 0.2.4

- Added state manager

## 0.2.3

- Added init command for first-time setup
- Added new sub command to databases command list (set-directory)
- New errors added to db_errors.rs

## 0.2.2

- Added pre-commit support
- Initial testing introduced for persistent.rs file

## 0.2.1

- Fixed known bug where current directory path was used instead of executable path. This caused errors when running tmgr
  from different terminals.
- Introduced database errors and ensured that database commands return errors properly.

## 0.2.0

- Functionality added to database commands
- Persistent function library implemented and commented out

## 0.1.0

- Boiler plate built
- Initial CLI design implemented
