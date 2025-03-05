# Project overview

This is a starter project for building and running a custom QEMU-based virtual machine
based on ebcl, ebclfsa and eb-hv. The project configuration assumes that you have access
to the proprietary ebclfsa repository which includes eb-hv packages.

## Development Container

The project includes a development container configuration in devcontainer.json to provide a consistent development environment. Open the project in a compatible IDE (e.g. Visual Studio Code) and the IDE will prompt you to reopen the project in the development container.

## Building and running the Image

To build the Linux image, run the following command:

```bash
cd images/ebclfsa
make image
```

To run the Linux image in QEMU, run the following command:

```bash
cd images/ebclfsa
make qemu
```

The default username is `root` and the default password is `linux`.


## Building the example applications

To build the example applications, contiune reading the README.md in the `apps/ebclfsa-demo` directory.

## License

This starter code is provided as is, feel free to use it for your own projects. 
