use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommands: Subcommands,
}

#[derive(StructOpt, Debug)]
pub enum Subcommands {
    /// Focus a different group
    #[structopt(name = "focus-group")]
    FocusGroup {
        #[structopt(name = "group-number")]
        group_number: Option<usize>,
    },
}
