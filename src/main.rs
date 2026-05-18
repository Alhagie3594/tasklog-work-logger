use clap::{Parser, Subcommand};
use chrono::prelude::*;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tasklog")]
#[command(about = "Log your daily work tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { description: String },
    /// Show today's tasks
    Today,
    /// Show all tasks
    List,
    /// Remove last task
    Undo,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Task {
    time: String,
    text: String,
}

fn get_file_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let mut path = PathBuf::from(home);
    path.push(".tasklog.json");
    path
}

fn read_tasks() -> Vec<Task> {
    let path = get_file_path();
    if !path.exists() {
        return Vec::new();
    }
    let content = read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
}

fn write_tasks(tasks: &[Task]) {
    let path = get_file_path();
    let json = serde_json::to_string_pretty(tasks).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Add { description } => {
            let now = Local::now();
            let time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut all = read_tasks();
            all.push(Task { time: time_str, text: description });
            write_tasks(&all);
            println!("✓ Task saved");
        }
        Commands::Today => {
            let today = Local::now().format("%Y-%m-%d").to_string();
            let all = read_tasks();
            let today_tasks: Vec<_> = all.iter()
                .filter(|t| t.time.starts_with(&today))
                .collect();
            
            if today_tasks.is_empty() {
                println!("No tasks for today");
            } else {
                println!("Today's tasks:");
                for task in today_tasks {
                    println!("  • {}", task.text);
                }
            }
        }
        Commands::List => {
            let all = read_tasks();
            if all.is_empty() {
                println!("No tasks logged");
            } else {
                println!("All tasks:");
                for task in all {
                    println!("  [{}] {}", task.time, task.text);
                }
            }
        }
        Commands::Undo => {
            let mut all = read_tasks();
            if all.is_empty() {
                println!("Nothing to undo");
            } else {
                let removed = all.pop().unwrap();
                write_tasks(&all);
                println!("Removed: {}", removed.text);
            }
        }
    }
}
