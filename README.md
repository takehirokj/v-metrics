# vbitrate-viewer [![Actions Status](https://github.com/takehirokj/vbitrate-viewer/workflows/vbitrate-viewer/badge.svg)](https://github.com/takehirokj/vbitrate-viewer/actions)

Showing a graph of a Video bitrate on each frames.

## Building
### Dependency: FFmpeg
FFmpeg-libav* library is used to get a bitrate of each frames.
```
sudo apt install libavcodec-dev libavformat-dev libavfilter-dev libavutil-dev
```

### Release binary
To build release inary in target/release/vbitrate-viewer

## Usage
```
cargo run --release -- -i test.mp4
```

## Contributing
### Coding style
Format code with rustfmt before submitting a PR.
```
cargo fmt
```

## Code Analysis
Use [clippy](https://github.com/rust-lang/rust-clippy) before submitting a PR.
```
cargo clippy
```

## Testing
Run unit test.
```
cargo test
```

