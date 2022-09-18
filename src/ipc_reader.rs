use regex::Regex;

use crate::opts::ExecMode;

pub struct IPCReader {
    mode: ExecMode,
    re: Regex
}

impl IPCReader {
    pub fn new(mode: ExecMode) -> anyhow::Result<IPCReader> {
        let re;
        match mode {
            ExecMode::ActiveWindow => {
                re = Regex::new("^activewindow>>(.*),(.*)")?;
            },
            ExecMode::Workspace => {
                re = Regex::new("^workspace>>(.*)")?;
            },
        };

        Ok(IPCReader { mode, re })
    }
    pub fn read_line<'a>(&self, line: &'a str) -> anyhow::Result<Option<&'a str>> {
        match self.mode {
            ExecMode::ActiveWindow => {
                if let Some(groups) = self.re.captures(&line) {
                    return Ok(Some(groups.get(2).ok_or(anyhow::anyhow!("Cannot read capture from second group"))?.as_str()));
                } else {
                    return Ok(None);
                }
            }
            ExecMode::Workspace => {
                if let Some(groups) = self.re.captures(&line) {
                    return Ok(Some(groups.get(1).ok_or(anyhow::anyhow!("Cannot read capture from first group"))?.as_str()));
                } else {
                    return Ok(None);
                }
            }
        }
    }
}
