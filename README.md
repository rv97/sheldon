# Sheldon

![Rust](https://img.shields.io/badge/Rust-1.79-orange?logo=rust) ![macOS](https://img.shields.io/badge/macOS-Apple%20Silicon-blue?logo=apple) ![License](https://img.shields.io/badge/license-MIT-green)

**Sheldon** is a sleek, terminal-based performance monitoring tool designed specifically for Apple Silicon Macs. Built with Rust and powered by `ratatui`, it provides real-time insights into your system’s CPU usage with a dynamic, scrolling graph—think Activity Monitor, but cooler and in your terminal.

## Features

- **Live CPU Graph**: Watch your CPU cores’ usage scroll from right to left in real time, with colorful lines for each core.
- **Apple Silicon Optimized**: Tailored for M1, M2, and beyond, leveraging Rust’s performance on macOS.
- **Terminal UI**: Built with `ratatui` for a smooth, interactive experience without leaving the command line.
- **Extensible**: Foundation for adding memory, swap, GPU, and power usage (coming soon!).

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.79 or later recommended)
- macOS running on Apple Silicon (M1, M2, etc.)

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/rv97/sheldon.git
   cd sheldon
   ```
2. Build and run:
    ```
    cargo run
    ```

### Usage
Run the tool using:
    ```
    cargo run
    ```
- A scrolling CPU usage graph will appear in your terminal.
- Press `q` to quit

## Roadmap
- [x] Scrolling CPU usage graph
- [ ] Memory usage display
- [ ] Swap usage display
- [ ] GPU usage graph (macOS-specific)
- [ ] Power (watts) monitoring
- [ ] Configurable graph styles and intervals

## Contributing
Love Rust or terminal UIs? Contributions are welcome!

1. Fork the repo.
2. Create a feature branch (`git checkout -b feature/cool-thing`).
3. Commit your changes (`git commit -m "Add cool thing"`).
4. Push to the branch (`git push origin feature/cool-thing`).
5. Open a pull request.

## License
Distributed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments
- Built with [Rust](https://www.rust-lang.org/), [sysinfo](https://crates.io/crates/sysinfo), [ratatui](https://crates.io/crates/ratatui), and [crossterm](https://crates.io/crates/crossterm).
- Inspired by macOS Activity Monitor and a love for terminal tools.