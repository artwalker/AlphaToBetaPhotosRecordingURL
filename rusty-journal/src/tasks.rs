use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Result, Error, ErrorKind, Seek, SeekFrom};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    // Rewind the file to the start
    file.seek(SeekFrom::Start(0))?;

    // Attempt to read tasks from the file as JSON
    let tasks = match serde_json::from_reader(file) {
        // If the file was read successfully, use the tasks
        Ok(tasks) => tasks,

        // If the file is empty (EOF), use an empty vector
        Err(e) if e.is_eof() => Vec::new(),

        // If there was an error other than EOF, return the error
        Err(e) => Err(e)?,
    };

    // Rewind the file to the start again for future reads
    file.seek(SeekFrom::Start(0))?;

    // Return the tasks
    Ok(tasks)
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file in read-write mode, or create it if it does not exist
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Collect existing tasks from the file
    let mut tasks = collect_tasks(&file)?;

    // Add the new task to the list of tasks
    tasks.push(task);

    // Write the updated list of tasks back to the file in JSON format
    serde_json::to_writer(file, &tasks)?;

    // Return Ok if everything succeeded
    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Consume file's contents as a vector of tasks.
    let mut tasks = collect_tasks(&file)?;

    // Try to remove the task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    // Write the modified task list back into the file.
    file.set_len(0)?; // clean the file.
    serde_json::to_writer(file, &tasks)?; // rewrite the file.
    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new().read(true).open(journal_path)?;
    // Parse the file and collect the tasks.
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

// Implement the Display trait for the Task struct
impl fmt::Display for Task {
    // Define how to format a Task for display
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the created_at field as a local time string
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        // Write the text and created_at fields to the Formatter
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}
