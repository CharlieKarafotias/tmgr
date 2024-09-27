# Task Manager CLI: `tmgr`

A command line tool to track tasks, designed for those that live in IDEs.

### Key features include:

- CRUD operations for tasks
- Duration tracking for tasks
- More to come in the future! Think Jira integration, labels, linking tasks to lines in code, etc.!

## Getting Started

### Installation

_DISCLAIMER_: There are several ways to install and start using `tmgr`. As I developed this on macOS, the directions
provided below have only been tested on macOS and may not work as well on other platforms. I look to expand on this in
the future as the project matures.

- **macOS - downloading the binary from latest release (use if you do not have/want to install Rust)**:
    - Download the latest release from GitHub repository. The file to download is under assets named `tmgr`.
        - [Release page (latest)](https://github.com/CharlieKarafotias/tmgr/releases/latest)
    - In the downloads folder, you should see a new file named `tmgr`
    - Open terminal and run the command `cd Downloads && chmod 751 ./tmgr`. This command will move your directory to the
      downloads folder and make `tmgr` an executable program
    - Move `tmgr` program to one of your computer's bin folders. For info on what a bin
      is, [see this resource](https://apple.stackexchange.com/questions/99788/os-x-create-a-personal-bin-directory-bin-and-run-scripts-without-specifyin)
      on creating a personal bin and setting the path for your terminal.
    - Once set up, open a terminal and run `tmgr`. If successful, you should see all the available commands listed by
      default as the response.
- **macOS - building from source**:
    - Requirements:
        - [Install Rust](https://www.rust-lang.org/tools/install) (if not installed)
        - [Install Git](https://git-scm.com/downloads) (if not installed)
    - Go to [tmgr repository](https://github.com/CharlieKarafotias/tmgr/) and clone it to your computer.
      Alternatively, run `git clone https://github.com/CharlieKarafotias/tmgr.git`
    - Open terminal and change directory to the `tmgr` folder
    - Run `cargo build -r`. This will build a release version of the project for your system.
    - Once complete, the release version should be in at `tmgr/target/release/tmgr`.
    - Move the `tmgr` binary to one of your computer's bin folders.
    - Once set up, open a terminal and run `tmgr`. If successful, you should see all the available commands listed out.

### Development Setup

If you wish to change the source code, you can follow the steps below:

1. Install Rust on your computer. For instructions on installing Rust, see
   the [Rust installation guide](https://www.rust-lang.org/tools/install).
2. Pull this repository
3. Run `cargo run`. This should compile `tmgr` and then return all the commands that are available for the program.
4. To run a specific command, such as adding a new task, use the following command `cargo run -- add "my first todo"`
5. To build an optimized release of the project, run `cargo build -r`
6. If you wish to commit to this repo, before opening a pull request,
   ensure [Pre-commit](https://pre-commit.com/#install) is installed on your computer. This project utilizes pre-commit
   to ensure consistency throughout commits.

### Release a change

If you wish to release a new version of `tmgr`, follow the steps below:
1. Ensure [Pre-commit](https://pre-commit.com/#install) is installed on your computer
2. Open a pull request on the `tmgr` repository
3. Review the PR and merge to `main` branch locally 
4. When the merge occurs locally, the pre-commit script will run. This runs `cargo build -r && gh release create`

## CLI Commands

`tmgr` has the following commands:

### Command Reference

| Command Name | Description                                                         |
|--------------|---------------------------------------------------------------------|
| add          | adds a new task                                                     |
| complete     | marks a task as complete                                            |
| delete       | deletes a task                                                      |
| list         | lists tasks                                                         |
| status       | info regarding file locations, current database, general statistics |
| update       | updates an existing task                                            |
| upgrade      | upgrades `tmgr` to the latest version                               |
| view         | shows all information about a specific task                         |
| help         | prints out CLI usage information                                    |


### Add Command

The `add` command will add a new task.

#### Usage

- `tmgr add <Name> [Priority] [Description]`
    - Note: Priority has 3 possible values: `low`, `medium`, and `high`.
    - If no priority is provided, it will default to `high`.
- `tmgr add 'The most basic task... just a name is set'`
- `tmgr add 'Read AWS document' 'low' 'Read the concurrent execution section of the lambda documentation'`
    - Where `Read AWS document` is the name of the task
    - Where `low` is the priority of the task
    - Where `Read the concurrent execution section of the lambda documentation` is the description of the task

### Complete Command

The `complete` command will mark a task as complete.

#### Usage

- `tmgr complete <ID>`
- `tmgr complete '1w08w2'`
    - Where `1w08w2` is the beginning part of an existing task ID. To find task IDs, run `tmgr list`

### Delete Command

The `delete` command will delete a task.

#### Usage

- `tmgr delete <ID>`
- `tmgr delete '1w08w2'`
    - Where `1w08w2` is the beginning part of an existing task ID. To find task IDs, run `tmgr list`

### List Command

Lists all tasks. By default, this will only list in-progress tasks. This provides general information about the tasks
like the name, priority, and description.

#### Usage 

- `tmgr list`
- `tmgr list -a`
    - List all tasks (includes completed tasks)

### Status Command

The `status` command will show information regarding the current state & location of the database and information about
the location of the binary.

#### Usage

- `tmgr status`

### Update Command

The `update` command will update information about a particular task.

#### Usage

- `tmgr update <ID> [Name] [Priority] [Description]`
    - Note: Priority has 3 possible values: `low`, `medium`, and `high`.
- `tmgr update '1w08w2' 'Read AWS document' 'low' 'Read the concurrent execution section of the lambda documentation'`
    - Updates the name, priority, and description of the task starting with ID `1w08w2`.

### Upgrade Command

The `upgrade` command will update `tmgr` to the latest version. The command works in the following steps:

1. Checks the latest release version on the GitHub repository
2. Checks if a version update is needed
3. If needed, the latest release is downloaded from GitHub repository and stored in the system's downloads folder.
4. Locate where the current `tmgr` executable is stored
5. Delete the current `tmgr` executable
6. Move the latest `tmgr` executable from the downloads folder to the folder where the current `tmgr` executable was
   just deleted from.

#### Usage

- `tmgr upgrade`

### View Command

The `view` command will show all information about a specific task.

#### Usage

- `tmgr view <ID>`
- `tmgr view '1w08w2'`
    - Where `1w08w2` is the beginning part of an existing task ID. To find task IDs, run `tmgr list`

### Help Command

The help command will present all available subcommands that `tmgr` supports. Further, help can be used within other
subcommands to learn more about each commands' functionality.

#### Usage

- `tmgr -h`
- `tmgr --help`
