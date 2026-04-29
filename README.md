# asciicam

webcam -> ascii art, rendered in a browser.

## Build

```
wasm-pack build --target web --release --out-dir web/pkg --no-pack
cargo build --release
```

## Run

```
./target/release/asciicam 1234  # hosts on http://localhost:1234
```

open the URL, click **start camera**, grant permission, enjoy :p
