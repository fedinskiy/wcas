# wcas
Word Counter(https://linux.die.net/man/1/wc) as a Service.

Web application, counts symbols in a text, including and excluding spaces. Proper handling of symbols, which do not fit in two bytes is not guaranteed 🙃.

Same as https://github.com/fedinskiy/symbol-counter, but rewritten in Rust using warp and askama.

## Build:
`cargo build`


## Cross-build

### Using `cross` (probably not working anymore):
1. Install a tool for a crosslinking and cross-compilation(https://github.com/rust-embedded/cross):
`cargo install cross`
2. Build an executable file:
`cross build --target armv7-unknown-linux-gnueabihf`

### Using OS tools:
1. Install rust tools: `rustup target add aarch64-unknown-linux-gnu --toolchain stable`
2. Install linker for target architecture: `sudo apt install gcc-aarch64-linux-gnu`
3. (for new acrhitectures) Point cargo to the linker in .cargo/config.toml (see the file)
4. Do cross build: `cargo build --target aarch64-unknown-linux-gnu` or `cargo build --target aarch64-unknown-linux-gnu --release`


## Installation:
1. Build for desired architecture and type of deployment(edit makefile, if needed):
`make`
OR
`make -e 'EXEC=$(PROD)' all` for production build. Do not forget to remove `target` folder beforehead.
2. Copy file `target/wcas.tar.xz` to the target machine:
`rsync --progress target/wcas.tar.xz ${user}@${host}:${path}`
3. Unarchive and install:
`tar -xaf wcas.tar.xz`
`cd wcas`
`make install makefile`
 

Default parameters expect Raspberry Pi 4 and Linux on ARM64, but can be tuned to target any Linux system with systemd(required for autostart scripts ) and at least Tier 2 Rust support[1].

[1] https://doc.rust-lang.org/stable/rustc/platform-support.html


## REMOVAL:
1. Copy `makefile` to the target machine:
`rsync --progress makefile ${to}`
2. Run uninstall script:
`make uninstall`
