mod opts;
mod ipc_reader;

use std::{
    io::{BufReader, stdout, prelude::*},
    os::unix::net::UnixStream,
    env
};

use ipc_reader::IPCReader;

fn main() -> anyhow::Result<()> {
    let hypr_sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;
    let stream = UnixStream::connect(format!("/tmp/hypr/{}/.socket2.sock", hypr_sig))?;
    let mut stream_reader = BufReader::new(stream);

    let curr_opts = opts::opts();
    let ipcreader = IPCReader::new(curr_opts.mode)?;

    loop {
        let mut line = String::new();
        stream_reader.read_line(&mut line)?;

        if let Some(printable_string) = ipcreader.read_line(&line)? {
            print!("\n{}", printable_string);
            stdout().flush()?;
        }
    }
}
