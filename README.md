# Crust8: A CHIP-8 Emulator in Rust

Crust8 is a fast, portable, and modular CHIP-8 emulator written in Rust. It aims to provide an accurate and maintainable implementation of the CHIP-8 virtual machine, with a focus on code clarity, performance, and extensibility. Whether you're a retro gaming enthusiast or a developer interested in emulation, Crust8 makes it easy to run CHIP-8 programs on modern hardware.

## Features

- **Accurate Emulation**: Faithful implementation of the CHIP-8 instruction set.
- **Modular Design**: Separates core emulation logic from the desktop frontend for flexibility.
- **Cross-Platform**: Built with Rust for portability across platforms.
- **Sample ROMs**: Includes a collection of public-domain CHIP-8 ROMs for testing.
- **Developer-Friendly**: Comprehensive test suite, linting, and formatting tools.

## Project Structure

The repository is organized as a Cargo workspace:

- **`crust8_core/`**: Core CHIP-8 emulator logic, independent of any frontend.
- **`desktop/`**: Desktop frontend for running CHIP-8 games with a graphical interface.
- **`roms/`**: Sample CHIP-8 ROMs for testing and development.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable, latest version recommended)
- [just](https://github.com/casey/just) (optional, for simplified build and development commands)

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/crust8.git
   cd crust8
   ```

2. Install Rust if not already installed:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. (Optional) Install `just` for streamlined commands:
   ```sh
   cargo install just
   ```

## Building

Crust8 uses [just](https://github.com/casey/just) for common development tasks. To list available commands, run:

```sh
just
```

### Build in Release Mode

```sh
just build-release
```

### Build in Debug Mode

```sh
just build-debug
```

## Running

To run a CHIP-8 ROM using the desktop frontend:

```sh
just run-desktop ./roms/pong.ch8
```

Replace `./roms/pong.ch8` with the path to your desired ROM file.

## Testing

Run the full test suite to verify emulator correctness:

```sh
just test
```

## Code Quality

Ensure consistent code style and catch potential issues with the following commands:

### Check Formatting

```sh
just fmt-check
```

### Format Code

```sh
just fmt
```

### Run Clippy Linter

```sh
just lint
```

## License

Crust8 is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the CHIP-8 community and its rich history of emulators.
- Built with Rust and the amazing [Cargo](https://doc.rust-lang.org/cargo/) ecosystem.