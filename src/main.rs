mod rdclient;

use anyhow::Result;
use clap::Parser;
use nucleo_matcher::{Config, Matcher, pattern};
use rdclient::{RDClient, UnrestrictedLink};
use std::io::{self, Write};
use std::process::Command;

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
    match Command::new("curl")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
    {
        true => (),
        false => {
            eprintln!("curl not installed or not in path. Please install curl and try again");
            std::process::exit(1)
        }
    }
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
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: usize = input.trim().parse().expect("Invalid number");
        &torrents.get(number).expect("Invalid number selected")
    };

    let links = match torrent
        .links
        .iter()
        .map(|link| client.unrestrict_link(link))
        .collect::<Result<Vec<UnrestrictedLink>>>()
    {
        Ok(links) => links,
        Err(err) => {
            eprintln!("Unable to unrestrict link: {}", err);
            std::process::exit(1)
        }
    };

    let download = if cli.all || links.len() == 1 {
        &links[..]
    } else {
        for (idx, link) in links.iter().enumerate() {
            println!("{}: {}", idx, link.filename)
        }

        print!("\nPlease enter a range to download (eg. 0-4): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let split: Vec<&str> = input.split("-").collect();
        if split.len() != 2 {
            eprintln!("Invalid syntax for range");
            std::process::exit(1)
        }

        let left: usize = split[0].trim().parse().expect("Invalid number");
        let right: usize = split[1].trim().parse().expect("Invalid number");

        &links.get(left..=right).expect("Invalid Range")
    };

    for link in download.into_iter() {
        Command::new("curl")
            .arg(&link.download)
            .arg("--output")
            .arg(&link.filename)
            .status()
            .expect("Unable to call curl");
    }
}
