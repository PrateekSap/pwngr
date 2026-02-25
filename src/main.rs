// init, add, get, list, delete

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("pwngr")
        .about("A simple password manager")
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

    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Initializing application ");
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
