# **InfinityOS**

>[!IMPORTANT]
>InfinityOS is still in very early development, is being developed on and off, and is currently not ready for normal usage.

Welcome to **InfinityOS**, an experimental operating system written in Rust. This project explores the boundaries of Rust in systems programming and serves as a platform for learning and innovation.

## **🛠️ Features**
- ✅ Limine Framebuffer
- ✅ Global Descriptor Table (GDT)
- ✅ Print functions
- ✅ Interrupts
- ✅ Keyboard Driver
- ✅ Memory Management
- ✅ Shell
- ✅ ACPI/AML Shutdown
- ✅ CpuId Support
- ✅ Serial Support
- ❌ Mouse Driver
- ❌ In-memory File System
- ❌ Graphical Interface (GUI)
- ❌ ELF Loader
- ❌ Task State Segment (TSS)
- ❌ Network Driver
- ❌ Audio Driver
- ❌ FAT32 Support
- ❌ OpenGL-like API
- ❌ Integrated Development Environment (IDE)
- ❌ C/C++ Compiler
- ❌ Processes
- ❌ Multitasking
- ❌ Installation Setup
- ❌ Web Browser
- ❌ User Documentation
- ❌ Package manager

## **⚙️ Building**

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

## **🚀 Running**

You can run the disk image in [QEMU] through:

[QEMU]: https://www.qemu.org/

```
cargo run
```

[QEMU] and the [`bootimage`] tool need to be installed for this.

You can also write the image to an USB stick for booting it on a real machine. On Linux, the command for this is:

```
dd if=target/x86_64-infinity_os/release/bootimage-infinity_os.bin of=/dev/sdX && sync
```

Where `sdX` is the device name of your USB stick. **Be careful** to choose the correct device name, because everything on that device is overwritten.

## **🧪 Testing**

>[!NOTE]
>You will need QEMU to test properly

To run the unit and integration tests, execute `cargo test`.

## **🤝 Contribute**
InfinityOS is an Open Source project, so everyone can contibute for it! You can help me starring/forking this project or help with code using pull requests (and optimize more!)
