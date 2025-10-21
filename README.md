# reboot-force

Emergency Windows shutdown utility using `NtSetSystemPowerState`.

## What it does

Performs an immediate emergency shutdown equivalent to Ctrl+Click on the power button in Windows. This bypasses normal shutdown procedures and terminates the system immediately.

## Usage

Run from an elevated command prompt:

```cmd
reboot-force.exe
```

**Warning**: This will immediately shut down your system without saving any work or gracefully closing applications.

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
