mod opts;
mod ipc_reader;
mod ws_renderer;

use std::{
    os::unix::net::UnixStream,
    env
};

use ipc_reader::IPCReader;

fn main() -> anyhow::Result<()> {
    let xdg_runtime_dir = xdg::BaseDirectories::new()?
        .get_runtime_directory()?
        .clone()
        .as_os_str().to_str()
        .map(|path_str| path_str.to_string())
        .ok_or(anyhow::anyhow!("Couldn't convert XDG Runtime Path to String"))?;
    let hypr_sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;

    let event_socket = UnixStream::connect(format!("{}/hypr/{}/.socket2.sock", xdg_runtime_dir, hypr_sig))?;
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
            let event_socket = UnixStream::connect(format!("{}/hypr/{}/.socket2.sock", xdg_runtime_dir, hypr_sig))?;
            // Reset the IPC reader
            ipcreader = IPCReader::new(curr_opts.mode.clone(), event_socket)?;

            // Resume printing
            println!("{}", ipcreader.force_out());
        }
    }
}
