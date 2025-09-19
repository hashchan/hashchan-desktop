# HashChan Desktop (Alpha, Prototyping)

A cross-platform desktop application that combines a lightweight Ethereum node with a GUI interface for HashChan3.

## Overview

HashChan Desktop is an all-in-one application that:

1. Runs a specialized Reth node focused on the HashChan3 contract
2. Provides a native GUI interface built with egui
3. Stores data in an embedded SQLite database
4. Works on Windows, macOS, and Linux

## Architecture

The application is built with a modular architecture:

```
src/
├── main.rs       # Entry point and application coordination
├── gui/          # GUI components using egui/eframe
├── db/           # SQLite database implementation
└── reth/         # Ethereum node and contract event indexing
```

### Key Components

1. **GUI Layer (egui/eframe)**
   - Pure Rust GUI framework with no JavaScript
   - Displays contract data and node status
   - Updates in real-time as new data is indexed

2. **Database Layer (SQLite)**
   - Embedded database with no external dependencies
   - Stores boards, threads, posts, and relationships
   - Automatically creates schema on first run

3. **Blockchain Layer (Reth)**
   - Lightweight Ethereum node focused on HashChan3 contract
   - Uses Reth's Execution Extensions (ExEx) for efficient indexing
   - Runs in a separate thread with graceful shutdown

## Setup

### Prerequisites

- Rust and Cargo
- For building Reth:
  - Ubuntu/Debian: `sudo apt-get install libclang-dev`
  - Fedora/RHEL: `sudo dnf install clang-devel`
  - Arch Linux: `sudo pacman -S clang`
  - Windows: Install LLVM from the [LLVM download page](https://releases.llvm.org/download.html)

No database setup is required as SQLite is embedded and the schema is created automatically on first run.

### Building

```bash
# Clone the repository
git clone https://github.com/yourusername/hashchan.git
cd hashchan/desktop

# Build the application
cargo build --release
```

### Running

```bash
# Run the application
cargo run --release --bin hashchan-node
```

## Features

- **Integrated Node**: Runs a specialized Ethereum node focused on the HashChan contract
- **Embedded Database**: Uses SQLite for data storage with no external dependencies
- **Native GUI**: Built with egui for a fast, native experience on all platforms
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Resource Efficient**: Optimized for lower resource usage compared to web-based alternatives

## Contract Details

- **Address**: 0x458c27D5a6421AfAFF435e27E870584Fe03a938F
- **Events**:
  - `NewBoard`: Emitted when a new board is created
  - `NewThread`: Emitted when a new thread is created
  - `NewPost`: Emitted when a new post is created

## Current Status

This project is in alpha stage with the following components working:

- ✅ Basic GUI with egui
- ✅ SQLite database integration
- ✅ Reth node integration
- ✅ Multi-threaded architecture
- ✅ Graceful shutdown mechanism
- ⏳ Board/thread/post views (in progress)
- ⏳ Event indexing from contract (in progress)
- ⏳ Cross-platform packaging (planned)

## Future Enhancements

1. Complete board/thread/post views in the GUI
2. Implement full event data parsing from the contract
3. Add user settings and preferences
4. Create installers for Windows, macOS, and Linux
5. Add support for multiple networks (mainnet, testnets)
