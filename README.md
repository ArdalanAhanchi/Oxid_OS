# Oxid OS
* A modern educational kernel written in Rust and Assembly.
* No dependencies besides Rust Core, and Alloc crates.
* Targets the x86_64 platform.

* `Author` : Ardalan Ahanchi
* `Date` : Winter 2021

## Features
* Virtual memory and paging
* Keyboard input
* Formatted output
* Multi-layered interrupt handling
* Pre-emptive multitasking (kernel level)
* Kernel dynamic memory allocator
* x86_64 bit Long mode operation

## Guide
* Install Rust and Cargo
* Install NASM
* Install Binutils
* Install QEMU
* Use GNU make to build and launch the kernel

## Make targets
The following targets are available for use:
* boot : Build in release mode and boot in QEMU.
* test : Build in debug mode, and launch QEMU in debug mode.
* debug: Build in debug mode, and attach a remote gdb debugger to QEMU,

## Structure
* All the architecture dependent portion of the kernel is in `src/arch`.
* The rest of the directories in `src` are higher-level code.
* The `config` directory includes configuration files needed for building.
* The `debug` script builds and launches the system with a remote GDB instance.

## Documentation
To generate the documentation website, please run the following command: 
`cargo doc --document-private-items`
