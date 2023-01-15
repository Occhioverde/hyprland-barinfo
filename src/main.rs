mod opts;
mod ipc_reader;
mod ws_renderer;

use std::{
    os::unix::net::UnixStream,
    env
};

use ipc_reader::IPCReader;

fn main() -> anyhow::Result<()> {
    let hypr_sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;

    let event_socket = UnixStream::connect(format!("/tmp/hypr/{}/.socket2.sock", hypr_sig))?;
    let curr_opts = opts::opts();
    let mut ipcreader = IPCReader::new(curr_opts.mode.clone(), event_socket)?;

    println!("{}", ipcreader.force_out());

    loop {
        if let Ok(read_res) = ipcreader.read() {
            if let Some(printable_string) = read_res {
                println!("{}", printable_string);
            }
        } else {
            eprintln!("Recovering from error...");

            // Drop the corrupt IPC reader
            drop(ipcreader);

            // Establish a new socket connection
            let event_socket = UnixStream::connect(format!("/tmp/hypr/{}/.socket2.sock", hypr_sig))?;
            // Reset the IPC reader
            ipcreader = IPCReader::new(curr_opts.mode.clone(), event_socket)?;

            // Resume printing
            println!("{}", ipcreader.force_out());
        }
    }
}
