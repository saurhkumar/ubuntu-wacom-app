# Hello World Ubuntu App

A simple Rust GTK application that shows "Hello World" when a button is clicked.

## Prerequisites

You need to have Rust and GTK3 development libraries installed:

```bash
sudo apt update
sudo apt install -y build-essential libgtk-3-dev
```

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Building the Application

```bash
cd /path/to/hello_world_app
cargo build --release
```

## Running the Application

```bash
./target/release/hello_world_app
```

## Features

- Simple GTK-based UI
- Button that toggles visibility of a "Hello, World!" message when clicked
- Includes an application icon
