mod authorize;

use std::process::Command;

use nix::unistd::{setgid, setuid, Gid, Uid};

fn run() -> Result<(), String> {
    let authorized = authorize::authorize();
    if !authorized {
        return Err("Authorization failed".to_string());
    }

    // Set the user's uid and gid to the root user and return error if it fails.
    let root_uid = Uid::from_raw(0);
    let root_gid = Gid::from_raw(0);
    if setuid(root_uid).is_err() {
        return Err("Failed to setuid to root".to_string());
    }
    if setgid(root_gid).is_err() {
        return Err("Failed to setgid to root".to_string());
    }

    // Execute the command.
    let mut args = std::env::args();
    args.next(); // Skip the first argument, which is the program name.
    let _returned = Command::new(args.next().unwrap())
        .args(args)
        .status()
        .map_err(|e| e.to_string());
    return Ok(());
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
