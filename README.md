# wcas
Word Counter(https://linux.die.net/man/1/wc) as a Service.

Web application, counts symbols in a text, including and excluding spaces. Proper handling of symbols, which do not fit in two bytes are not guaranteed 🙃.

Same as https://github.com/fedinskiy/symbol-counter, but rewritten in Rust with warp and askama.

##Build:
`cargo build`


##Cross-build:
1. Install a tool for a crosslinking and cross-compilation(https://github.com/rust-embedded/cross)
`cargo install cross`
`cross build --target armv7-unknown-linux-gnueabihf`
