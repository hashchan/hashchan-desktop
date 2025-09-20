<<<<<<< HEAD
# HashChan Desktop: Research Project
=======
# HashChan Desktop (Alpha, Prototyping)
>>>>>>> 4acf642d10868f4cd6673d2068df4cea3db5ef2e

## The Problem We're Solving

Current decentralized applications face significant usability challenges:

1. **Heavy JavaScript Dependencies**: Most dApps require numerous JS libraries and frameworks, creating a bloated user experience
2. **RPC Provider Dependency**: Users must find and trust third-party RPC providers just to browse content
3. **Centralization Risks**: Relying on external RPC services introduces centralization and privacy concerns
4. **Resource Intensity**: Web-based interfaces are often resource-heavy and inefficient

## Our Research Approach

We're exploring a radically different architecture for dApp interfaces using:

1. Reth's Execution Extensions (ExEx) to create an **ultralight Ethereum client** focused only on the HashChan3 contract
2. A native GUI built with egui (pure Rust, no JavaScript)
3. Local data storage with an embedded SQLite database
4. True peer-to-peer networking without centralized RPC providers

## Experimental Architecture

Our research prototype uses a modular architecture to test our hypotheses:

```
src/
├── main.rs       # Entry point and application coordination
├── gui/          # GUI components using egui/eframe
├── db/           # SQLite database implementation
└── reth/         # Ethereum node and contract event indexing
```

### Key Experimental Components

1. **GUI Layer (egui/eframe)**
   - **Zero JavaScript**: Pure Rust GUI framework eliminates all JS dependencies
   - **Native Performance**: Direct hardware access without browser overhead
   - **Minimal Resource Usage**: Significantly lower memory and CPU requirements

2. **Database Layer (SQLite)**
   - **Offline-First**: Local data storage enables browsing without constant network access
   - **Zero External Dependencies**: Fully embedded database requires no setup
   - **Efficient Indexing**: Optimized for the specific data structures of HashChan

3. **Blockchain Layer (Reth ExEx)**
   - **Ultralight Client**: Focused solely on HashChan3 contract events
   - **No RPC Provider Needed**: Direct P2P connection to Ethereum network
   - **Privacy Preserving**: No third-party can track your browsing habits

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

## Research Goals & Hypotheses

We're testing several key hypotheses with this experimental approach:

1. **User Experience Improvement**: Can we create a more responsive, reliable experience by eliminating JavaScript and web dependencies?

2. **True Decentralization**: Can users browse HashChan content without relying on centralized RPC providers?

3. **Resource Efficiency**: Can we significantly reduce CPU, memory, and network usage compared to web-based alternatives?

4. **Accessibility**: Can we lower the technical barriers to using truly decentralized applications?

5. **Privacy Enhancement**: Can we enable users to browse content without exposing their IP or browsing habits to third parties?

## Contract Details

- **Address**: 0x458c27D5a6421AfAFF435e27E870584Fe03a938F
- **Events**:
  - `NewBoard`: Emitted when a new board is created
  - `NewThread`: Emitted when a new thread is created
  - `NewPost`: Emitted when a new post is created

## Current Research Status

This experimental prototype is in early research phase with the following progress:

- ✅ Proof of concept: Pure Rust GUI with egui (zero JavaScript)
- ✅ Local data persistence with embedded SQLite 
- ✅ Integration with Reth node using Execution Extensions
- ✅ Multi-threaded architecture with resource isolation
- ✅ Graceful shutdown mechanism for blockchain components
- ⏳ Content viewing interfaces (in research)
- ⏳ Contract event indexing optimization (in research)
- ⏳ Distribution methods evaluation (planned)

### Platform Testing Notes

<<<<<<< HEAD
Our research currently focuses on specific platforms due to technical constraints:

- **Linux**: Primary research platform with full functionality
- **macOS**: Secondary research platform with basic functionality
- **Windows**: Limited testing due to Reth compatibility constraints

According to [Reth's documentation](https://reth.rs/installation/overview/), Reth currently "runs on Linux and macOS (Windows tracked)", meaning Windows support is being researched but isn't fully implemented. Our cross-compilation experiments for Windows have encountered dependency challenges with platform-specific code.

## Research Roadmap & User Feedback Goals

We're seeking user feedback on several key aspects of this experimental approach:

1. **User Experience**: Does eliminating JavaScript dependencies create a noticeably better experience?
2. **Setup Process**: Is the local node approach easier than finding and configuring RPC providers?
3. **Performance**: How does the resource usage compare to web-based alternatives on your system?
4. **Feature Parity**: What web features are most critical to maintain in a native application?
5. **Network Effects**: Does the P2P approach provide adequate content synchronization speed?

## Alternative Research Path: Containerization

We're also exploring containerization as an alternative research direction, especially for Windows users where Reth support is limited:

```yaml
# docker-compose.yml (experimental)
version: '3'
services:
  hashchan-node:
    image: hashchan/desktop-node:latest
    ports:
      - "8545:8545"  # JSON-RPC API
      - "8000:8000"  # GUI interface
    volumes:
      - hashchan-data:/app/data

volumes:
  hashchan-data:
```

This experimental approach would:
- Enable cross-platform testing with minimal configuration
- Isolate complex dependencies within the container
- Provide consistent behavior across different environments
- Facilitate easier deployment of updates during research

We welcome feedback on whether this containerized approach would be valuable for your testing and usage scenarios.
=======
1. Complete board/thread/post views in the GUI
2. Implement full event data parsing from the contract
3. Add user settings and preferences
4. Create installers for Windows, macOS, and Linux
5. Add support for multiple networks (mainnet, testnets)
>>>>>>> 4acf642d10868f4cd6673d2068df4cea3db5ef2e
