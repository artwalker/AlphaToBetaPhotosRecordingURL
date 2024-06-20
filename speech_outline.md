Ladies and gentlemen, welcome!

I'm delighted to have you all here today for this event. It's a great pleasure to share and exchange knowledge with such an enthusiastic audience. Today, we will discuss a crucial topic: how to enhance our productivity using modern development tools, particularly how to effectively manage and execute tasks.

## 1. Drawbacks of Not Using a TO-DO List

In our daily lives and work, many people don't develop the habit of using a TO-DO list, which can lead to several issues:

1. **Lack of direction**: Without a clear task list, we may feel lost and unsure where to start.
2. **Anxiety**: Constantly thinking about the tasks we need to complete without a specific action plan can lead to stress and anxiety.
3. **Lack of action**: Our ideas might be great, but without a clear task list, it's easy to get caught up in overthinking instead of taking action.

In reality, taking action is the key to solving problems. Once we start moving, things will start to fall into place step by step.

## 2. Self-Introduction

Before we dive into the main content, let me briefly introduce myself. My name is Ethan Wang, and I am a master's student at North China Electric Power University, majoring in Software Engineering. I am honored to be a Microsoft Student Ambassador and excited to share some useful tools and experiences with you.

## 3. Program Outline and Main Tasks

Today's talk will focus on the following sections:

1. **Program Outline**: We will create a simple CLI project using Rust.
2. **Learning Resources**: GitHub repository and Microsoft official documentation.
3. **Directory Structure**: How to organize our project files.
4. **Creating a Project with Cargo**: Quickly set up a Rust project.
5. **Introduction to Cargo.toml**: The project configuration file.
6. **Introduction to cli.rs**: Handling the command-line interface.
7. **Introduction to tasks.rs**: Defining our task logic.
8. **Introduction to main.rs**: The main entry point of the program.

Let's go through each part step by step.

## 4. Introduction to the Directory Structure

Before we start coding, we need to understand the project directory structure. A typical Rust project directory looks like this:

```
./rusty-journal/
├── Cargo.toml
├── src
 ├── cli.rs
 ├── main.rs
 └── tasks.rs
```

Each of these files serves a different purpose, which we will cover shortly.

## 5. Creating a Project with Cargo

First, we need to use the cargo command to create a new Rust project. You can run the following command in your terminal:

```sh
$ cargo new rusty-journal
$ cd rusty-journal
```

This will create a new Rust project and generate the necessary files and directories.

## 6. Introduction to Cargo.toml

The `Cargo.toml` file is the configuration file for Rust projects. In this file, we can specify the project's  dependencies, version information, and more. A simple `Cargo.toml` file might look like this:

```toml
[package]
name = "rusty-journal"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0"
home = "0.5"
serde_json = "1.0"
structopt = "0.3"

[dependencies.chrono]
features = ["serde"]
version = "0.4"

[dependencies.serde]
features = ["derive"]
version = "1.0"
```

Here, we have included the `clap` library, which is used for parsing command-line arguments.

## 7. Introduction to cli.rs

The `cli.rs` file is used to handle the command-line interface. In this file, we can define command-line arguments and options. For example:

```rust
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
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

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
```

## 8. Introduction to tasks.rs

The `tasks.rs` file is used to define our task logic. Here we can define functions to handle various tasks. For example:

```rust
pub fn perform_task(task: &str) {
    match task {
        "hello" => println!("Hello, world!"),
        _ => println!("Unknown task: {}", task),
    }
}
```

## 9. Introduction to main.rs

Finally, let's look at the `main.rs` file, which is the main entry point of the program. In this file, we combine the previous `cli.rs` and `tasks.rs` to implement the overall CLI program logic:

```rust
mod cli;
mod tasks;

fn main() {
    let matches = cli::build_cli().get_matches();
    if let Some(task) = matches.value_of("task") {
        tasks::perform_task(task);
    }
}
```

With these steps, we have completed a simple CLI project. I hope today's sharing will help you better manage and execute your tasks, improving your efficiency in study and work.

## 10. Conclusion

Today, we discussed the drawbacks of not using a TO-DO list and demonstrated how to create a CLI application using Rust and related tools. I hope you will actively practice what you've learned and continually enhance your skills.

Thank you all for listening! If you have any questions, feel free to ask. We can also access more resources through GitHub and Microsoft official documentation.

Once again, thank you all for participating!

---
