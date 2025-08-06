# Ubuntu Wacom Tablet App

A simple Rust GTK application that allows switching the mapping of Wacom tablets between monitors.

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
cd /path/to/ubuntu-wacom-app
cargo build --release
```

## Running the Application

```bash
./target/release/wacom_tablet_app
```

## Features

- Simple GTK-based UI
- Button that switches Wacom tablet mapping between monitors
- Shows status updates for successful or failed operations
- Includes an application icon
