<h1 align="center"> ━━━━  ❖  ━━━━ </h1>

<!-- BADGES -->
<div align="center">
   <p></p>

   <img src="https://img.shields.io/github/stars/dotzenith/surgery?color=F8BD96&labelColor=302D41&style=for-the-badge">
   <img src="https://img.shields.io/github/commit-activity/y/dotzenith/surgery?color=96CDFB&labelColor=302D41&style=for-the-badge&label=COMMITS"/>
   <br>
</div>

<p/>

---

## ❖ Surgery

Surgery is a simple CLI to interact with Real Debrid. The aim is to allow users to fuzzy match and download torrents
from their Real Debrid account.

---

## ❖ Requirements

1. Ensure [curl](https://curl.se/) is installed
2. Set the following environment variables:
```sh
# You likely do not need to change this
# also note the lack of trailing slash
export RD_BASE_URL="https://api.real-debrid.com/rest/1.0"

# Available at: https://real-debrid.com/apitoken
export RD_API_KEY=""
```
---

## ❖ Installation

#### Shell
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dotzenith/surgery/releases/latest/download/surgery-installer.sh | sh
```

#### Brew
```sh
brew install dotzenith/tap/surgery
```

#### Powershell
```sh
powershell -ExecutionPolicy ByPass -c "irm https://github.com/dotzenith/surgery/releases/latest/download/surgery-installer.ps1 | iex"
```

#### Cargo
```sh
cargo install surgery
```

#### Binaries
Pre-Compiled binaries for linux, mac, and windows are available in [Releases](https://github.com/dotzenith/surgery/releases)

#### Source
- First, install [rust](https://rustup.rs/)
```sh
git clone https://github.com/dotzenith/surgery.git
cd surgery
cargo build --release
./target/release/sg
```

---

## ❖ Usage

```
A simple CLI for Real Debrid

Usage: sg [OPTIONS] <NAME>

Arguments:
  <NAME>  torrent name for fuzzy matching (required)

Options:
  -b, --best     Download the best matching torrent for the provided name
  -a, --all      Download all files for the selected torrent
  -h, --help     Print help
  -V, --version  Print version
```

### Normal Usage

```sh
sg "debian" # torrent names are fuzzy matched
```
- This will prompt the user to select a specific torrent if there is more than one match
- This will also prompt the user to select a range if the torrent has multiple files

### Pick the best matched torrent

```sh
sg "ubuntu" --best # torrent names are fuzzy matched
```
- This will pick the torrent that best matches the search string 
- This will still prompt the user to select a range if the torrent has multiple files

### Download all files for a given torrent

```sh
sg "ubuntu" --all # torrent names are fuzzy matched
```
- This will prompt the user to select a specific torrent if there is more than one match
- This will download all files for the selected torrent

### 

```sh
sg "ubuntu" --best --all # torrent names are fuzzy matched
```
- This will pick the best matched torrent, and download all files

---

## ❖ What's New?

1.0.0 - Initial release

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
