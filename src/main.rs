mod rdclient;
mod data;

use clap::Parser;
use rdclient::RDClient;

#[derive(Parser)]
#[command(name = "sg")]
#[command(about, version, author)]
struct Cli {
    download: Option<String>,
}

fn main() {

    let _cli = Cli::parse();

    let client: RDClient = match RDClient::new() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Unable to build RDClient: {}", err);
            std::process::exit(1)
        }
    };

    match client.get_username() {
        Ok(user) => println!("Hi! {}", user),
        Err(err) => {
            eprintln!("Unable to get user info: {}", err);
            std::process::exit(1)
        }
    }
}
