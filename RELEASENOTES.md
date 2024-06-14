# 0.3.0
- Updated outputs of the `database list` and `status` command to improve readability
# 0.2.9
- Updated list command to support arg called `all`. By default, calling list will only print in progress tasks.
# 0.2.8
- Improved listing of todos
- Clap version bump
# 0.2.7
- Implemented the status command
# 0.2.6
- Improved file structure of project
- Updated readme with details on the status command
# 0.2.5
- Improved error messages for todo commands
- Addressed suggestion from compiler which prevents a potential bug in state manager
- Version bumps for chrono, clap, and tempfile dependencies
# 0.2.4
- Added state manager
# 0.2.3
- Added init command for first-time setup
- Added new sub command to databases command list (set-directory)
- New errors added to db_errors.rs
# 0.2.2
- Added pre-commit support
- Initial testing introduced for persistent.rs file
# 0.2.1
- Fixed known bug where current directory path was used instead of executable path. This caused errors when running tmgr from different terminals.
- Introduced database errors and ensured that database commands return errors properly.
# 0.2.0
- Functionality added to database commands
- Persistent function library implemented and commented out
# 0.1.0
- Boiler plate built
- Initial CLI design implemented
