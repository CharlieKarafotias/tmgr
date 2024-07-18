use std::{
    env::current_exe,
    fmt,
    fs::{File, OpenOptions},
    io::{self, BufRead, Read, Seek, Write},
    path::Path,
};

// --- State Manager ---
pub struct State {
    db_dir: Option<String>,
    db_var: Option<String>,
    path: String,
}

impl State {
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_db_dir(&self) -> Option<String> {
        self.db_dir.clone()
    }

    pub fn get_db_name(&self) -> Option<String> {
        self.db_var.clone()
    }

    pub fn new(path: Option<&Path>) -> Result<Self, StateManagerError> {
        let mut config_path = current_exe().map_err(|e| StateManagerError {
            kind: StateManagerErrorKind::IoError,
            message: e.to_string(),
        })?;
        config_path.pop(); // remove executable name
        config_path.push("tmgr_config.toml");
        let default_path = Path::new(&config_path);

        let f = match path {
            Some(p) => p,
            None => default_path,
        };

        let path_as_str = f.to_str().ok_or(StateManagerError {
            kind: StateManagerErrorKind::StringConversionError,
            message: "Failed to convert path to string".to_string(),
        })?;

        let mut state_res = Self {
            db_dir: None,
            db_var: None,
            path: path_as_str.to_string(),
        };

        let file_exists = f.try_exists().map_err(|e| StateManagerError {
            kind: StateManagerErrorKind::ConfigFileNotFound,
            message: format!("State manager config file not found: {}", e),
        })?;

        if file_exists {
            // read in values
            let lines = read_lines(f).map_err(|e| StateManagerError {
                kind: StateManagerErrorKind::IoError,
                message: format!("Failed to read lines from config file: {}", e),
            })?;

            // Consumes the iterator, returns an (Optional) String
            for line in lines.map_while(|x| x.ok()) {
                match line.split_once('=') {
                    Some((key, value)) => match key.trim() {
                        "db_dir" => {
                            if value.trim() == "\"\"" {
                                state_res.db_dir = None
                            } else {
                                state_res.db_dir = Some(value.trim().replace('\"', "").to_string());
                            }
                        }
                        "db_var" => {
                            if value.trim() == "\"\"" {
                                state_res.db_var = None
                            } else {
                                state_res.db_var = Some(value.trim().replace('\"', "").to_string());
                            }
                        }
                        _ => {
                            return Err(StateManagerError {
                                kind: StateManagerErrorKind::UnknownStateVariable,
                                message: format!(
                                    "State manager encountered variable not being tracked {} = {}",
                                    key, value
                                ),
                            })
                        }
                    },
                    None => {
                        return Err(StateManagerError {
                            kind: StateManagerErrorKind::InvalidConfigFileStructure,
                            message: "Invalid config file structure".to_string(),
                        })
                    }
                }
            }
        } else {
            let mut file = File::create(f).map_err(|e| StateManagerError {
                kind: StateManagerErrorKind::IoError,
                message: format!("Failed to create config file: {}", e),
            })?;

            let initial_values: &str = "db_dir = \"\"\ndb_var = \"\"\n";

            file.write_all(initial_values.as_bytes())
                .map_err(|e| StateManagerError {
                    kind: StateManagerErrorKind::IoError,
                    message: format!("Failed to write to config file: {}", e),
                })?;
            println!("successfully wrote to {:#?}", &f);

            // update state - empty string is None
            state_res.db_dir = None;
            state_res.db_var = None;
        }
        Ok(state_res)
    }

    pub fn update_var(&mut self, key: &str, value: &str) -> Result<(), StateManagerError> {
        // load in file
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .map_err(|e| StateManagerError {
                kind: StateManagerErrorKind::IoError,
                message: format!("Failed to open config file: {}", e),
            })?;

        let mut buf: String = String::new();
        f.read_to_string(&mut buf).map_err(|e| StateManagerError {
            kind: StateManagerErrorKind::IoError,
            message: format!("Failed to read from config file: {}", e),
        })?;

        // update state
        match key {
            "db_dir" => {
                self.db_dir = Some(value.to_string());
                update_var_in_file(f, buf, "db_dir".to_string(), value.to_string()).map_err(
                    |e| StateManagerError {
                        kind: StateManagerErrorKind::IoError,
                        message: format!("Failed to write to config file: {}", e),
                    },
                )?;
                Ok(())
            }
            "db_var" => {
                self.db_var = Some(value.to_string());
                update_var_in_file(f, buf, "db_var".to_string(), value.to_string()).map_err(
                    |e| StateManagerError {
                        kind: StateManagerErrorKind::IoError,
                        message: format!("Failed to write to config file: {}", e),
                    },
                )?;
                Ok(())
            }
            _ => Err(StateManagerError {
                kind: StateManagerErrorKind::UpdateVariableFailed,
                message: format!(
                    "state manager encountered variable not being tracked {}",
                    key
                ),
            }),
        }
    }
}

// --- Helper Functions ---

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
#[allow(unused)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn update_var_in_file(
    mut f: File,
    mut buf: String,
    var_name: String,
    new_val: String,
) -> Result<(), io::Error> {
    // Find the start position of the "var_name" variable (var_name=)
    if let Some(idx) = buf.find(&var_name) {
        // Update the variable to new_value

        // Find the end position of the variable (\n)
        let idx_of_end_of_line = buf[idx..].find('\n').unwrap() + idx;

        // Update the variable with the new_value
        buf.replace_range(
            idx..idx_of_end_of_line,
            &format!("{} = {}", var_name, pad_sides(&new_val)),
        );
    }
    f.seek(std::io::SeekFrom::Start(0))?;
    f.write_all(buf.as_bytes())?;
    f.set_len(buf.len() as u64)?;
    f.flush()?;
    Ok(())
}

/// pad sides with the char '\"'
fn pad_sides(s: &str) -> String {
    let mut s = s.to_string();
    s.insert(0, '\"');
    s.push('\"');
    s
}

// --- StateManagerError ---
#[derive(Debug)]
pub struct StateManagerError {
    kind: StateManagerErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum StateManagerErrorKind {
    ConfigFileNotFound,
    InvalidConfigFileStructure,
    IoError,
    StringConversionError,
    UpdateVariableFailed,
    UnknownStateVariable,
}

impl fmt::Display for StateManagerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StateManagerErrorKind::UpdateVariableFailed => {
                write!(f, "state manager failed to update variable in config file")
            }
            StateManagerErrorKind::IoError => write!(f, "io error occurred"),
            StateManagerErrorKind::StringConversionError => write!(f, "string conversion error"),
            StateManagerErrorKind::ConfigFileNotFound => write!(f, "config file not found"),
            StateManagerErrorKind::UnknownStateVariable => write!(f, "unknown state variable"),
            StateManagerErrorKind::InvalidConfigFileStructure => {
                write!(f, "invalid config file structure")
            }
        }
    }
}

impl fmt::Display for StateManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (state manager error: {})", self.message, self.kind)
    }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_new_state() {
        // Path
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let f = temp_file.path();

        // new state with temp path
        let state = State::new(Some(f)).unwrap();

        // Verify that db_dir and db_var are initially None
        assert_eq!(state.db_dir, None);
        assert_eq!(state.db_var, None);
        assert_eq!(state.path, f.to_str().unwrap().to_string());
    }

    #[test]
    fn test_new_state_existing_file() {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        // Write initial values to the file
        let initial_values: &str = "db_dir = \"test1\"\ndb_var = \"test2\"\n";
        temp_file.write_all(initial_values.as_bytes()).unwrap();

        let f = temp_file.path();
        let state = State::new(Some(f)).unwrap();

        assert_eq!(state.db_dir, Some("test1".to_string()));
        assert_eq!(state.db_var, Some("test2".to_string()));
        assert_eq!(state.path, f.to_str().unwrap().to_string());
    }
}
