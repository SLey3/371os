# Running the RISC-V Bare-Metal OS Image

This guide explains how to build and run the bare-metal RISC-V kernel image on QEMU.

## Prerequisites

Before running the kernel, ensure you have the following installed:

- [ ] Rust with the `riscv64gc-unknown-none-elf` target
- [ ] QEMU for RISC-V (`qemu-system-riscv64`)
- [ ] Cargo build tools

### Installing the RISC-V Target

If you haven't already, install the RISC-V target for Rust:

```bash
rustup target add riscv64gc-unknown-none-elf
```

### Installing QEMU

On macOS, you can install QEMU via Homebrew:

```bash
brew install qemu
```

## Building the Kernel

Navigate to the project directory and build the bare-metal kernel:

```bash
cd hw/risc-v/hwr5
cargo build --release
```

The compiled kernel object file will be generated at the specified path.

## Running with QEMU

Once built, run the kernel using QEMU with the following command:

```bash
qemu-system-riscv64 -machine sifive_u -bios none -kernel main.main.5732f73d3dd865d-cgu.0.rcgu.o
```

### Command Breakdown

| Parameter | Description |
|-----------|-------------|
| `-machine sifive_u` | Emulates the SiFive U54 RISC-V machine |
| `-bios none` | Disables BIOS loading |
| `-kernel <path>` | Specifies the kernel image to load |

## Execution Steps

To build and run the kernel:

- [ ] Navigate to the project: `cd hw/risc-v/hwr5`
- [ ] Build the project: `cargo build --release`
- [ ] Locate the compiled kernel object file
- [ ] Run QEMU with the command above
- [ ] Exit QEMU with `Ctrl+A` then `X`

## Troubleshooting

- **QEMU not found**: Ensure QEMU is installed and in your PATH
- **Target not found**: Install the RISC-V target with `rustup`
- **Build errors**: Check that you're using a compatible Rust version