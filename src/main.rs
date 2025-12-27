mod rdclient;

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use nucleo_matcher::{Config, Matcher, pattern};
use rdclient::{RDClient, UnrestrictedLink};
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser)]
#[command(name = "srg")]
#[command(about, version, author)]

struct Cli {
    /// torrent name for fuzzy matching (required)
    name: String,

    /// Download the best matching torrent for the provided name
    #[arg(short, long, default_value_t = false)]
    best: bool,

    /// Download all files for the selected torrent
    #[arg(short, long, default_value_t = false)]
    all: bool,
}

fn main() -> Result<()> {
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

    let mut matcher = Matcher::new(Config::DEFAULT);
    let client: RDClient = RDClient::new()?;

    println!("Fetching torrents........");
    let torrents = client.get_torrents()?;
    let names: Vec<&str> = torrents.iter().map(|item| item.filename.as_str()).collect();
    let cleaned_name: String = cli.name.chars().filter(|c| c.is_alphanumeric()).collect();

    let matches = pattern::Pattern::new(
        &cleaned_name,
        pattern::CaseMatching::Ignore,
        pattern::Normalization::Smart,
        pattern::AtomKind::Fuzzy,
    )
    .match_list(&names, &mut matcher);

    if matches.len() == 0 {
        eprintln!("No matches found");
        std::process::exit(1)
    }

    let torrent = if cli.best || matches.len() == 1 {
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

        let number: usize = input.trim().parse().context("Invalid number")?;
        &torrents.get(number).context("Invalid number selected")?
    };

    println!("Unrestricting links......");
    let links = torrent
        .links
        .iter()
        .map(|link| client.unrestrict_link(link))
        .collect::<Result<Vec<UnrestrictedLink>>>()
        .map_err(|e| anyhow!("Unable to unrestrict link: {}", e))?;

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

        let left: usize = split[0].trim().parse().context("Invalid range")?;
        let right: usize = split[1].trim().parse().context("Invalid range")?;

        &links.get(left..=right).context("Invalid range")?
    };

    println!("\nDownloading..............\n");
    for link in download.into_iter() {
        println!("{}", &link.filename);

        let status = Command::new("curl")
            .arg("--progress-bar")
            .arg(&link.download)
            .arg("--output")
            .arg(&link.filename)
            .status()
            .expect("Failed to execute curl");

        if !status.success() {
            eprintln!("Unable to download: {}", &link.filename);
        }
    }

    Ok(())
}
