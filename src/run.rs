use crate::config::*;

use json::object;
use std::fmt::format;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::exit;

pub fn run(config: Config) -> Result<(), String> {
    // let task = config.task.as_ref().unwrap();

    match config.mode {
        Mode::Message(message) => println!("{}", message),
        Mode::Create => create(config)?,
        Mode::Read => read(config)?,
        // Mode::Update => update(config)?,
        Mode::Delete => delete(config)?,
        Mode::List => list(config)?,
        _ => println!("Not implimented err"),
    }

    Ok(())
}

fn parse_tasks(file_path: String) -> json::JsonValue {
    let content = fs::read_to_string(file_path.clone()).unwrap();

    let tasks_parsed: json::JsonValue = json::parse(&content).unwrap_or(json::array![]);
    tasks_parsed
}

// Calculates amount of edits to transform one string into another.
// I use it to give suggestions when someone makes a typo.
fn levenshtein_distance(s1: String, s2: String) -> usize {
    let (len_s1, len_s2) = (s1.len(), s2.len());
    let mut dp: Vec<Vec<usize>> = vec![vec![0; len_s2 + 1]; len_s1 + 1];

    for i in 0..(len_s1 + 1) {
        for j in 0..(len_s2 + 1) {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] =
                    1 + std::cmp::min(dp[i - 1][j], std::cmp::min(dp[i][j - 1], dp[i - 1][j - 1]));
            }
        }
    }

    dp[len_s1][len_s2]
}

// Found task index in Ok and suggested task index in Err. Negative Err means no suggestion.
fn match_or_suggest_task(all_tasks: Vec<Task>, task: Task, threshold: usize) -> Result<usize, i32> {
    let mut ld_low_score = 5; // If its higher than 5, the words aren't similar.
    let mut ld_low_score_index = -1;
    let mut ld;

    for (i, t) in all_tasks.iter().enumerate() {
        ld = levenshtein_distance(t.name.clone(), task.name.clone());
        if ld == 0 {
            return Ok(i);
        }

        if ld < ld_low_score {
            ld_low_score = ld;
            ld_low_score_index = i as i32;
        }
    }

    if ld_low_score > threshold {
        return Err(-1);
    }

    Err(ld_low_score_index)
}

fn deserialize_tasks(tasks_parsed: json::JsonValue) -> Vec<Task> {
    let mut task_list: Vec<Task> = Vec::new();

    for index in 0..tasks_parsed.len() {
        let name = tasks_parsed[index]["name"].to_string();
        let status = &tasks_parsed[index]["status"].to_string()[..];
        let status = match status {
            "Todo" => Status::Todo,
            "InProgress" => Status::InProgress,
            "Done" => Status::Done,
            s => {
                println!("database contains a status that is invalid: {}", s);
                exit(0)
            }
        };
        task_list.push(Task { name, status });
    }

    task_list
}

fn clear_and_write_database(database_path: String, contents: String) -> Result<(), String> {
    let mut file = OpenOptions::new().write(true).open(database_path).unwrap();

    // Clear database.
    if let Err(e) = file.set_len(0) {
        return Err(format(format_args!("{}", e)).to_string());
    }

    // Write database.
    if let Err(e) = writeln!(file, "{}", contents) {
        return Err(format(format_args!("Ok don't get mad at me but I may have accidentaly deleted the entire database :(.\n Error: {}", e)));
    }

    Ok(())
}

fn create(config: Config) -> Result<(), String> {
    let task: Task = config.task.unwrap();
    let database_path: String = config.database_path;
    let mut all_tasks = parse_tasks(database_path.clone());

    let contents = object! {
        name: task.name.clone(),
        status: task.status.to_string(),
    };

    for index in 0..all_tasks.len() {
        if task.name[..] == all_tasks[index]["name"] {
            println!("task '{}' already exists!\n0 changes made.", task.name);
            exit(0)
        }
    }

    all_tasks
        .push(contents)
        .expect("json should be able to push ion know why iss akin cray cray");

    let all_tasks = json::stringify_pretty(all_tasks, 4);

    clear_and_write_database(database_path, all_tasks)
}

fn read(config: Config) -> Result<(), String> {
    let task = config.task.unwrap();
    let all_tasks = deserialize_tasks(parse_tasks(config.database_path));
    let task_index = match_or_suggest_task(all_tasks.clone(), task.clone(), 3);

    if let Ok(i) = task_index {
        println!("{:#?}", all_tasks[i]);
    } else if let Err(i) = task_index {
        println!("Task `{}` not found.", task.name);

        if i >= 0 {
            println!("Try `{}`?", all_tasks[i as usize].name);
        }
    }

    Ok(())
}

fn delete(config: Config) -> Result<(), String> {
    let database_path = config.database_path;
    let all_tasks = deserialize_tasks(parse_tasks(database_path.clone()));
    let task = config.task.expect("`config` should contain `task`");
    let task_index = match_or_suggest_task(all_tasks.clone(), task.clone(), 3);

    if let Ok(i) = task_index {
        let mut all_tasks = parse_tasks(database_path.clone());
        all_tasks.array_remove(i);

        println!("task `{}` removed.", task.name);

        let all_tasks = json::stringify_pretty(all_tasks, 4);
        return clear_and_write_database(database_path, all_tasks);
    } else if let Err(i) = task_index {
        println!("Task `{}` not found.", task.name);

        if i >= 0 {
            println!("Try `{}`?", all_tasks[i as usize].name);
        }

        println!("\n0 changes made.");
    }

    Ok(())
}

fn list(config: Config) -> Result<(), String> {
    let database_path = config.database_path;
    let all_tasks = deserialize_tasks(parse_tasks(database_path.clone()));

    println!("~~~~ Todo ~~~~");
    for task in &all_tasks {
        if task.status != Status::Todo {
            continue;
        }
        println!("{}", task.name);
    }

    println!("\n~~~~ InProgress ~~~~");
    for task in &all_tasks {
        if task.status != Status::InProgress {
            continue;
        }
        println!("{}", task.name);
    }

    println!("\n~~~~ Done ~~~~");
    for task in &all_tasks {
        if task.status != Status::Done {
            continue;
        }
        println!("{}", task.name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_and_write_database() {
        assert!(clear_and_write_database(
            "data/test.txt".to_string(),
            "Hello from the tests".to_string(),
        )
        .is_ok());

        assert_eq!(
            std::fs::read_to_string("data/test.txt").unwrap(),
            "Hello from the tests\n".to_string()
        );

        assert!(clear_and_write_database("data/test.txt".to_string(), "".to_string(),).is_ok());

        assert_eq!(
            std::fs::read_to_string("data/test.txt").unwrap(),
            "\n".to_string()
        );
    }

    #[test]
    fn test_parse_task() {}
}
