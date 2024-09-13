# slot - Symbolic Link Organizer with TOML

**slot** is a lightweight command-line tool for managing symbolic links in your filesystem inspired by `stow`. 

## Why?

Because gnu stow almost does what I want, but not quite. I wanted a simpler tool that could manage symlinks for multiple packages with a single configuration file.
I need to be able to toogle links on demand to apply different configs.

## Demo

Create the `slot.toml` in the current directory
```toml
[packages]
nvim = { source = "./examples/source/nvim", target = "./examples/target/nvim" }
zsh = { source = "./examples/source/zshrc", target = "./examples/target/.zshrc" }
```
And run
```bash
slot link
Linking packages
  nvim: ./examples/target/nvim (new link)
  zsh: ./examples/target/.zshrc (new link)

ls -la examples/target                                                                                                                                                     [1:00:35]
total 0
drwxr-xr-x 4 cris 128 Sep 14 01:00 .
drwxr-xr-x 5 cris 160 Sep 14 01:00 ..
lrwxr-xr-x 1 cris  58 Sep 14 01:00 .zshrc -> /Users/cristianoliveira/other/slot/./examples/source/zshrc
lrwxr-xr-x 1 cris  57 Sep 14 01:00 nvim -> /Users/cristianoliveira/other/slot/./examples/source/nvim
cr
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
- Manage same link with diffrent source and toogle between them.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Examples](#examples)
- [License](#license)

## Installation

### With Cargo

You can install **Slot** using [Cargo](https://doc.rust-lang.org/cargo/):

```bash
cargo install slot-cli
```

## Usage

The **Slot** CLI allows you to install, remove, and check symlinks based on a TOML configuration file.

### Commands

- `link`: Create symbolic links for the specified packages.
- `unlink`: Remove symbolic links for the specified packages.
- `status`: Check the status of symbolic links.
- `list`: Display all available packages from the TOML configuration.

### Options

- `-c --config <file>`: Specify a custom TOML configuration file (default: `./slot.toml`).
- `--help`: Display help information.

### Basic Usage

#### Installing Symlinks

To create symlinks for all packages defined in `slot.toml`:

```bash
slot link
```

To link symlinks for specific packages:

```bash
slot link package1 package2
```

#### Uninstalling Symlinks

To remove symlinks for all packages:

```bash
slot unlink
```

To unlink specific packages:

```bash
slot unlink package1
```

#### Checking Symlink Status

To check the status of all symlinks:

```bash
slot status
# OR
slot status package1 package2
```

#### Listing All Packages

To list all available packages from the TOML configuration:

```bash
slot list
```

### Help

For more detailed command usage, run:

```bash
slot --help
```

## Configuration

The configuration is defined in a `slot.toml` file. It specifies the source directories for your packages and where the symlinks should be created.

### Example `slot.toml`:

```toml
[packages.package1]
source = "path/to/package1"
target = "/usr/local/bin"

[packages.package2]
source = "path/to/package2"
target = "/home/user/.config"

[packages.package3]
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

**Slot** is licensed under the [MIT License](LICENSE).

