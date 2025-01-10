# **InfinityOS**

>[!IMPORTANT]
>InfinityOS is still in very early development, is being developed on and off, and is currently not ready for normal usage.

Welcome to **InfinityOS**, an experimental operating system written in Rust. This project explores the boundaries of Rust in systems programming and serves as a platform for learning and innovation.

## **Features**
- **Rust-Powered**: Fully written in Rust, focusing on safety, performance, and modern systems programming practices.
- **Bare Metal Development**: Runs directly on hardware, without relying on an existing operating system.
- **Learning-Oriented**: Aimed at understanding OS architecture, low-level programming, and Rust's capabilities in systems development.
- **Modular Design**: Designed for extensibility and future feature development.

## Building

This project requires a nightly version of Rust because it uses some unstable features. At least nightly _2020-07-15_ is required for building. You might need to run `rustup update nightly --force` to update to the latest nightly even if some components such as `rustfmt` are missing it.

You can build the project by running:

```
cargo build
```

To create a bootable disk image from the compiled kernel, you need to install the [`bootimage`] tool:

[`bootimage`]: https://github.com/rust-osdev/bootimage

```
cargo install bootimage
```

After installing, you can create the bootable disk image by running:

```
cargo bootimage
```

This creates a bootable disk image in the `target/x86_64-infinity_os/debug` directory.

Please file an issue if you have any problems.

## Running

You can run the disk image in [QEMU] through:

[QEMU]: https://www.qemu.org/

```
cargo run
```

[QEMU] and the [`bootimage`] tool need to be installed for this.

You can also write the image to an USB stick for booting it on a real machine. On Linux, the command for this is:

```
dd if=target/x86_64-infinity_os/debug/bootimage-infinity_os.bin of=/dev/sdX && sync
```

Where `sdX` is the device name of your USB stick. **Be careful** to choose the correct device name, because everything on that device is overwritten.

## Testing

To run the unit and integration tests, execute `cargo test`.
