use std::fmt;

const HELP_MESSAGE: &str = r#"Usage: mkt [MODE] [TASK NAME] [[KEY] [VALUE]]...
Stores, edits, and lists tasks.

MODE OPTIONS
-c, --create    add task
-r, --read      read task
-u, --update    update task
-d, --delete    remove task
-l, --list      list all of the tasks

OTHER OPTIONS
    --help      prints this message
    --version   prints mkt's version
    
TASK KEYS
-n, --name      for editing task name
-s, --status    Todo, InProgress, Done

Examples:
'mkt --read "Do the dishes"'
'mkt --create "Make dinner" -s Todo -c 10:00pm'"#;

const USAGE_MESSAGE: &str = "Usage: mkt [MODE] [TASK NAME] [[TASK KEY] [TASK VALUE]]...\nTry 'mkt --help' for more information.";

const VERSION_MESSAGE: &str = "mkt (Manage Kai's Tasks) 0.0.1\n\
    Made by Kai Iverson from Alaska.\n\
    This is free software; see the source for copying conditions.  There is NO\n\
    warranty; not even for MERCHANTABILITY of FITNESS FOR A PARTICULAR PURPOSE.";

#[derive(Debug, PartialEq)]
pub struct Config {
    pub mode: Mode,
    pub task: Option<Task>,
    pub database_path: String,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Message(String),
    Create,
    Read,
    Update,
    Delete,
    List,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub name: String,
    pub status: Status,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Todo => "Todo",
                Status::InProgress => "InProgress",
                Status::Done => "Done",
            }
        )
    }
}

/*
impl Display for Task {
    fn fmt(F: )
}
*/

fn make_message_config(message: String) -> Config {
    Config {
        mode: Mode::Message(message),
        task: None,
        database_path: "".to_string(),
    }
}

impl Config {
    pub fn build(args: Vec<String>) -> Config {
        let mode: Mode;
        let mut task: Option<Task> = None;
        let database_path: String = "data/tasks.txt".to_string();

        let mut build_task: bool = true;

        if args.len() <= 1 {
            return make_message_config(USAGE_MESSAGE.to_string());
        }

        match &args[1][..] {
            "--help" => return make_message_config(HELP_MESSAGE.to_string()),
            "--version" => return make_message_config(VERSION_MESSAGE.to_string()),
            "-c" | "--create" => mode = Mode::Create,
            "-r" | "--read" => mode = Mode::Read,
            "-u" | "--update" => mode = Mode::Update,
            "-d" | "--delete" => mode = Mode::Delete,
            "-l" | "--list" => {
                mode = Mode::List;
                build_task = false;
            }
            invalid_option => {
                return make_message_config(format!(
                    "Invalid Option: {invalid_option}\n{USAGE_MESSAGE}"
                ))
            }
        }

        if build_task {
            let task_name: String;
            match args.get(2) {
                None => {
                    return make_message_config(format!("No task name was given!\n{USAGE_MESSAGE}"))
                }
                Some(n) => task_name = n.clone(),
            }

            task = Some(Task {
                name: task_name,
                status: Status::Todo,
            })
        }

        Config {
            mode,
            task,
            database_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_args(mode: &str) -> Vec<String> {
        let args: Vec<String> = vec![
            String::from("mkt"),
            String::from(mode),
            String::from("Example Task"),
        ];
        args
    }

    fn build_test_config(mode: Mode, has_task: bool) -> Config {
        let config: Config = Config {
            mode,
            task: match has_task {
                true => Some(Task {
                    name: "Example Task".to_string(),
                    status: Status::Todo,
                }),
                false => None,
            },
            database_path: "data/tasks.txt".to_string(),
        };
        config
    }

    #[test]
    fn test_not_enough_args() {
        let args: Vec<String> = vec![String::from("mkt")];
        let config_mode: Mode = Mode::Message(USAGE_MESSAGE.to_string());
        assert_eq!(Config::build(args).mode, config_mode);
    }

    #[test]
    fn test_help_message() {
        let args: Vec<String> = vec![String::from("mkt"), String::from("--help")];
        let config_mode: Mode = Mode::Message(HELP_MESSAGE.to_string());
        assert_eq!(Config::build(args).mode, config_mode);
    }

    #[test]
    fn test_create_task() {
        let args: Vec<String> = build_test_args("--create");
        let config: Config = build_test_config(Mode::Create, true);
        assert_eq!(Config::build(args), config);
    }

    #[test]
    fn test_read_task() {
        let args: Vec<String> = build_test_args("--read");
        let config: Config = build_test_config(Mode::Read, true);
        assert_eq!(Config::build(args), config);
    }

    #[test]
    fn test_update_task() {
        let args: Vec<String> = build_test_args("--update");
        let config: Config = build_test_config(Mode::Update, true);
        assert_eq!(Config::build(args), config);
    }

    #[test]
    fn test_delete_task() {
        let args: Vec<String> = build_test_args("--delete");
        let config: Config = build_test_config(Mode::Delete, true);
        assert_eq!(Config::build(args), config);
    }

    #[test]
    fn test_list_tasks() {
        let args: Vec<String> = build_test_args("--list");
        let config: Config = build_test_config(Mode::List, false);
        assert_eq!(Config::build(args), config);
    }
}
