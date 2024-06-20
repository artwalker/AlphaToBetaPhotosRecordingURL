use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

// This function is used to find the default journal file.
fn find_default_journal_file() -> Option<PathBuf> {
    // The home_dir function from the home crate is used to get 
    // the home directory of the current user.
    home::home_dir().map(|mut path| {
        // The push method is used to append ".rust-journal.json" to the path.
        path.push(".rust-journal.json");
        // The updated path is returned.
        path
    })
}

/* 
* It first parses the command line arguments, then performs the corresponding actions 
* based on the parsed arguments. Possible actions include adding a new task, l
* isting all tasks, and completing a task. All actions are implemented through 
* an operation log file, the location of which can be specified via command line arguments; 
* if not specified, the default location is used.
*/
// Define the main function that returns a Result
fn main() -> anyhow::Result<()> {
    // Parse command line arguments into a CommandLineArgs struct
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Determine the journal file to use, either from the command line argument or a default location
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    // Perform the action specified by the command line arguments
    match action {
        // If the action is Add, add a new task
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        // If the action is List, list all tasks
        List => tasks::list_tasks(journal_file),
        // If the action is Done, mark a task as complete
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    
    // Return Ok if everything succeeded
    Ok(())
}