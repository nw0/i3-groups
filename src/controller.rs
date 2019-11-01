use std::sync::Mutex;

use i3ipc::{reply::Workspace,I3Connection};

pub struct Controller {
    i3_connection: Mutex<I3Connection>,
}

impl Controller {
    pub fn new(i3_connection: I3Connection) -> Controller {
        Controller { i3_connection: Mutex::new(i3_connection) }
    }

    fn send_i3_command(&self, command: &str) {
        self.i3_connection.lock().unwrap().run_command(command).expect("failed to execute command");
    }

    fn get_workspaces(&self) -> Vec<Workspace> {
        self.i3_connection.lock().unwrap().get_workspaces().expect("failed to get workspaces").workspaces
    }

    // Only re-focus workspaces not belonging to the default group
    // Re-focus to the same- or lower-numbered workspace of the target group
    pub fn refocus_group(&self, group: Option<usize>) {
        let workspaces = self.get_workspaces();
        println!("{:?}", workspaces);
    }
}
