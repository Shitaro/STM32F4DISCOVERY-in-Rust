# STM32 in Rust

This is a project to develop [STM32F4DISCOVERY](https://www.st.com/en/evaluation-tools/stm32f4discovery.html "STM32F4DISCOVERY") in Rust.

# Installation (Arch Linux)


Add the target to the Rust toolchain for Cortex-M4F and M7F with FPU support.
> STM32F407VGT6 microcontroller featuring 32-bit ARM®Cortex®-M4 with FPU core

```sh
$ rustup target add thumbv7em-none-eabihf
```

[`cargo-binutils`](https://github.com/rust-embedded/cargo-binutils)

```sh
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

Debugger for ARM Cortex-M programs

```sh
$ yay -S arm-none-eabi-gdb
```

[OpenOCD](http://openocd.org/)

```sh
$ yay -S openocd
```

# Usage

Build the project

```sh
$ cargo build
```

Run OpenOCD to connect to the ST-LINK on the discovery board on a terminal

```sh
$ openocd
```

Run GDB on another terminal.

```sh
$ cargo run
```

# Reference

[The Embedded Rust Book](https://rust-embedded.github.io/book/)