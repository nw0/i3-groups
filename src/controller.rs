use std::sync::Mutex;

use i3ipc::{reply::Workspace, I3Connection};

#[derive(Debug)]
struct WorkspaceName {
    group: Option<usize>,
    number: usize,
}

impl WorkspaceName {
    fn from_name(name: &str) -> WorkspaceName {
        let fields = name.split(":").collect::<Vec<&str>>();
        let (group, number) = match fields.len() {
            2 => (
                Some(fields[0].parse::<usize>().expect("group is not a number")),
                fields[1].parse::<usize>().expect("local is not a number"),
            ),
            1 => (
                None,
                fields[0].parse::<usize>().expect("global is not a number"),
            ),
            _ => panic!("failed to parse workspace name"),
        };
        WorkspaceName {
            group: group,
            number: number,
        }
    }

    fn from_workspace(workspace: &Workspace) -> WorkspaceName {
        WorkspaceName::from_name(&workspace.name)
    }

    fn title(&self) -> String {
        match self.group {
            None => format!("{}", self.number),
            Some(group) => format!("{}:{}", group, self.number),
        }
    }
}

pub struct Controller {
    i3_connection: Mutex<I3Connection>,
    workspaces: Option<Vec<Workspace>>,
}

impl Controller {
    pub fn new(i3_connection: I3Connection) -> Controller {
        Controller {
            i3_connection: Mutex::new(i3_connection),
            workspaces: None,
        }
    }

    fn send_i3_command(&self, command: &str) {
        self.i3_connection
            .lock()
            .unwrap()
            .run_command(command)
            .expect("failed to execute command");
    }

    fn get_workspaces(&mut self) -> &[Workspace] {
        self.workspaces = Some(
            self.i3_connection
                .lock()
                .unwrap()
                .get_workspaces()
                .expect("failed to get workspaces")
                .workspaces,
        );
        self.workspaces.as_ref().unwrap()
    }

    fn get_visible_workspaces(&mut self) -> Vec<&Workspace> {
        self.get_workspaces()
            .iter()
            .filter(|w| w.visible)
            .collect::<Vec<_>>()
    }

    fn get_focused_workspace(&mut self) -> &Workspace {
        self.get_workspaces()
            .iter()
            .find(|w| w.focused)
            .expect("no focused workspace")
    }

    fn switch_workspace(&self, workspace: &WorkspaceName) {
        self.send_i3_command(
            format!("workspace --no-auto-back-and-forth {}", workspace.title()).as_ref(),
        );
    }

    fn move_to_workspace(&self, workspace: &WorkspaceName) {
        // FIXME: no-auto-back-and-forth?
        self.send_i3_command(format!("move container to workspace {}", workspace.title()).as_ref());
    }

    pub fn focus_workspace(&mut self, number: Option<usize>) {
        let mut focused = WorkspaceName::from_workspace(self.get_focused_workspace());
        focused.number = number.expect("must specify workspace number");
        self.switch_workspace(&focused);
    }

    // Re-focus the current workspace to a new group, retaining local number
    pub fn focus_group(&mut self, group: Option<usize>) {
        let target_group = group.expect("must explicitly specify group");
        let mut focused = WorkspaceName::from_workspace(self.get_focused_workspace());
        focused.group = match target_group {
            0 => None,
            x => Some(x),
        };
        self.switch_workspace(&focused);
    }

    // Only re-focus workspaces not belonging to the default group
    // Re-focus to the same- or lower-numbered workspace of the target group
    pub fn focus_group_all(&mut self, group: Option<usize>) {
        let visible_vec = self.get_visible_workspaces();
        let focused = visible_vec
            .iter()
            .find(|w| w.focused)
            .expect("no focused workspace")
            .name
            .clone();
        let visible = visible_vec
            .iter()
            .map(|w| w.name.clone())
            .collect::<Vec<_>>();

        for workspace in visible {
            if workspace != focused {
                self.switch_workspace(&WorkspaceName::from_name(&workspace));
                self.focus_group(group);
            }
        }
        self.switch_workspace(&WorkspaceName::from_name(&focused));
        self.focus_group(group);
    }

    pub fn move_container_to_workspace(&mut self, number: Option<usize>) {
        let mut focused = WorkspaceName::from_workspace(self.get_focused_workspace());
        focused.number = number.expect("must specify workspace number");
        self.move_to_workspace(&focused);
    }

    pub fn move_container_to_group(&mut self, group: Option<usize>) {
        let target_group = group.expect("must explicitly specify group");
        let mut focused = WorkspaceName::from_workspace(self.get_focused_workspace());
        focused.group = match target_group {
            0 => None,
            x => Some(x),
        };
        self.move_to_workspace(&focused);
    }
}
