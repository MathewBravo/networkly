use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use serde_derive::{Deserialize, Serialize};

use crate::getdir;

#[derive(Serialize, Deserialize)]
struct Connections {
    connections: Vec<Connection>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Connection {
    name: String,
    email: Option<String>,
    phone: Option<String>,
    links: Links,
    strength: String,
    company: Option<String>,
    position: Option<String>,
    tags: Vec<String>,
    met: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Links {
    linkedin: Option<String>,
    website: Option<String>,
    github: Option<String>,
    twitter: Option<String>,
    facebook: Option<String>,
}

pub fn list_connections() {
    todo!()
}

pub fn list_connections_by_tag() {
    todo!()
}

pub fn build_connection() {
    print!("Creating New Connection In Network\n");
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    print!("Please Rank the Strength of the Connection (1-10): ");
    let _ = handle.flush();
    let mut stren = String::new();
    io::stdin()
        .read_line(&mut stren)
        .expect("Failed to read name");
    let _ = io::stdout().flush();

    print!("Name: ");
    let _ = handle.flush();
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    print!("Email (optional): ");
    let _ = handle.flush();
    let mut email = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read name");

    print!("Phone Number (optional): ");
    let _ = handle.flush();
    let mut phone = String::new();
    io::stdin()
        .read_line(&mut phone)
        .expect("Failed to read name");

    print!("Position (optional): ");
    let _ = handle.flush();
    let mut pos = String::new();
    io::stdin()
        .read_line(&mut pos)
        .expect("Failed to read name");

    print!("Company (optional): ");
    let _ = handle.flush();
    let mut company = String::new();
    io::stdin()
        .read_line(&mut company)
        .expect("Failed to read name");

    print!("Met At: ");
    let _ = handle.flush();
    let mut met = String::new();
    io::stdin()
        .read_line(&mut met)
        .expect("Failed to read name");

    print!("LinkedIn (Optional): ");
    let _ = handle.flush();
    let mut linked = String::new();
    io::stdin()
        .read_line(&mut linked)
        .expect("Failed to read name");

    print!("Website (Optional): ");
    let _ = handle.flush();
    let mut website = String::new();
    io::stdin()
        .read_line(&mut website)
        .expect("Failed to read name");

    print!("Github (Optional): ");
    let _ = handle.flush();
    let mut git = String::new();
    io::stdin()
        .read_line(&mut git)
        .expect("Failed to read name");

    print!("Twitter (Optional): ");
    let _ = handle.flush();
    let mut twit = String::new();
    io::stdin()
        .read_line(&mut twit)
        .expect("Failed to read name");

    print!("Facebook (Optional): ");
    let _ = handle.flush();
    let mut face = String::new();
    io::stdin()
        .read_line(&mut face)
        .expect("Failed to read name");

    let links = Links {
        linkedin: Some(linked),
        website: Some(website),
        github: Some(git),
        twitter: Some(twit),
        facebook: Some(face),
    };

    let con = Connection {
        name: name,
        email: Some(email),
        phone: Some(phone),
        links: links,
        position: Some(pos),
        company: Some(company),
        strength: stren,
        tags: vec![],
        met: met,
    };

    add_connection(con)
}

fn add_connection(con: Connection) {
    let connections_dir = getdir::get_networkly_connections_dir();
    let connections_file_path = connections_dir.join("connections.json");
    let mut connections_json: Connections = get_connections_json(&connections_file_path);
    let con_name = con.name.clone();

    connections_json.connections.push(con);

    let updated_connections_data =
        serde_json::to_string(&connections_json).expect("Could not write back new connections");

    let mut file = File::create(connections_file_path).expect("Failed to rewrite connections file");
    file.write_all(updated_connections_data.as_bytes())
        .expect("Failed to write new connections to new connections file");

    println!("Added {}", con_name);
}

pub fn delete_connection(name: &String) {
    let connections_dir = getdir::get_networkly_connections_dir();
    let connections_file_path = connections_dir.join("connections.json");
    let mut connections_json: Connections = get_connections_json(&connections_file_path);

    let matched_con: Vec<&Connection> = connections_json
        .connections
        .iter()
        .filter(|con| con.name.contains(name))
        .collect();

    let con_to_delete = list_by_name_and_get_connections(matched_con);
    if con_to_delete.is_some() {
        if let Some(index) = connections_json
            .connections
            .iter()
            .position(|c| c == con_to_delete.as_ref().unwrap())
        {
            connections_json.connections.remove(index);
        }

        let updated_connections = serde_json::to_string(&connections_json)
            .expect("Could not write connections following delete");

        let mut file = File::create(connections_file_path)
            .expect("Failed to create new connections after delete");

        file.write_all(updated_connections.as_bytes())
            .expect("Failed to write to updated connections file following delete");

        println!("Removed Connection");
    }
}

fn get_connections_json(path: &PathBuf) -> Connections {
    let connections_data = fs::read_to_string(path).expect("Could not read connections.json");
    let connections_json: Connections =
        serde_json::from_str(&connections_data).expect("Could not parse connections.json");
    connections_json
}

fn list_by_name_and_get_connections(matched_con: Vec<&Connection>) -> Option<Connection> {
    if matched_con.len() == 0 {
        println!("No matching connections");
        return None;
    }
    println!("Found the following connections:");
    for (index, con) in matched_con.iter().enumerate() {
        println!("{}: {}", index + 1, con.name);
    }
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    print!("Please enter the number of the one you are looking for: ");
    let _ = handle.flush();
    let mut con_id = String::new();
    io::stdin()
        .read_line(&mut con_id)
        .expect("Failed to read line");

    let numeric_con_id: usize = match con_id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("You entered an invalid number. Please try again.");
            return None;
        }
    };
    let con = matched_con.into_iter().nth(numeric_con_id - 1).cloned();
    con
}
