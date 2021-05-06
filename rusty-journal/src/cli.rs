use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug,StructOpt)]
pub enum Action{
    /// write tasks to the joutnal file
    Add{
        // The task description text
        #[structopt()]
        text:String,
    },
    /// remove an entry from the journal file by position
    Done{
        #[structopt()]
        postions:usize,
    },
    /// List all tasks in the journal file
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs{
    #[structopt(subcommand)]
    pub action:Action,

    /// use a different journal file.
    #[structopt(parse(from_os_str),short, long)]
    pub journal_file:Option<PathBuf>,
}