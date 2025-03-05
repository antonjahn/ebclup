# ebclup

Utility script for EB corbos Linux projects to start new projects.

## Installation

Currently only supports Linux.

Download latest release from https://github.com/antonjahn/ebclup/releases

chmod +x ebclup

sudo mv ebclup /usr/local/bin/

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
