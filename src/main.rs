use clap::{command, Arg, Command};

fn main() {
    let match_result = command!()
        .about("This application creates new files and folders in current directory")
        .subcommand(
            Command::new("create-file")
                        .about("Creates a new file")
                        .arg(
                            Arg::new("file-name")
                                .short('n')
                                .long("new-file")
                                .required(true)
                                .help("The name of the new file")
                        )   
        )
        .subcommand(
            Command::new("create-folder")
                        .about("Creates a new folder")
                        .arg(
                            Arg::new("folder-name")
                                .long("new-folder")
                                .required(true)
                                .help("The name of the new file")
                        )
        )
    .get_matches();

    match match_result.subcommand_matches("create-file") {
        Some(file_args) => {
            let file_path = file_args.get_one::<String>("file-name").unwrap();
            new_commands::create::create_file(file_path);
        },
        None => (),
    };
    match match_result.subcommand_matches("create-folder") {
        Some(folder_args) => {
            let folder_path = folder_args.get_one::<String>("folder-name").unwrap();
            new_commands::create::create_folder(folder_path);
        },
        None => (),
    };
}