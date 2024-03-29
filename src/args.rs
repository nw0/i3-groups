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
    #[structopt(name = "focus-group-all")]
    FocusGroupAll {
        #[structopt(name = "group-number")]
        group_number: Option<usize>,
    },
    #[structopt(name = "focus-workspace")]
    FocusWorkspace {
        #[structopt(name = "workspace-number")]
        workspace_number: Option<usize>,
    },
    #[structopt(name = "move-container-to-workspace")]
    MoveContainerToWorkspace {
        #[structopt(name = "workspace-number")]
        workspace_number: Option<usize>,
    },
    #[structopt(name = "move-container-to-group")]
    MoveContainerToGroup {
        #[structopt(name = "group-number")]
        group_number: Option<usize>,
    },
}
