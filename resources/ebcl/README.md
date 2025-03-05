# Project overview

This is a starter project for building and running a custom QEMU-based virtual machine
based on ebcl and crinit.

## Development Container

The project includes a development container configuration in devcontainer.json to provide a consistent development environment. Open the project in a compatible IDE (e.g. Visual Studio Code) and the IDE will prompt you to reopen the project in the development container.

## Building and running the Linux image

To build the Linux image, run the following command:

```bash
make image
```

To run the Linux image in QEMU, run the following command:

```bash
make qemu
```

The default username is `root` and the default password is `linux`.


## License

This starter code is provided as is, feel free to use it for your own projects. 
