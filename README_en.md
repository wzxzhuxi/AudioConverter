# AudioConverter - Audio File to Array Converter

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)[![Tests](https://img.shields.io/badge/tests-95%25-green.svg)](tests/)
[English](README_en.md) | [简体中文](README.md)

A powerful command-line tool that converts various audio formats into array formats for programming languages. Supports multiple audio formats and output types with high performance and rich configuration options.

## ✨ Key Features

- 🎵 **Multiple Format Support**: MP3, WAV, FLAC, OGG, AAC
- 🔢 **Various Output Formats**: f32, f64, i16, i32 arrays
- ⚡ **High Performance**: Microsecond-level audio processing
- 🎛️ **Audio Processing**: Resampling, channel conversion, volume control
- ⚙️ **Configuration File Support**: JSON configuration files
- 🧪 **Comprehensive Testing**: 95%+ test coverage
- 📊 **Performance Optimized**: 10-100x performance boost in Release mode

## 🚀 Quick Start

### Installation

```
# Clone the repository
git clone https://github.com/wzxzhuxi/AudioConverter.git
cd AudioConverter

# Build Release version
cargo build --release
```

### Basic Usage

```
# Basic conversion
./target/release/audio-converter -i music.mp3 -o output.rs

# Specify output format
./target/release/audio-converter -i music.wav -o output.rs -f i16

# Resampling and channel conversion
./target/release/audio-converter -i music.flac -o output.rs -s 44100 -c 1

# Using configuration file
./target/release/audio-converter -i music.mp3 -o output.rs -C config.json
```

## 📖 Detailed Usage

### Command Line Arguments

| Short | Long | Description | Example |
|-------|------|-------------|---------|
| `-i` | `--input` | Input audio file path | `-i music.mp3` |
| `-o` | `--output` | Output file path | `-o output.rs` |
| `-f` | `--format` | Output array format [f32, f64, i16, i32] | `-f i16` |
| `-s` | `--sample-rate` | Target sample rate (Hz) | `-s 44100` |
| `-c` | `--channels` | Number of channels (1=mono, 2=stereo) | `-c 1` |
| `-g` | `--gain` | Volume gain (dB) | `-g 6.0` |
| `-C` | `--config` | Configuration file path | `-C config.json` |
| `-v` | `--verbose` | Verbose output | `-v` |

### Supported Audio Formats

**Input Formats**:
- MP3 (MPEG Audio Layer III)
- WAV (Waveform Audio File Format)
- FLAC (Free Lossless Audio Codec)
- OGG (Ogg Vorbis)
- AAC (Advanced Audio Coding)

**Output Formats**:
- `f32`: 32-bit floating point array
- `f64`: 64-bit floating point array
- `i16`: 16-bit integer array
- `i32`: 32-bit integer array

### Configuration File Example

Create `config.json` file:

```
{
"output_format": "I16",
"sample_rate": 44100,
"channels": 2,
"gain": 3.0,
"normalize": true,
"output_settings": {
"array_type": "Vec",
"include_metadata": true,
"compress": false
}
}
```

## 📊 Usage Examples

### Basic Conversion Examples

```
# Convert MP3 to f32 array
audio-converter -i song.mp3 -o song_data.rs

# Convert WAV to i16 array
audio-converter -i audio.wav -o audio_data.rs -f i16
```

### Audio Processing Examples

```
# Resample to 22050Hz
audio-converter -i music.flac -o music_data.rs -s 22050

# Convert stereo to mono
audio-converter -i stereo.mp3 -o mono_data.rs -c 1

# Apply 6dB volume gain
audio-converter -i quiet.wav -o loud_data.rs -g 6.0
```

### Advanced Usage Examples

```
# Combine multiple parameters
audio-converter -i input.ogg -o output.rs -f i32 -s 48000 -c 2 -g 3.0

# Use configuration file
audio-converter -i music.mp3 -o output.rs -C my_config.json

# Verbose output mode
audio-converter -i music.mp3 -o output.rs -v
```

## 🧪 Development and Testing

### Running Tests

```
# Run all tests
cargo test

# Run specific test suites
cargo test --test cli_tests
cargo test --test audio_format_tests

# Performance tests (Release mode)
cargo test --release --test performance_tests

# Benchmark tests
cargo bench
```

### Test Coverage

```
# Generate coverage report
cargo tarpaulin --out html --output-dir coverage

# View report
open coverage/tarpaulin-report.html
```

### Project Structure

```
AudioConverter/
├── src/
│   ├── main.rs              \# Main program entry
│   ├── lib.rs               \# Library interface
│   ├── audio/               \# Audio processing modules
│   │   ├── decoder.rs       \# Audio decoder
│   │   └── converter.rs     \# Audio converter
│   ├── cli/                 \# Command line interface
│   │   └── args.rs          \# Argument parsing
│   ├── config/              \# Configuration management
│   │   └── settings.rs      \# Configuration structures
│   └── output/              \# Output processing
│       └── array_writer.rs  \# Array writer
├── tests/                   \# Test files
├── test_data/              \# Test data
└── benches/                \# Benchmark tests
```

## ⚡ Performance Features

- **Microsecond Processing**: Core audio operations complete in microseconds to milliseconds
- **Memory Efficient**: Optimized memory usage patterns
- **Concurrent Support**: Multi-threaded audio processing support
- **Zero-Copy Optimization**: Minimal memory allocation and copying

### Performance Benchmarks

| Operation Type | Processing Time | Sample Count |
|----------------|----------------|--------------|
| Basic Conversion | ~65 ns | 1,024 samples |
| Resampling | ~87 μs | 44.1k→22k |
| Channel Conversion | ~44 μs | Stereo→Mono |
| Format Conversion | ~3.2 μs | 44.1k samples |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Standards

- Use `cargo fmt` to format code
- Use `cargo clippy` for code quality checks
- Ensure all tests pass (`cargo test`)
- Add appropriate documentation comments

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Symphonia](https://github.com/pdeljanov/Symphonia) - Audio decoding library
- [Hound](https://github.com/ruuda/hound) - WAV file processing
- [Clap](https://github.com/clap-rs/clap) - Command line argument parsing

---

**Author**: [Lao Wang]  
**Email**: w1355457260@gmail.com  
**Homepage**: https://github.com/wzxzhuxi

