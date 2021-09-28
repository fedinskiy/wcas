# wcas
Word Counter(https://linux.die.net/man/1/wc) as a Service.

Web application, counts symbols in a text, including and excluding spaces. Proper handling of symbols, which do not fit in two bytes are not guaranteed 🙃.

Same as https://github.com/fedinskiy/symbol-counter, but rewritten in Rust with warp and askama.

## Build:
`cargo build`


## Cross-build:
1. Install a tool for a crosslinking and cross-compilation(https://github.com/rust-embedded/cross):
`cargo install cross`
2. Build an executable file:
`cross build --target armv7-unknown-linux-gnueabihf`


## Installation:
1. Build for desired architecture and type of deployment(edit makefile, if needed):
`make`
OR
`make EXEC=$(PROD) all` for production build
2. Copy file `target/wcas.tar.xz` to the target machine:
`rsync --progress target/wcas.tar.xz ${to}`
3. Unarchive and install:
`tar -xaf wcas.tar.xz`
`make install -f wcas/makefile`

Default parameters expect Raspberry Pi 4 and Raspbian buster, but can be tuned to target any Linux system with systemd(required for autostart scripts ) and at least Tier 2 Rust support[1].

[1] https://doc.rust-lang.org/stable/rustc/platform-support.html


## REMOVAL:
1. Copy `makefile` to the target machine:
`rsync --progress makefile ${to}`
2. Run uninstall script:
`make uninstall`
