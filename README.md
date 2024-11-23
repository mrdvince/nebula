# Nebula: Sobel filter in Rust (as an FFmpeg video filter)
![Nebula](./images/car.gif)

A Sobel edge detection filter implemented in Rust as a Frei0r plugin for FFmpeg.

> This was created as an experiment to learn about FFmpeg filter development and video processing in Rust.

## What it does
Takes a video and highlights edges using the Sobel operator - basically makes your videos look like a cool sketch!

## Building
```bash
cargo build
cp target/release/libnebula.dylib ~/.frei0r-1/lib/nebula.dylib
```

## Usage
```bash
ffmpeg -i input.mp4 -vf frei0r=nebula output.mp4
```

## Notes
- Works with any video format FFmpeg supports (it'll convert it for you)
- For high-res videos (like 4K), expect some CPU workout
- HDR videos will be tone-mapped to SDR before processing
