use clap::{Arg, Command};

fn main() {
    let rex_root = std::env::var("REX_ROOT").expect("Expected REX_ROOT to be set.");

    let matches = Command::new("rex")
        .subcommand(
            Command::new("new")
                .about("Creates a new latex document")
                .arg(
                    Arg::new("type")
                        .short('t')
                        .long("type")
                        .value_name("TYPE")
                        .help("Specifies the value of the document")
                        .required(true),
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .value_name("NAME")
                        .help("Specifies the name of the file. Creates <name>.tex"),
                )
                .arg(
                    Arg::new("directory")
                        .value_name("DIRECTORY")
                        .help("Specifies the relative directory")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let entity_type = matches.get_one::<String>("type").expect("Expected type");
        let entity_name = matches.get_one::<String>("name").expect("Expected name");
        let directory = matches
            .get_one::<String>("directory")
            .expect("Expected directory");

        println!(
            "Creating a new {} named '{}' in directory '{}'",
            entity_type, entity_name, directory
        );
    }

    println!("REX_ROOT: {}", rex_root);
}
