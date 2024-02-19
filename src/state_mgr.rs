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
    pub fn new() -> Self {
        let mut config_path = current_exe().unwrap();
        config_path.pop(); // remove executable name
        config_path.push("tmgr_config.toml");
        let f = Path::new(&config_path);

        let mut state_res = Self {
            db_dir: None,
            db_var: None,
        };

        match f.try_exists() {
            Ok(true) => {
                println!("State manager found config, reading...");
                // read in values
                if let Ok(lines) = read_lines(&config_path) {
                    // Consumes the iterator, returns an (Optional) String
                    for line in lines.flatten() {
                        match line.split_once('=') {
                            Some((key, value)) => match key.trim() {
                                "db_dir" => {
                                    if value.trim() == "\"\"" {
                                        state_res.db_dir = None
                                    } else {
                                        state_res.db_dir = Some(value.trim().to_string());
                                    }
                                }
                                "db_var" => {
                                    if value.trim() == "\"\"" {
                                        state_res.db_var = None
                                    } else {
                                        state_res.db_var = Some(value.trim().to_string());
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
                println!("State manager config file not found, creating...");
                let mut file = match File::create(&config_path) {
                    Err(why) => {
                        panic!("couldn't create {:#?}: {}", &config_path, why)
                    }
                    Ok(file) => file,
                };

                let initial_values: &str = "db_dir = \"\"\ndb_var = \"\"\n";
                match file.write_all(initial_values.as_bytes()) {
                    Err(why) => panic!("couldn't write to {:#?}: {}", &config_path, why),
                    Ok(_) => println!("successfully wrote to {:#?}", &config_path),
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

    #[test]
    fn test_new_state() {
        // Path
        let mut config_path = current_exe().unwrap();
        config_path.pop(); // remove executable name
        config_path.push("tmgr_config.toml");
        // new state
        let state = State::new();

        // remove temp file
        std::fs::remove_file(config_path).unwrap();
        // Verify that db_dir and db_var are initially None
        assert_eq!(state.db_dir, None);
        assert_eq!(state.db_var, None);
    }

    #[test]
    fn test_new_state_existing_file() {
        // Path
        let mut config_path = current_exe().unwrap();
        config_path.pop(); // remove executable name
        config_path.push("tmgr_config.toml");
        // new state to create the file
        let state = State::new();

        // Verify that db_dir and db_var are initially None
        assert_eq!(state.db_dir, None);
        assert_eq!(state.db_var, None);

        // New state using read file
        let state2 = State::new();

        // remove temp file
        match std::fs::remove_file(&config_path) {
            Ok(_) => println!("successfully removed {:#?}", &config_path),
            Err(why) => println!("couldn't remove {:#?}: {}", &config_path, why),
        }
        assert_eq!(state2.db_dir, None);
        assert_eq!(state2.db_var, None);
    }
}
