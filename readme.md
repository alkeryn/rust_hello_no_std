this is a dumb repo just to show that doing no_std in rust is trivial\
to run just do
```bash
cargo run
# or
cargo run --release
```

you could also not have the .cargo/config.toml file if you run it like that :
```bash
cargo rustc --release -- -C link-arg=-nostartfiles && ./target/release/hello_no_std
```
