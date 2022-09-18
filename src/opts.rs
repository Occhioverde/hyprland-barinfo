use bpaf::{command, construct, pure, Parser};

#[derive(Debug, Clone)]
pub enum ExecMode {
    Workspace,
    ActiveWindow,
}

#[derive(Debug, Clone)]
pub struct Opts {
    pub mode: ExecMode,
}

pub fn opts() -> Opts {
    let workspace_command_cnt = pure(())
        .to_options()
        .descr("Track the current workspace");
    let activewindow_command_cnt = pure(())
        .to_options()
        .descr("Track the current active window");

    let workspace_command = command(
        "workspace",
        workspace_command_cnt,
    )
    .map(|_| ExecMode::Workspace);
    let activewindow_command = command(
        "activewindow",
        activewindow_command_cnt,
    )
    .map(|_| ExecMode::ActiveWindow);

    let mode = construct!([workspace_command, activewindow_command]);

    construct!(Opts { mode })
        .to_options()
        .descr("Eww driver for Hyperland")
        .run()
}
