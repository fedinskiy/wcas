# wcas
Word Counter(https://linux.die.net/man/1/wc) as a Service.

Web application, counts symbols in a text, including and excluding spaces. Proper handling of symbols, which do not fit in two bytes is not guaranteed 🙃.

Same as https://github.com/fedinskiy/symbol-counter, but rewritten in Rust using warp and askama.

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
`make -e 'EXEC=$(PROD)' all` for production build. Do not forget to remove `target` folder beforehead.
2. Copy file `target/wcas.tar.xz` to the target machine:
`rsync --progress target/wcas.tar.xz ${user}@${host}:${path}`
3. Unarchive and install:
`tar -xaf wcas.tar.xz`
`cd wcas`
`make install makefile`
 
 Points 3 and 4 can be replaced by running `ansible-playbook install/install.yml` if Ansible is installed on the host machine.

Default parameters expect Raspberry Pi 4 and Raspbian Buster, but can be tuned to target any Linux system with systemd(required for autostart scripts ) and at least Tier 2 Rust support[1].

[1] https://doc.rust-lang.org/stable/rustc/platform-support.html


## REMOVAL:
1. Copy `makefile` to the target machine:
`rsync --progress makefile ${to}`
2. Run uninstall script:
`make uninstall`
