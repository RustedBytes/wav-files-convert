# wav-files-convert

A simple, efficient Rust CLI tool for recursively converting audio files to standardized WAV format (Microsoft PCM, 16-bit, mono, 16kHz sample rate) using FFmpeg. Ideal for preparing audio datasets for machine learning, speech recognition, or archival purposes.

## Features

- **Recursive Processing**: Scans input directories and all subdirectories for audio files.
- **Supported Formats**: Handles common audio extensions like MP3, FLAC, OGG, M4A, AAC, WMA, AIFF, AU, MP2, and WAV.
- **Preserved Structure**: Maintains the original folder hierarchy in the output directory.
- **Custom FFmpeg Path**: Specify the path to your FFmpeg binary for flexibility across environments.
- **Error-Resilient**: Continues processing on individual file failures, with detailed logging.
- **Idiomatic Rust**: Built with safety, concurrency awareness, and minimal dependencies.

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable channel, version 1.70+ recommended)
- [FFmpeg](https://ffmpeg.org/download.html) (version 4.0+ for full codec support)

### From Source
Clone the repository and build with Cargo:

```bash
git clone https://github.com/RustedBytes/wav-files-convert.git
cd wav-files-convert
cargo build --release
```

The binary will be available at `target/release/wav-files-convert`.

## Usage

```bash
wav-files-convert [OPTIONS] <INPUT> <OUTPUT>
```

- **`<INPUT>`**: Path to the input directory containing audio files (required).
- **`<OUTPUT>`**: Path to the output directory for converted WAV files (required).
- **`-f, --ffmpeg-bin <FFMPEG_BIN>`**: Path to the FFmpeg executable (default: `ffmpeg`).

Run with `--help` for full options.

## Examples

### Basic Conversion
Convert all audio files in `./audio-input/` to `./audio-output/`:

```bash
wav-files-convert ./audio-input ./audio-output
```

### Custom FFmpeg Path
If FFmpeg is not in your PATH:

```bash
wav-files-convert ./audio-input ./audio-output --ffmpeg-bin /usr/local/bin/ffmpeg
```

### Processing with Subfolders
The tool automatically handles nested directories, e.g.:

```
audio-input/
├── song.mp3
└── subfolder/
    └── track.flac
```

Results in:

```
audio-output/
├── song.wav
└── subfolder/
    └── track.wav
```

Each WAV file will be 16-bit PCM, mono, 16kHz.

## Output Details

- **Format**: WAV (RIFF) with Microsoft PCM encoding.
- **Channels**: Mono (`-ac 1`).
- **Sample Rate**: 16kHz (`-ar 16000`).
- **Bit Depth**: 16-bit signed little-endian (`-acodec pcm_s16le`).

FFmpeg handles resampling, channel mixing, and format conversion transparently.

## Troubleshooting

- **FFmpeg Not Found**: Ensure FFmpeg is installed and accessible. Use `--ffmpeg-bin` to specify the full path.
- **Permission Errors**: Run with appropriate read/write permissions on input/output directories.
- **Unsupported Formats**: Add extensions to the source code's `AUDIO_EXTENSIONS` constant if needed.
- **Conversion Failures**: Check console output for per-file errors (e.g., corrupted input).

## Development

This project is maintained under the [RustedBytes](https://github.com/RustedBytes) organization.

### Building and Testing
```bash
cargo build
cargo test
cargo fmt  # Ensure code style
cargo clippy -- -D warnings  # Lint
```

### Dependencies
- `clap`: Argument parsing.
- `anyhow`: Error handling.

See `Cargo.toml` for versions.

### Contributing
1. Fork the repo and create a feature branch (`git checkout -b feat/amazing-feature`).
2. Commit changes (`git commit -m 'Add amazing feature'`).
3. Push to the branch (`git push origin feat/amazing-feature`).
4. Open a Pull Request.

We welcome bug reports, features, and documentation improvements!

## License

MIT License. See [LICENSE](LICENSE) for details.

## Cite

```
@software{Smoliakov_Wav_Files_Toolkit,
  author = {Smoliakov, Yehor},
  month = oct,
  title = {{WAV Files Toolkit: A suite of command-line tools for common WAV audio processing tasks, including conversion from other formats, data augmentation, loudness normalization, spectrogram generation, and validation.}},
  url = {https://github.com/RustedBytes/wav-files-toolkit},
  version = {0.4.0},
  year = {2025}
}
```
