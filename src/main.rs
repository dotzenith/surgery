mod data;
mod rdclient;

use clap::Parser;
use nucleo_matcher::{Config, Matcher, pattern};
use rdclient::RDClient;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "sg")]
#[command(about, version, author)]

struct Cli {
    /// torrent name for fuzzy matching (required)
    name: String,

    /// Download first/best matching torrent for the provided name
    #[arg(short, long, default_value_t = false)]
    first: bool,

    /// Download all files for the selected torrent
    #[arg(short, long, default_value_t = false)]
    all: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());

    let client: RDClient = match RDClient::new() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Unable to build RDClient: {}", err);
            std::process::exit(1)
        }
    };

    let torrents = match client.get_torrents() {
        Ok(torrents) => torrents,
        Err(err) => {
            eprintln!("Unable to get torrents: {}", err);
            std::process::exit(1)
        }
    };

    let names: Vec<&str> = torrents.iter().map(|item| item.filename.as_str()).collect();

    let matches = pattern::Pattern::new(
        &cli.name,
        pattern::CaseMatching::Ignore,
        pattern::Normalization::Smart,
        pattern::AtomKind::Fuzzy,
    )
    .match_list(&names, &mut matcher);

    if matches.len() == 0 {
        eprintln!("No matches found");
        std::process::exit(1)
    }

    let torrent = if cli.first || matches.len() == 1 {
        torrents
            .iter()
            .find(|elem| elem.filename == *matches[0].0)
            .expect("Torrent somehow disappeared")
    } else {
        for (idx, item) in matches.iter().enumerate() {
            println!("{}: {}", idx, item.0);
        }
        print!("\nPlease enter the torrent number to download: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: u32 = input.trim().parse().expect("Invalid number");

        if number >= matches.len() as u32 {
            eprintln!("Invalid number selected");
            std::process::exit(1)
        }
        &torrents[number as usize]
    };

    println!("{:?}", torrent);
}
