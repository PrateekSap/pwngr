// init, add, list, delete
mod store;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("pwngr")
        .about("A simple password manager")
        .subcommand_required(true) // a command should always be provided
        .subcommand(Command::new("init").about("Initialize appilcation"))
        .subcommand(
            Command::new("add")
                .about("Add a password")
                .arg(Arg::new("Service").required(true))
                .arg(Arg::new("Password").required(true)),
        )
        .subcommand(Command::new("list").about("List services saved"))
        .subcommand(
            Command::new("remove")
                .about("Delete a service info")
                .arg(Arg::new("Service").required(true)),
        )
        .get_matches();

    let subcommand = matches.subcommand();

    if !matches!(subcommand, Some(("init", _))) && !store::is_initialized() {
        eprintln!("Vault not initialized. Run `init` first.");
        std::process::exit(1);
    }

    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Initializing application ");
            if let Err(e) = store::create_file() {
                eprintln!("Intitalizing failed. Something went wrong. {}", e)
            }
        }

        Some(("add", sub_matches)) => {
            let service = sub_matches.get_one::<String>("Service").unwrap();
            let password = sub_matches.get_one::<String>("Password").unwrap();

            println!("Adding {} -> {}", service, password);
        }

        Some(("list", _)) => {
            println!("The services listed are... ");
        }

        Some(("remove", sub_matches)) => {
            let service = sub_matches.get_one::<String>("Service").unwrap();

            println!("Removing {} ", service);
        }

        _ => {}
    }
}
