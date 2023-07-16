use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use serde_derive::{Deserialize, Serialize};

use crate::getdir;

#[derive(Serialize, Deserialize)]
struct TagsData {
    tags: Vec<String>,
}

fn get_tags_json(path: &Path) -> TagsData {
    let tag_data = fs::read_to_string(path).expect("Could not read tags.json");
    let tag_json: TagsData = serde_json::from_str(&tag_data).expect("Could not parse tags.json");
    tag_json
}

pub fn add_tag_to_config(tag: &String) {
    let home_dir = getdir::get_home_dir();
    let file_path = format!("{}/.networkly/tags.json", home_dir);
    let path = Path::new(&file_path);
    let mut tag_json: TagsData = get_tags_json(path);

    if !tag_json.tags.contains(tag) {
        tag_json.tags.push(tag.clone());

        let updated_tag_data =
            serde_json::to_string(&tag_json).expect("Could not write tags to string");

        let mut file = File::create(path).expect("Failed to rewrite tags");
        file.write_all(updated_tag_data.as_bytes())
            .expect("Failed to write updated tag data to tags.json");

        println!("Added tag {} to tags", tag)
    } else {
        println!("{} already exists as a tag", tag)
    }
}

pub fn delete_tag_from_config(tag: &String) {
    let home_dir = getdir::get_home_dir();
    let file_path = format!("{}/.networkly/tags.json", home_dir);
    let path = Path::new(&file_path);
    let mut tag_json: TagsData = get_tags_json(path);

    if tag_json.tags.contains(tag) {
        if let Some(index) = tag_json.tags.iter().position(|t| t == tag) {
            tag_json.tags.remove(index);
        }

        let updated_tag_data =
            serde_json::to_string(&tag_json).expect("Could not write tags to string");

        let mut file = File::create(path).expect("Failed to rewrite tags");
        file.write_all(updated_tag_data.as_bytes())
            .expect("Failed to write updated tag data to tags.json");

        println!("Removed tag {} from tags", tag)
    } else {
        println!("{} could not be found in your tags", tag)
    }
}

pub fn list_tags() {
    let home_dir = getdir::get_home_dir();
    let file_path = format!("{}/.networkly/tags.json", home_dir);
    let path = Path::new(&file_path);
    let tag_json: TagsData = get_tags_json(path);

    println!("Your tags are: ");
    for ( index, tag ) in tag_json.tags.iter().enumerate() {
        println!("{}: {}", index+1, tag);
    }
}


