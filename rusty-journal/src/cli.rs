use std::path::PathBuf;
use structopt::StructOpt;

/*
* Debug and StructOpt are both traits. 
* Debug is a trait in Rust's standard library used for formatting output 
* for debugging information. 
* StructOpt is a trait provided by the structopt crate, used for 
* automatically parsing command line arguments into data structures.
*/
#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /* The task description text.
        * #[structopt()] is an attribute macro provided by the structopt crate. 
        * It is used to automatically parse command line arguments into the values of a struct.
        */
        #[structopt()]
        task: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

/*
* cargo run -- --help
*/
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    // Use a different journal file.
    // short: cargo run -- -j test-journal.json add "buy a Surface Pro"
    // long: cargo run -- --journal-file test-journal.json add "buy a Surface Laptop"
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
