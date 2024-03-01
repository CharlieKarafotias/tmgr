# Todo Manager CLI (TMGR)
A simple to use todo tracker command line interface for tracking all things coding. 

## Development Roadmap

- [] Multi Sqlite3 Database support __(In-Progress)__
    - Data migration based on Rust types. Dev feature that enables new DBs to be created based off Rust types to reduce redundancy __(Planned)__
    - Known bug: Correct path functions to ensure tmgr runs properly no matter the execution location __(Done)__
- [] Data structures designed __(In-Progress)__
- [] Command Line Interface Implemented __(In-Progress)__
    - Improve error handling. Custom error kinds/messages for various scenarios __(In-Progress)__
- [] Bash install script __(Planned)__
- [] Improved persistent state storage solution. See [Behind The Scenes -> Persistent Storage](#persistent-storage) section for more information __(Done)__
- [] Simple GUI __(Planned)__

## Getting Started

### Installation
- Ensure Rust is installed on your computer. For instructions on installing Rust, see the [Rust installation guide](https://www.rust-lang.org/tools/install).
- Run `cargo run`. This should return all the commands that are available for the program by default. 
    - Run a specific command like `cargo run -- todo add "my first todo"`
- Build the project with `cargo build`
- To contribute to this repo, ensure [Pre-commit](https://pre-commit.com/#install) is installed on your computer. This project utilizes pre-commit to ensure consistency throughout commits.

## CLI Commands 

TMGR commands are broken up into 4 types. 

### Database Commands 

Database commands manage the state of the sqlite3 database that store tasks.
- Add: Adds a new database inside the databases directory
    - Arguments:
        - NAME: The database name
    - Example:
        - `tmgr database add 2024Tasks`
- Delete: Deletes a database inside the databases directory (if found)
    - Arguments:
        - NAME: The database name
    - Example:
        - `tmgr database delete 2024Tasks`
- List: Lists all databases by name stored inside the databases directory
    - Arguments:
        - None
    - Example:
        - `tmgr database list`
- Set: Sets the current working database (to add/remove tasks from). This must be a database located in the databases directory.
    - Arguments:
        - Name: The database name
    - Example:
        - `tmgr database set 2024Tasks`

### Init Command

Initializes tmgr for first time use. This consists of running 3 subcommands from the databases subcommand list:
- `tmgr database set-directory .`
- `tmgr database add init-db`
- `tmgr database set init-db`

If successful, a message stating _Initializer complete!_ will show.

### Status Command

Returns the status of the program such as the current working directory, the location of the database folder, the total amount of tasks in the current database and more. This is not implemented yet.

### Todo Commands 

Todo commands manage the state of a task inside the current working database.
- Add: Adds a new task
    - Arguments:
        - Name: A short description of the task
        - Priority (default high): The priority of the task [possible values: low, medium, high]
        - Description (optional): A long description of the task
    - Example(s):
        - `tmgr todo add "Read AWS document"`
        - `tmgr todo add "Read AWS document" low "Read the concurrent execution section of the lambda documentation"`
- Complete: Mark a task as complete
    - Arguments:
        - ID: The id of the task
    - Example
        - `tmgr todo complete 1`
- Delete: Delete a task
    - Arguments:
        - ID: The id of the task
    - Example
        - `tmgr todo delete 1`
- List: List all tasks
    - Arguments:
        - None
    - Example:
        - `tmgr todo list`
- Update: Update an existing task
    - Arguments:
        - ID: The id of the task
        - Name (Optional): A short description of the task
        - Priority (Optional): The priority of the task [possible values: low, medium, high]
        - Description (Optional): A long description of the task
    - Example:
        - `tmgr todo update 2024Tasks`

### Help Command 

The help command will present all available subcommands that TMGR supports. Further, help can be used within other subcommands to learn more about each commands functionality. 

Example: 
- `tmgr -h`
- `tmgr --help`

## Behind The Scenes:

### Persistent Storage

TMGR utilizes the [dotenv](https://docs.rs/dotenv/latest/dotenv/) crate to store state between sessions. In the future, this will be replaced with a production ready solution, as setting environment variables through a .env file is not practical long term. The following variables are tracked in the .env file:
- db_var: Stores the current database. This is set by the `change_db` function under src/commands/db_cmds/persistent.rs and can be executed by running the command `tmgr database set {db_name}`. 
- db_dir: Stores the save location of databases for the program. This can be set by the user by running the command `tmgr database set-directory {path}`.