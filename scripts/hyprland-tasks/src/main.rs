use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::Result;
use hyprland::shared::Address;

use serde::{Deserialize, Serialize};

const DEFAULT_ICON: &str = "/usr/share/icons/breeze/categories/32/applications-all.svg";

#[derive(Serialize, Deserialize)]
struct Entry {
    title: String,
    class: String,
    icon: String,
    tasks: Vec<Task>
}

#[derive(Serialize, Deserialize)]
struct Task {
    title: String,
    address: Address
}

fn main() -> Result<()> {
    let clients = Clients::get()?.to_vec();
    let clients_it = clients.iter();

    let mut entries: Vec<Entry> = Vec::new();

    for client in clients_it {
        // Skip clients which shouldn't be listed (Dialogs, etc.)
        if !client.mapped {
            continue;
        }

        // Create a task for the client
        let task = Task{
            title: client.title.to_owned(),
            address: client.address.to_owned()
        };

        // If an entry for the task is already present, group it with
        // other tasks of its type
        let task_entry = entries
            .iter_mut()
            .find(|entry| entry.class == client.class);

        match task_entry {
            Some(entry) => {
                entry.tasks.push(task)
            }
            None => {
                entries.push(Entry {
                    title: client.initial_title.to_owned(),
                    class: client.class.to_owned(),
                    icon: format!("/usr/share/icons/WhiteSur/apps/scalable/{}.svg", client.class.to_owned()),
                    tasks: vec![task]
                })
            }
        }
    }

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&entries)?;

    println!("{}", j);
    Ok(())
}