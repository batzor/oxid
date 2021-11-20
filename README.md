# OxidOS
A minimal OS in Rust

## Running on QEMU
To just build:  
`cargo build`

To link bootloader with the kernel, you need a tool named `bootimage`,  
`cargo install bootimage`

`bootimage` needs `llvm-tools-preview` component.  
`rustup component add llvm-tools-preview`

Now, you can create the boot image using  
`cargo bootimage`  
The bootable disk image should be created at `target/x86_64-oxid-os/debug/bootimage-oxid-os.bin`.

To run on QEMU first install qemu
`sudo apt install qemu`
NOTE: on WSL, you may need to
`sudo apt install qemu-system-x86`

To run using the default configuration in `.cargo/build`  
`cargo run`
