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

Surgery is a simple CLI to interact with Real Debrid. The aim is to allow user to fuzzy match and download files
from their Real Debrid account.

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

Usage: sg [DOWNLOAD]

Arguments:
  [DOWNLOAD]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
---

## ❖ What's New?

0.1.0 - Initial Incomplete release

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
