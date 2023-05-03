# Asteroids

An implementation of Asteroids in Rust, using the [Macroquad](https://github.com/not-fl3/macroquad) library for rendering, input handling, sound, etc.

NB: Macroquad does actually have an Asteroids example, however this was implemented without referring to that at all.

## Running

```
cargo run --release
```

## Publishing to web

```
publish-wasm.bat
```

Test it with:
```
cargo install basic-http-server
basic-http-server dist/
```
