# Crust8

Crust8 is a CHIP-8 emulator written in Rust.  
It is designed to be portable, modular, and easy to maintain, with a focus on code clarity and correctness.

## Project Structure

The repository is organized as a Cargo workspace:

- **desktop/** – Desktop frontend for running CHIP-8 games.
- **crust8_core/** – Core CHIP-8 emulator implementation.
- **roms/** – Collection of sample CHIP-8 ROMs for testing.

## Building

This project uses [just](https://github.com/casey/just) for common development commands.  
To list all available commands:

```sh
just
```

### Build in release mode

```sh
just build-release
```

### Build in debug mode

```sh
just build-debug
```

## Running

To run the desktop frontend with a ROM:

```sh
just run-desktop ./roms/pong.ch8
```

Replace `./roms/pong.ch8` with the path to your ROM.

## Testing

Run the full test suite:

```sh
just test
```

## Linting and Formatting

Check formatting:

```sh
just fmt-check
```

Automatically format code:

```sh
just fmt
```

Run Clippy linter:

```sh
just lint
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
