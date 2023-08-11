use hyprland::data::*;
use hyprland::dispatch::Dispatch;
use hyprland::dispatch::DispatchType;
use hyprland::dispatch::WindowIdentifier;
use hyprland::dispatch::WorkspaceIdentifierWithSpecial;
use hyprland::prelude::*;
use hyprland::Result;
use hyprland::shared::Address;

use serde_json::to_string;

use std::env;
use std::ffi::OsStr;
use std::process::Command;

fn screenshot_window(active_window: &Client) -> String {
    let path = format!("/tmp/{}.png", active_window.address);

    Command::new("grim")
        .arg("-g")
        .arg(OsStr::new(&format!("{},{} {}x{}",
            active_window.at.0, active_window.at.1,
            active_window.size.0, active_window.size.1
        )))
        .arg(OsStr::new(&path))
        .output()
        .expect("Failed to take Screenshot of window");
    return path;
}

fn get_active_window() -> Client {
    Client::get_active()
        .expect("Failed to get active window info")
        .expect("Didn't get any active window info")
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments supplied!") }

    match args[1].as_str() {
        "minimize" => { 
            let active_window = get_active_window();

            println!("{}",screenshot_window(&active_window));
        
            match Dispatch::call(DispatchType::MoveToWorkspaceSilent(
                WorkspaceIdentifierWithSpecial::Special(None),
                None
            )) {
                Ok(_) => {},
                Err(e) => {print!("{}",e)}
            };
        },
        "list" => {
            let clients = Clients::get()
                .expect("Couldn't get list of clients");
            let minimized_clients: Vec<(&String, &Address)> = clients
                .iter()
                .filter(|c| c.mapped && c.workspace.name == "special")
                .map(|c| (&c.initial_title, &c.address))
                .collect();
            let json_clients = to_string(&minimized_clients).expect("Serialization failed");
            println!("{}", json_clients);
        },
        "restore" => {
            if args.len() < 3 { panic!("No window address supplied!") }
            // todo!("Impl using movetoworkspace workspace,window")
        
            match Dispatch::call(DispatchType::MoveToWorkspace(
                WorkspaceIdentifierWithSpecial::Relative(0),
                Some(WindowIdentifier::Address(
                    Address::new(
                        args[2].to_owned().split_off(2)
                    )
                    )
                )
            )) {
                Ok(_) => {},
                Err(e) => {print!("{}",e)}
            };           
        },
        _ => { panic!("Unknown command") },
    }

    Ok(())
}