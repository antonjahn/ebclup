# ebclup

Kickstart projects based on EB corbos templates and examples.

## Installation

Currently only supports Linux.

Download latest release from https://github.com/antonjahn/ebclup/releases

```bash
chmod +x ebclup
sudo mv ebclup /usr/local/bin/
```

## Usage

```bash
ebclup --help
```

Create example project based on ebcl for qemu:

```bash
ebclup startproject ebcl hello
```

Then open the project in a devcontainer compatible IDE like VSCode.

```bash
code hello
# Follow instructions to open in devcontainer
# Follow instructions in README.md
```

## Development

To release a new version, update version number in Cargo.toml and run:

```bash
cargo release
<dry-run output>
cargo release --no-publish --execute
```

For more information see https://github.com/crate-ci/cargo-release
