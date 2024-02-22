use core::panic;
use std::env::current_exe;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

#[allow(unused)]
pub struct State {
    db_dir: Option<String>,
    db_var: Option<String>,
}

impl State {
    #[allow(unused)]
    pub fn new(path: Option<&Path>) -> Self {
        let mut config_path = current_exe().unwrap();
        config_path.pop(); // remove executable name
        config_path.push("tmgr_config.toml");
        let default_path = Path::new(&config_path);

        let f = match path {
            Some(p) => p,
            None => default_path,
        };

        let mut state_res = Self {
            db_dir: None,
            db_var: None,
        };

        match f.try_exists() {
            Ok(true) => {
                // read in values
                if let Ok(lines) = read_lines(f) {
                    // Consumes the iterator, returns an (Optional) String
                    for line in lines.flatten() {
                        match line.split_once('=') {
                            Some((key, value)) => match key.trim() {
                                "db_dir" => {
                                    if value.trim() == "\"\"" {
                                        state_res.db_dir = None
                                    } else {
                                        state_res.db_dir =
                                            Some(value.trim().replace('\"', "").to_string());
                                    }
                                }
                                "db_var" => {
                                    if value.trim() == "\"\"" {
                                        state_res.db_var = None
                                    } else {
                                        state_res.db_var =
                                            Some(value.trim().replace('\"', "").to_string());
                                    }
                                }
                                _ => panic!(
                                    "State manager encountered variable not being tracked {} = {}",
                                    key, value
                                ),
                            },
                            None => panic!("Invalid state manager config file"),
                        }
                    }
                }
            }
            Ok(false) => {
                let mut file = match File::create(f) {
                    Err(why) => {
                        panic!("couldn't create {:#?}: {}", &f, why)
                    }
                    Ok(file) => file,
                };

                let initial_values: &str = "db_dir = \"\"\ndb_var = \"\"\n";
                match file.write_all(initial_values.as_bytes()) {
                    Err(why) => panic!("couldn't write to {:#?}: {}", &f, why),
                    Ok(_) => println!("successfully wrote to {:#?}", &f),
                }

                // update state - empty string is None
                state_res.db_dir = None;
                state_res.db_var = None;
            }
            Err(e) => {
                println!("ERROR: State manager config file not found: {}", e);
            }
        }
        state_res
    }
}

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
        let state = State::new(Some(f));

        // Verify that db_dir and db_var are initially None
        assert_eq!(state.db_dir, None);
        assert_eq!(state.db_var, None);
    }

    #[test]
    fn test_new_state_existing_file() {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        // Write initial values to the file
        let initial_values: &str = "db_dir = \"test1\"\ndb_var = \"test2\"\n";
        temp_file.write_all(initial_values.as_bytes()).unwrap();

        let f = temp_file.path();
        let state = State::new(Some(f));

        assert_eq!(state.db_dir, Some("test1".to_string()));
        assert_eq!(state.db_var, Some("test2".to_string()));
    }
}
