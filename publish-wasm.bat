@REM wow I miss shell scripts - never thought I'd say that.
@REM but I guess it is what it is.

@REM Anyway, this allows publishing to github pages. It's a little weird abusing the docs/ dir
@REM but it sure is convenient, and really, nobody is going to care for this little thing... right?

del /Q .\docs\*

cargo build --target wasm32-unknown-unknown --release
copy .\target\wasm32-unknown-unknown\release\asteroids-macroquad.wasm .\docs\asteroids-macroquad.wasm

mkdir .\docs\assets
copy .\assets\*.wav .\docs\assets\

copy .\src\index.html .\docs\index.html
