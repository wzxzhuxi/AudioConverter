//! AudioConverter 库
//!
//! 提供音频文件转换为数组格式的核心功能

pub mod audio;
pub mod cli;
pub mod config;
pub mod output;

pub use audio::{converter::AudioConverter, decoder::AudioDecoder};
pub use cli::args::{Args, OutputFormat};
pub use config::settings::Config;
pub use output::array_writer::ArrayWriter;
