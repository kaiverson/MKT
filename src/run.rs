use crate::config::*;

use json::object;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::exit;

pub fn run(config: Config) -> Result<(), String> {
    // let task = config.task.as_ref().unwrap();

    match config.mode {
        Mode::Create => create(config)?,
        Mode::Read => read(config)?,
        // Mode::Update => update(task)?,
        // Mode::Delete => delete(task)?,
        Mode::List => list(config)?,
        _ => println!("Not implimented err"),
    }

    Ok(())
}

fn parse_tasks(file_path: String) -> json::JsonValue {
    let content = fs::read_to_string(file_path.clone()).unwrap();

    let tasks_parsed: json::JsonValue = json::parse(&content).unwrap();
    tasks_parsed
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

    let mut file = OpenOptions::new().write(true).open(database_path).unwrap();

    // TODO: only change the lines of data that have been updated.
    if let Err(e) = file.set_len(0) {
        eprintln!("{e}");
    }

    if let Err(e) = writeln!(file, "{}", all_tasks) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}

fn read(config: Config) -> Result<(), String> {
    let task = config.task.unwrap();
    let all_tasks = deserialize_tasks(parse_tasks(config.database_path));

    for t in all_tasks {
        if t.name == task.name {
            println!("{:#?}", t);
            break;
        }
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
