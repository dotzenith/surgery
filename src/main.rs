mod rdclient;

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use rdclient::{RDClient, UnrestrictedLink};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use skimple::SkimpleMatcher;
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

    let matcher = SkimpleMatcher::default();
    let client: RDClient = RDClient::new()?;
    let mut rl = DefaultEditor::new()?;

    println!("Fetching torrents........");
    let torrents = client.get_torrents()?;
    let names: Vec<&str> = torrents.iter().map(|item| item.filename.as_str()).collect();
    let cleaned_name: String = cli.name.chars().filter(|c| c.is_alphanumeric()).collect();

    let matches = match cleaned_name.is_empty() {
        true => names,
        false => matcher.fuzzy_all(&names, &cleaned_name)?,
    };

    let torrent = if cli.best || matches.len() == 1 {
        torrents
            .iter()
            .find(|elem| elem.filename == *matches[0])
            .expect("Torrent somehow disappeared")
    } else {
        for (idx, item) in matches.iter().enumerate() {
            println!("{}: {}", idx, item);
        }

        let input = match rl.readline("\nPlease enter the torrent number to download: ") {
            Ok(number) => number,
            Err(ReadlineError::Interrupted) => return Ok(()),
            Err(ReadlineError::Eof) => return Ok(()),
            Err(err) => return Err(err.into()),
        };

        let number: usize = input.trim().parse().context("Invalid number")?;
        let name = matches.get(number).context("Invalid number selected")?;
        torrents
            .iter()
            .find(|elem| elem.filename == *name)
            .context("Torrent somehow disappeared")?
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

        let input = match rl.readline("\nPlease enter a range to download (eg. 0-4): ") {
            Ok(number) => number,
            Err(ReadlineError::Interrupted) => return Ok(()),
            Err(ReadlineError::Eof) => return Ok(()),
            Err(err) => return Err(err.into()),
        };

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
