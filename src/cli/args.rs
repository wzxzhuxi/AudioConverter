use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "audio-converter")]
#[command(about = "将音频文件转换为数组的命令行工具")]
#[command(long_about = r#"
AudioConverter - 音频文件转数组工具

这是一个强大的命令行工具，可以将各种音频格式转换为编程语言中的数组格式。

支持的音频格式：
  输入: MP3, WAV, FLAC, OGG, AAC
  输出: f32, f64, i16, i32 数组格式

使用示例：
  audio-converter -i music.mp3 -o output.rs
  audio-converter -i music.wav -o output.rs -f i16 -s 44100 -c 1
  audio-converter -i music.flac -o output.rs -g 3.0 -v
"#)]
pub struct Args {
    /// 输入音频文件路径
    #[arg(short, long)]
    pub input: Option<String>,

    /// 输出文件路径
    #[arg(short, long)]
    pub output: Option<String>,

    /// 输出数组格式
    #[arg(short, long)]
    pub format: Option<OutputFormat>,

    /// 目标采样率 (Hz)
    #[arg(short, long)]
    pub sample_rate: Option<u32>,

    /// 声道数 (1=单声道, 2=立体声)
    #[arg(short, long)]
    pub channels: Option<u32>,

    /// 音频开始时间 (秒)
    #[arg(long)]
    pub start_time: Option<f64>,

    /// 音频持续时间 (秒)
    #[arg(long)]
    pub duration: Option<f64>,

    /// 音量增益 (dB)
    #[arg(short, long)]
    pub gain: Option<f32>,

    /// 配置文件路径
    #[arg(short = 'C', long)]
    pub config: Option<String>,

    /// 详细输出
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum OutputFormat {
    /// 32位浮点
    F32,
    /// 64位浮点  
    F64,
    /// 16位整数
    I16,
    /// 32位整数
    I32,
}

