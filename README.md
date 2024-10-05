# pono - `p`ack `on`ly `o`nce

[![Crate version](https://img.shields.io/crates/v/pono.svg?)](https://crates.io/crates/pono)
[![CI checks](https://github.com/cristianoliveira/pono/actions/workflows/on-push.yml/badge.svg)](https://github.com/cristianoliveira/pono/actions/workflows/on-push.yml)
[![Periodic checks](https://github.com/cristianoliveira/pono/actions/workflows/on-schedule-checks.yml/badge.svg)](https://github.com/cristianoliveira/pono/actions/workflows/on-schedule-checks.yml)

**pono** (__poh-no__ to place/store in Latin) is a CLI for managing symbolic links in one place inspired by `stow`. Because symbolic links management in bash script sucks.

## Why?

Let's face it, managing symbolic links with bash scripts sucks because the source may be missing the target may already exist, etc, etc. The alternative [GNU stow](https://www.gnu.org/software/stow/) does almost what I wanted, but not quite. I wanted a tool that could manage symlinks for multiple packages independent of the source file structure, using a flat configuration. I also need to be able to toggle links on demand to apply different configs.

Use cases: 

  - Managing enabling/disabling git hooks see pono's pono.toml :)
    - `pono enable git:pre-commit` or `pono enable git:pre-push`

  - Toggling between different `.env` when developing locally
    - `pono toggle develop | stage | live` + with autocompletion!

  - Dotfiles linking management and checks
    - `pono enable && pono status` or single "pono" `pono disable nvim`

## Demo

Create the `pono.toml` in the current directory
```toml
[ponos]
nvim = { source = "./examples/from/nvim", target = "./examples/to/nvim" }
zsh = { source = "./examples/from/zshrc", target = "./examples/to/.zshrc" }
```
And run
```bash
pono enable

Linking packages
  nvim: ./examples/to/nvim (new link)
  zsh: ./examples/to/.zshrc (new link)

ls -la examples/to                                                                                                                                                     [1:00:35]
total 0
..
lrwxr-xr-x 1 cris  58 Sep 14 01:00 .zshrc -> /home/cris/pono/./examples/src/zshrc
lrwxr-xr-x 1 cris  57 Sep 14 01:00 nvim -> /home/cris/pono/./examples/src/nvim
```

## Features

- Declarative symlink management with a simple TOML configuration file.
- Create, remove, and check symlinks for multiple packages at once.
- Customizable target directories for each package.
- Check the status of symlinks to detect broken links.

### Future features
- Supports pattern matching for including or excluding files.
- Built-in dry-run mode to preview changes before applying them.
- Verbose mode for detailed output.
- Manage the same link with different sources and toggle between them.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Examples](#examples)
- [License](#license)

## Installation

### With Cargo

You can install **pono** using [Cargo](https://doc.rust-lang.org/cargo/):

```bash
cargo install pono
```

### With Nix

You can install **pono** using [Nix](https://nixos.org/):

```bash
nix profile install github:cristianoliveira/pono#pono
```

# After Installation

Enable pono completions for your shell by adding the following to your shell configuration file:

```bash
if command -v pono &> /dev/null; then
  eval "$(pono completions)" ## or $(pono completions <shell>)
fi
```

Check `pono --help` for more information.

## Usage

The **pono** CLI allows you to install, remove, and check symlinks based on a TOML configuration file.

### Commands

- `enable`: Create symbolic links for the defined ponos.
- `disable`: Remove symbolic links for the defined ponos.
- `toggle`: Toggle a given pono and verify.
- `status`: Check the status the define ponos.
- `list`: Display all available ponos from the TOML configuration.

### Options

- `-c --config <file>`: Specify a custom TOML configuration file (default: `./pono.toml`).
- `--help`: Display help information.

### Basic Usage

#### Enabling symlinks (ponos)

To create symlinks for all packages defined in `pono.toml`:

```bash
pono enable
```

To enable symlinks for specific packages:

```bash
pono enable package1 package2
```

#### Disabling symlinks (ponos)

To remove symlinks for all packages:

```bash
pono disable
```

To disable specific packages:

```bash
pono disable package1
```

#### Checking Symlink Status

To check the status of all symlinks:

```bash
pono status
# OR
pono status package1 package2
```

#### Listing All Packages

To list all available packages from the TOML configuration:

```bash
pono list
```

### Help

For more detailed command usage, run:

```bash
pono --help
```

## Configuration

The configuration is defined in a `pono.toml` file. It specifies the source directories for your packages and where the symlinks should be created.

### Example `pono.toml`:

```toml
[ponos.package1]
source = "path/to/package1"
target = "/usr/local/bin"

[ponos.package2]
source = "path/to/package2"
target = "/home/user/.config"

[ponos.package3]
source = "path/to/package3"
target = "/opt/tools"
```

### Fields:

- **source**: The directory containing the files to be linked.
- **target**: The directory where the symlinks should be created.

## Contributing

### Building from Source

To build the project from source, you will need to have Rust installed on your system. You can then clone the repository and build the project using Cargo:

```bash
cargo build --release
```

### Running Tests

To run the test suite, you can use the following command:

```bash
cargo test
```

### Code Formatting

The project uses `rustfmt` for code formatting. You can run the following command to format the code:

```bash
cargo fmt
```

## License

**pono** is licensed under the [MIT License](LICENSE).

