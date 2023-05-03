# Asteroids

An implementation of Asteroids in Rust, using the [Macroquad](https://github.com/not-fl3/macroquad) library for rendering, input handling, sound, etc.

You can [play it in your web browser](https://caspark.github.io/asteroids-macroquad/), assuming that it hasn't bitrotted.

![Screenshot showing Asteroids game](/media/screenshot.png?raw=true "Shot of Asteroids in action")

NB: Macroquad does actually have an Asteroids example, however this was implemented without referring to that at all.

## Running

```
cargo run --release
```

## Publishing to Github Pages

```
publish-wasm.bat
```

Test it locally with:
```
cargo install basic-http-server
basic-http-server dist/
```

Once the changes are committed to master* and pushed, the changes will be live at https://caspark.github.io/asteroids-macroquad/ within a minute or two.
