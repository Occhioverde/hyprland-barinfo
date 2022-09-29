use std::{str::FromStr, io::{prelude::*, BufReader}, process::Command, os::unix::net::UnixStream};
use regex::Regex;
use serde::Deserialize;

use crate::{opts::ExecMode, ws_renderer::render_workspaces_bar};

pub struct IPCReader {
    mode: ExecMode,
    regexes: Vec<Regex>,
    event_socket_reader: BufReader<UnixStream>,
    monitor_name: Option<String>,
    workspaces: Vec<Workspace>
}

#[derive(Deserialize, Clone, Debug)]
struct Monitor {
    name: String,
    focused: bool,
    #[serde(alias = "activeWorkspace")]
    workspace: Workspace
}

#[derive(Deserialize, Clone, Debug)]
pub struct Workspace {
    pub id: usize,
    #[serde(default)]
    monitor: String, // Useful only when getting workspaces list
    #[serde(skip)]
    pub status: usize // Internal field
}

impl IPCReader {
    pub fn new(mode: ExecMode, event_socket: UnixStream) -> anyhow::Result<IPCReader> {
        let mut regexes = Vec::new();
        let mut monitor_name = None;
        let mut workspaces = Vec::new();
        match mode {
            ExecMode::ActiveWindow => {
                regexes.push(Regex::new("^activewindow>>(.*),(.*)")?);
            },
            ExecMode::Workspace(my_mon) => {
                regexes.push(Regex::new("^workspace>>(.*)")?);
                regexes.push(Regex::new("^createworkspace>>(.*)")?);
                regexes.push(Regex::new("^destroyworkspace>>(.*)")?);
                regexes.push(Regex::new("^focusedmon>>(.*),(.*)")?);

                IPCReader::get_monitors()?.iter().enumerate().for_each(|(i, monitor)| {
                    let mut ws = monitor.workspace.clone();
                    if i == my_mon {
                        monitor_name = Some(monitor.name.clone());
                        if monitor.focused {
                            ws.status = 1;
                        } else {
                            ws.status = 3;
                        }
                    } else {
                        ws.status = 0;
                    }

                    workspaces.push(ws);
                });
            },
        };

        let event_socket_reader = BufReader::new(event_socket);

        Ok(IPCReader { mode, regexes, event_socket_reader, monitor_name, workspaces })
    }

    fn get_monitors() -> anyhow::Result<Vec<Monitor>> {
        let monitors = serde_json::from_str::<Vec<Monitor>>(
            std::str::from_utf8(&Command::new("hyprctl")
                                .args([ "-j", "monitors" ])
                                .output()?
                                .stdout)?
        )?;

        Ok(monitors)
    }

    fn get_workspaces() -> anyhow::Result<Vec<Workspace>> {
        let workspaces = serde_json::from_str::<Vec<Workspace>>(
            std::str::from_utf8(&Command::new("hyprctl")
                                .args([ "-j", "workspaces" ])
                                .output()?
                                .stdout)?
        )?;

        Ok(workspaces)
    }

    pub fn force_out(&self) -> String {
        match self.mode {
            ExecMode::Workspace(_) => render_workspaces_bar(&self.workspaces),
            ExecMode::ActiveWindow => "Welcome!".to_string()
        }
    }

    pub fn read(&mut self) -> anyhow::Result<Option<String>> {
        let mut line: String = String::new();
        self.event_socket_reader.read_line(&mut line)?;

        match self.mode {
            ExecMode::Workspace(_) => {
                if let Some(groups) = self.regexes[0].captures(&line) {
                    // Change WS (workspace>>)
                    let current_ws_id = groups.get(1).map(|ws_id| usize::from_str(ws_id.as_str())).ok_or(anyhow::anyhow!("Unexpeced workspace index value"))??;
                    let current_ws = self.workspaces.iter().find(|ws| ws.id == current_ws_id).ok_or(anyhow::anyhow!("The workspace is not in internal list!"))?;

                    if current_ws.status != 0 {
                        self.workspaces = self.workspaces.iter_mut().map(|ws| {
                            if ws.status == 1 {
                                ws.status = 2;
                            } else if ws.status == 3 {
                                ws.status = 2;
                            } else if ws.id == current_ws_id {
                                ws.status = 1;
                            }
                            ws.to_owned()
                        }).collect();
                    } else {
                        self.workspaces = self.workspaces.iter_mut().map(|ws| {
                            if ws.status == 1 {
                                ws.status = 3;
                            }
                            ws.to_owned()
                        }).collect();
                    }

                    return Ok(Some(render_workspaces_bar(&self.workspaces)));
                } else if let Some(groups) = self.regexes[1].captures(&line) {
                    // Create a WS (createworkspace>>)
                    let new_ws_id = groups.get(1).map(|ws_id| usize::from_str(ws_id.as_str())).ok_or(anyhow::anyhow!("Unexpeced workspace index value"))??;
                    let workspaces = IPCReader::get_workspaces()?;
                    let mut new_ws = workspaces.iter().find(|ws| ws.id == new_ws_id).ok_or(anyhow::anyhow!("The workspace is not in Hyprland list!"))?.to_owned();

                    if new_ws.monitor.eq(self.monitor_name.as_ref().unwrap()) {
                        new_ws.status = 2;
                    } else {
                        new_ws.status = 0;
                    }

                    self.workspaces.push(new_ws);

                    return Ok(Some(render_workspaces_bar(&self.workspaces)));
                } else if let Some(groups) = self.regexes[2].captures(&line) {
                    // Remove a WS (destroyworkspace>>)
                    let removed_ws_id = groups.get(1).map(|ws_id| usize::from_str(ws_id.as_str())).ok_or(anyhow::anyhow!("Unexpeced workspace index value"))??;
                    let removed_ws_with_pos = self.workspaces.iter().enumerate().find(|(_, ws)| ws.id == removed_ws_id).ok_or(anyhow::anyhow!("The workspace is not in internal list!"))?;

                    self.workspaces.remove(removed_ws_with_pos.0);

                    return Ok(Some(render_workspaces_bar(&self.workspaces)));
                } else if let Some(groups) = self.regexes[3].captures(&line) {
                    // Change monitor (focusedmon>>)
                    let curr_mon = groups.get(1).ok_or(anyhow::anyhow!("Couldn't get monitor name"))?.as_str();

                    if curr_mon.eq(self.monitor_name.as_ref().unwrap()) {
                        self.workspaces = self.workspaces.iter_mut().map(|ws| {
                            if ws.status == 3 {
                                ws.status = 1;
                            }
                            ws.to_owned()
                        }).collect();
                    } else {
                        self.workspaces = self.workspaces.iter_mut().map(|ws| {
                            if ws.status == 1 {
                                ws.status = 3;
                            }
                            ws.to_owned()
                        }).collect();
                    }

                    return Ok(Some(render_workspaces_bar(&self.workspaces)));
                }

                Ok(None)
            }
            ExecMode::ActiveWindow => {
                if let Some(groups) = self.regexes[0].captures(&line) {
                    return Ok(Some(groups.get(2).ok_or(anyhow::anyhow!("Cannot read capture from second group"))?.as_str().to_string()));
                }

                Ok(None)
            }
        }

    }
}
