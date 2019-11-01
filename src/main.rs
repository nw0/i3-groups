mod args;
mod controller;

use i3ipc::I3Connection;
use structopt::StructOpt;

use args::{Args, Subcommands};

fn main() {
    let args = Args::from_args();
    let connection = I3Connection::connect().expect("failed to connect to i3-msg");
    let mut controller = controller::Controller::new(connection);

    match args.subcommands {
        Subcommands::FocusGroup { group_number } => {
            controller.focus_group(group_number);
        }
        Subcommands::FocusGroupAll { group_number } => {
            controller.focus_group_all(group_number);
        }
        Subcommands::FocusWorkspace { workspace_number } => {
            controller.focus_workspace(workspace_number);
        }
        Subcommands::MoveContainerToWorkspace { workspace_number } => {
            controller.move_container_to_workspace(workspace_number);
        }
        Subcommands::MoveContainerToGroup { group_number } => {
            controller.move_container_to_group(group_number);
        }
    }
}
