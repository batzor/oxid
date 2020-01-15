# OxidOS
A minimal OS in Rust

## Building the crate
1. You need to use `nightly` channel to build. To set it for the current directory, run  
`rustup override add nightly`
2. also, you need `xbuild` tool to build the crate for custom target.  
`cargo install cargo-xbuild`
3. `xbuild` requires `rust-src`  
`rustup component add rust-src`
4. Now, you should be able to build the crate by running  
`cargo xbuild`

## Creating bootimage
1. To link bootloader with the kernel, you need a tool named `bootimage`,  
`cargo install bootimage --version "^0.7.7"`
2. `bootimage` needs `llvm-tools-preview` component.  
`rustup component add llvm-tools-preview`
3. Now, you can create the boot image using  
`cargo bootimage`  
The bootable disk image should be created at `target/x86_64-oxid-os/debug/bootimage-oxid-os.bin`.

## Running on QEMU
1. Install QEMU  
`sudo apt install qemu` on ubuntu
2. Run using the default configuration in `.cargo/build`  
`cargo xrun`
