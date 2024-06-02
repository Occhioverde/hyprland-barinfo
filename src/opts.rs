use bpaf::{construct, long, pure, Parser};

#[derive(Debug, Clone)]
pub enum ExecMode {
    Workspace(usize),
    ActiveWindow,
}

#[derive(Debug, Clone)]
pub struct Opts {
    pub mode: ExecMode,
}

pub fn opts() -> Opts {
    let monitor = long("monitor")
        .short('m')
        .help("The monitor of which you want to track the workspaces")
        .argument::<usize>("MONITOR")
        .to_options();
    let workspace_command = monitor.command("workspace")
    .help("Tracks the workspaces's status")
    .map(|monitor: usize| ExecMode::Workspace(monitor));

    let activewindow_command = pure(()).to_options().command("activewindow")
    .help("Tracks the window that is currently selected")
    .map(|_| ExecMode::ActiveWindow);

    let mode = construct!([workspace_command, activewindow_command]);

    construct!(Opts { mode })
        .to_options()
        .descr("Eww driver for Hyperland")
        .run()
}
