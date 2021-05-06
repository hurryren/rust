use structopt::StructOpt;
mod cli;
mod task;

use cli::{Action::*, CommandLineArgs};
use task::Task;

fn main() {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file.expect("Failed to find journal file");

    // Perform the action.
    match action {
        Add { text } => task::add_task(journal_file, Task::new(text)),
        List => task::list_tasks(journal_file),
        Done { position } => task::complete_task(journal_file, position),
    }
    .expect("Failed to perform action")
}
