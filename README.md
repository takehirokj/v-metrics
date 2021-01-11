# vbitrate-viewer

Showing a graph of a Video bitrate on each frames.

## Building
### Dependency: FFmpeg
FFmpeg-libav* library is used to get a bitrate of each frames.
```
sudo apt install libavcodec-dev
sudo apt install libavformat-dev
sudo apt install libavfilter-dev
sudo apt install libavutil-dev
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

