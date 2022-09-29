use bpaf::{command, construct, long, pure, Parser};

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
    let workspace_command = command(
        "workspace",
        monitor,
    )
    .help("Tracks the workspaces's status")
    .map(|monitor: usize| ExecMode::Workspace(monitor));

    let activewindow_command = command(
        "activewindow",
        pure(()).to_options(),
    )
    .help("Tracks the window that is currently selected")
    .map(|_| ExecMode::ActiveWindow);

    let mode = construct!([workspace_command, activewindow_command]);

    construct!(Opts { mode })
        .to_options()
        .descr("Eww driver for Hyperland")
        .run()
}
