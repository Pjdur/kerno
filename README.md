# Kerno Shell

Kerno is a minimalist command-line shell written in Rust, focused on speed, simplicity, and portability.

[![Crates.io](https://img.shields.io/crates/v/kerno.svg)](https://crates.io/crates/kerno)
[![Build Status](https://img.shields.io/github/actions/workflow/status/pjdur/kerno/main.yml)](https://github.com/pjdur/kerno/actions)

---

## Features

- Built-in commands: `echo`, `scanpath`, `set`, `get`, `unset`, `env`, `cd`, `pwd`, `ls`, `cat`, `touch`, `rm`, `mkdir`, `rmdir`, `date`, `clear`, `write`, `read`, `exit`, `help`, `history`
- Automatically recognizes executables from `$PATH`
- Persistent environment variables stored in `$HOME/kerno.toml`
- Session history tracking
- Fully cross-platform
- Written in pure Rust — no bash dependency

---

## Quick Start

Install via Cargo:

```bash
cargo install kerno
```

Run the shell:

```bash
kerno
```

---

## Configuration

Kerno automatically loads environment variables from:

```bash
~/.kerno.toml
```

To add persistent variables:
```bash
set FOO bar
```

Or edit the TOML file directly:
```toml
FOO = "bar"
PATH = "/usr/local/bin"
```

---

## Examples

```bash
echo Hello world
set NAME Kerno
get NAME
scanpath              # List all system executables
cd src
ls
```

---

## Why Kerno?

Because sometimes you want a shell that is:
- Lightweight
- Understandable
- Hackable
- Rusty

---

## Author

Created by [Pjdur](https://github.com/Pjdur)  
Licensed under MIT

---

## Contribute

Pull requests welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.