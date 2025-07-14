# AudioConverter - 音频文件转数组工具

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)[![Tests](https://img.shields.io/badge/tests-95%25-green.svg)](tests/)
[English](README_en.md) | [简体中文](README.md)

一个强大的命令行工具，可以将各种音频格式转换为编程语言中的数组格式。支持多种音频格式和输出类型，具有高性能和丰富的配置选项。

## ✨ 主要特性

- 🎵 **多格式支持**: MP3, WAV, FLAC, OGG, AAC
- 🔢 **多种输出格式**: f32, f64, i16, i32 数组
- ⚡ **高性能处理**: 微秒级音频处理速度
- 🎛️ **音频处理**: 重采样、声道转换、音量调节
- ⚙️ **配置文件支持**: JSON 配置文件
- 🧪 **全面测试**: 95%+ 测试覆盖率
- 📊 **性能优化**: Release 模式下 10-100x 性能提升

## 🚀 快速开始

### 安装

```
# 克隆仓库
git clone https://github.com/wzxzhuxi/AudioConverter.git
cd AudioConverter

# 编译 Release 版本
cargo build --release
```

### 基本用法

```
# 基本转换
./target/release/audio-converter -i music.mp3 -o output.rs

# 指定输出格式
./target/release/audio-converter -i music.wav -o output.rs -f i16

# 重采样和声道转换
./target/release/audio-converter -i music.flac -o output.rs -s 44100 -c 1

# 使用配置文件
./target/release/audio-converter -i music.mp3 -o output.rs -C config.json
```

## 📖 详细使用说明

### 命令行参数

| 参数 | 长参数 | 描述 | 示例 |
|------|--------|------|------|
| `-i` | `--input` | 输入音频文件路径 | `-i music.mp3` |
| `-o` | `--output` | 输出文件路径 | `-o output.rs` |
| `-f` | `--format` | 输出数组格式 [f32, f64, i16, i32] | `-f i16` |
| `-s` | `--sample-rate` | 目标采样率 (Hz) | `-s 44100` |
| `-c` | `--channels` | 声道数 (1=单声道, 2=立体声) | `-c 1` |
| `-g` | `--gain` | 音量增益 (dB) | `-g 6.0` |
| `-C` | `--config` | 配置文件路径 | `-C config.json` |
| `-v` | `--verbose` | 详细输出 | `-v` |

### 支持的音频格式

**输入格式**:
- MP3 (MPEG Audio Layer III)
- WAV (Waveform Audio File Format)
- FLAC (Free Lossless Audio Codec)
- OGG (Ogg Vorbis)
- AAC (Advanced Audio Coding)

**输出格式**:
- `f32`: 32位浮点数组
- `f64`: 64位浮点数组
- `i16`: 16位整数数组
- `i32`: 32位整数数组

### 配置文件示例

创建 `config.json` 文件：

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

## 📊 使用示例

### 基础转换示例

```
# 转换 MP3 到 f32 数组
audio-converter -i song.mp3 -o song_data.rs

# 转换 WAV 到 i16 数组
audio-converter -i audio.wav -o audio_data.rs -f i16
```

### 音频处理示例

```
# 重采样到 22050Hz
audio-converter -i music.flac -o music_data.rs -s 22050

# 立体声转单声道
audio-converter -i stereo.mp3 -o mono_data.rs -c 1

# 应用 6dB 音量增益
audio-converter -i quiet.wav -o loud_data.rs -g 6.0
```

### 高级用法示例

```
# 组合多个参数
audio-converter -i input.ogg -o output.rs -f i32 -s 48000 -c 2 -g 3.0

# 使用配置文件
audio-converter -i music.mp3 -o output.rs -C my_config.json

# 详细输出模式
audio-converter -i music.mp3 -o output.rs -v
```

## 🧪 开发和测试

### 运行测试

```
# 运行所有测试
cargo test

# 运行特定测试套件
cargo test --test cli_tests
cargo test --test audio_format_tests

# 性能测试 (Release 模式)
cargo test --release --test performance_tests

# 基准测试
cargo bench
```

### 测试覆盖率

```
# 生成覆盖率报告
cargo tarpaulin --out html --output-dir coverage

# 查看报告
open coverage/tarpaulin-report.html
```

### 项目结构

```
AudioConverter/
├── src/
│   ├── main.rs              \# 主程序入口
│   ├── lib.rs               \# 库接口
│   ├── audio/               \# 音频处理模块
│   │   ├── decoder.rs       \# 音频解码器
│   │   └── converter.rs     \# 音频转换器
│   ├── cli/                 \# 命令行接口
│   │   └── args.rs          \# 参数解析
│   ├── config/              \# 配置管理
│   │   └── settings.rs      \# 配置结构
│   └── output/              \# 输出处理
│       └── array_writer.rs  \# 数组输出器
├── tests/                   \# 测试文件
├── test_data/              \# 测试数据
└── benches/                \# 基准测试
```

## ⚡ 性能特性

- **微秒级处理**: 核心音频操作在微秒到毫秒级完成
- **内存高效**: 优化的内存使用模式
- **并发支持**: 支持多线程音频处理
- **零拷贝优化**: 最小化内存分配和拷贝

### 性能基准

| 操作类型 | 处理时间 | 样本数量 |
|----------|----------|----------|
| 基础转换 | ~65 ns | 1,024 样本 |
| 重采样 | ~87 μs | 44.1k→22k |
| 声道转换 | ~44 μs | 立体声→单声道 |
| 格式转换 | ~3.2 μs | 44.1k 样本 |

## 🤝 贡献指南

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 确保所有测试通过 (`cargo test`)
- 添加适当的文档注释

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Symphonia](https://github.com/pdeljanov/Symphonia) - 音频解码库
- [Hound](https://github.com/ruuda/hound) - WAV 文件处理
- [Clap](https://github.com/clap-rs/clap) - 命令行参数解析

---

**作者**: [老王]  
**邮箱**: w1355457260@gmailcom  
**主页**: https://github.com/wzxzhuxi
