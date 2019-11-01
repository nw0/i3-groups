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
    }
}
