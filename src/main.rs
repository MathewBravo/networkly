mod connections;
mod getdir;
mod tags;

use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes your network
    Init(InitDir),
    /// Adds a new connection to your network
    Connect(ConnectTo),
    /// Deletes an existing connection
    DelCon(DeleteCon),
    /// Adds a tag to your tag list
    AddTag(AddTag),
    /// Deletes tag from your network
    DeleteTag(DeleteTag),
    /// Lists all tags in your network
    ListTags(ListTags),
}

#[derive(Args)]
struct DeleteCon {
    name: String,
}

#[derive(Args)]
struct ConnectTo {}

#[derive(Args)]
struct AddTag {
    tag: String,
}

#[derive(Args)]
struct ListTags {}

#[derive(Args)]
struct DeleteTag {
    tag: String,
}

#[derive(Args)]
struct InitDir {}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(_) => {
            init_file_structure();
        }
        Commands::Connect(_) => connections::build_connection(),
        Commands::DelCon(del) => connections::delete_connection(&del.name),
        Commands::AddTag(tag) => tags::add_tag_to_config(&tag.tag),
        Commands::ListTags(_) => tags::list_tags(),
        Commands::DeleteTag(tag) => tags::delete_tag_from_config(&tag.tag),
    }
}

fn init_file_structure() {
    let home_dir = getdir::get_home_dir();
    let dir_path = getdir::get_file_path(&home_dir, ".networkly");

    if !dir_path.exists() {
        fs::create_dir(&dir_path).expect("Failed to create networkly home directory");
        println!("Created .networkly directory at: {:?}", dir_path);

        let connections = r#"
{
    "connections":[

    ]
}      
"#;
        let connections_dir = dir_path.join("connections");
        fs::create_dir(&connections_dir).expect("Failed to initialize the connections directory");

        let connections_file_path = connections_dir.join("connections.json");
        let connections_file =
            File::create(&connections_file_path).expect("Could not create connections.json");
        let mut writer = BufWriter::new(connections_file);
        writer
            .write_all(connections.as_bytes())
            .expect("Could not write connections structure to connections.json");
        let _ = writer.flush();

        let tags = r#"
{
    "tags":[]
}"#;
        let file = File::create(home_dir + "/.networkly/tags.json")
            .expect("Failed whilst creating tags config");
        let mut writer = BufWriter::new(file);
        writer
            .write_all(tags.as_bytes())
            .expect("Could not write tags to tags.json config");
        let _ = writer.flush();
    } else {
        println!(".networkly directory has already been initialized");
    }
}
