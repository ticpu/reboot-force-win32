# reboot-force

Emergency Windows reboot/shutdown utility using `NtSetSystemPowerState`.

## What it does

Performs an immediate emergency reboot (or shutdown) equivalent to Ctrl+Click on the power button in Windows. This bypasses normal shutdown procedures and acts immediately.

## Usage

Run from an elevated command prompt:

```cmd
reboot-force.exe        # Emergency reboot (default)
reboot-force.exe -s     # Emergency shutdown
```

**Warning**: This will immediately reboot/shutdown your system without saving any work or gracefully closing applications.

## Build

Requires Rust and mingw-w64 toolchain:

```bash
cargo build --release
```

Binary will be located at `target/x86_64-pc-windows-gnu/release/reboot-force.exe`

## Technical Details

- Binary size: 3,584 bytes
- No standard library (`#![no_std]`)
- Direct Windows API calls via FFI
- Acquires `SeShutdownPrivilege` before shutdown
- Zero dependencies

## License

0BSD (BSD Zero Clause License)
