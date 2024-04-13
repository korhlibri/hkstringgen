# hkstringgen-cli
A simple random string generator written in Rust, using system randomness and mouse movement.

## Security Notes
No security audits of this program have ever been performed, and it has not been thoroughly assessed to be considered secure.

USE AT YOUR OWN RISK!

## Compile
This program can be compiled using Cargo (the Rust package manager).
```
cargo build
```
or
```
cargo build --release
```
for optimized artifacts.
## Run
After compiling, the executable should be located at `./target/debug/hkstringgen-cli` or `./target/release/hkstringgen-cli` for the release version (Windows versions will include the `.exe` file extension).

You can also run the executable with Cargo.
```
cargo run
```
Alternatively, you can find the executables in the release version.
