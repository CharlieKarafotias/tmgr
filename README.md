# Todo Manager CLI (TMGR)
A command line tool to track your tasks, organized using Sqlite3 databases.

## Getting Started

### Installation 
- MacOS:
    - Download the latest release from Github repository. The file to download is named `tmgr`.
        - [Release page (v0.4.1)](https://github.com/CharlieKarafotias/tmgr/releases/tag/v0.4.1)
        - [Direct download link (v0.4.1)](https://github.com/CharlieKarafotias/tmgr/releases/download/v0.4.1/tmgr)
    - In the downloads folder, you should see a new file named `tmgr`
    - Open terminal and run the command `cd Downloads && chmod 751 ./tmgr`. This command will move your directory to the downloads folder and make `tmgr` an executable program
    - Move `tmgr` program to one of your computer's bin folders. For info on what a bin is, [see this resource](https://apple.stackexchange.com/questions/99788/os-x-create-a-personal-bin-directory-bin-and-run-scripts-without-specifyin) on creating a personal bin and setting the path for your terminal.
    - Once set up, open a terminal and run `tmgr`. If successful, you should see all the available subcommands listed by default as the response.

### Development Setup
If you wish to change the source code, you can follow the steps below:
1. Install Rust on your computer. For instructions on installing Rust, see the [Rust installation guide](https://www.rust-lang.org/tools/install).
2. Pull this repository
3. Run `cargo run`. This should compile tmgr and then return all the commands that are available for the program. 
4. To run a specific command, such as adding a new task, use the following command `cargo run -- todo add "my first todo"`
5. To build an optimized release of the project, run `cargo build -r`
6. If you wish to commit to this repo, before opening a pull request, ensure [Pre-commit](https://pre-commit.com/#install) is installed on your computer. This project utilizes pre-commit to ensure consistency throughout commits.

## CLI Commands 

`tmgr` has 6 types of commands
1. `database`
2. `init`
3. `status`
4. `todo`
5. `update`
6. `help`

### Database Commands 

Database commands manage the state of the sqlite3 database that store tasks.
- `add`: Adds a new database inside the databases directory
    - Arguments:
        - NAME: The database name
    - Example:
        - `tmgr database add 2024Tasks`
- `delete`: Deletes a database inside the databases directory (if found)
    - Arguments:
        - NAME: The database name
    - Example:
        - `tmgr database delete 2024Tasks`
- `list`: Lists all databases by name stored inside the databases directory
    - Arguments:
        - None
    - Example:
        - `tmgr database list`
- `set`: Sets the current working database (to add/remove tasks from). This must be a database located in the databases directory.
    - Arguments:
        - Name: The database name
    - Example:
        - `tmgr database set 2024Tasks`
- `set-directory`: Sets the working directory of where databases are stored. This can be used if you wish to change the default location set by `tmgr` on initialization.
    - Arguments:
        - Path: The directory path
    - Example: 
        - `tmgr database set-directory /Users/Charlie/databases`
- `help`: Prints out a helpful message showing what each subcommand does

### Init Command

Initializes tmgr for first time use. This consists of running 3 subcommands from the databases subcommand list:
- `tmgr database set-directory .`
- `tmgr database add init-db`
- `tmgr database set init-db`

If successful, a message stating _Initializer complete!_ will show.

- Arguments: 
    - No arguments
- Example: 
    - `tmgr init`

### Status Command

The status command provides the user with the internal workings of `tmgr`. This includes information about the following:
- State Manager variables: 
    - The location of the database folder
    - The current database selected
- The total amount of tasks in the current database

More data will be added as development progresses

- Arguments:
    - No arguments
- Example: 
    - `tmgr status`

### Todo Commands 

Todo commands manage the state of a task inside the current working database.
- `add`: Adds a new task
    - Arguments:
        - `Name`: A short description of the task
        - `Priority` (default high): The priority of the task [possible values: low, medium, high]
        - `Description` (optional): A long description of the task
    - Example(s):
        - `tmgr todo add "Read AWS document"`
        - `tmgr todo add "Read AWS document" low "Read the concurrent execution section of the lambda documentation"`
- `complete`: Mark a task as complete
    - Arguments:
        - `ID`: The id of the task
    - Example
        - `tmgr todo complete 1`
- `delete`: Delete a task
    - Arguments:
        - `ID`: The id of the task
    - Example
        - `tmgr todo delete 1`
- `list`: List all tasks
    - Arguments:
        - None
    - Example:
        - `tmgr todo list`
- `update`: Update an existing task
    - Arguments:
        - `ID`: The id of the task
        - `Name` (Optional): A short description of the task
        - `Priority` (Optional): The priority of the task [possible values: low, medium, high]
        - `Description` (Optional): A long description of the task
    - Example:
        - `tmgr todo update 2024Tasks`
- `help`: Prints out a helpful message showing what each subcommand does

### Update Command
This command updates `tmgr` to the latest stable version. The command works in the following steps: 
1. Checks the latest release version on the Github repository
2. Checks if a version update is needed
3. If needed, the latest release is downloaded from Github repository and stored in the system's downloads folder.
4. Locate where the current `tmgr` executable is stored 
5. Delete the current `tmgr` executable
6. Move the latest `tmgr` executable from the downloads folder to the folder where the current `tmgr` executable was just deleted from.

### Help Command 

The help command will present all available subcommands that TMGR supports. Further, help can be used within other subcommands to learn more about each commands functionality. 

Example: 
- `tmgr -h`
- `tmgr --help`

## Behind The Scenes:

### Persistent Storage

TMGR utilizes a [toml](https://toml.io/en/) file to store state between sessions. The following variables are tracked in the `tmgr_config.toml` file:
- db_var: Stores the current database. This can be set by the user by running the command `tmgr database set {database_name}`.
- db_dir: Stores the save location of databases for the program. This can be set by the user by running the command `tmgr database set-directory {path}`.