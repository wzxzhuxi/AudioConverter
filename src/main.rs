mod audio;
mod cli;
mod config;
mod output;

use clap::{Parser, CommandFactory};
use cli::args::Args;
use crate::config::settings::Config;
use audio::{decoder::AudioDecoder, converter::AudioConverter};
use output::array_writer::ArrayWriter;
use anyhow::Result;

fn main() -> Result<()> {

    // 检查是否没有任何参数
    let args_vec: Vec<String> = std::env::args().collect();
    if args_vec.len() == 1 {
        // 没有参数时显示 help
        let mut cmd = Args::command();
        cmd.print_help().unwrap();
        std::process::exit(0);
    }

    let args = Args::parse();

    // 验证必需参数
    if args.input.is_none() {
        eprintln!("错误: 缺少输入文件参数 (-i 或 --input)");
        eprintln!("使用 --help 查看完整帮助信息");
        std::process::exit(1);
    }

    if args.output.is_none() {
        eprintln!("错误: 缺少输出文件参数 (-o 或 --output)");
        eprintln!("使用 --help 查看完整帮助信息");
        std::process::exit(1);
    }

    // 获取已验证的参数
    let input_path = args.input.as_ref().unwrap();
    let output_path = args.output.as_ref().unwrap();

    // 加载配置
    let mut config = if let Some(config_path) = &args.config {
        Config::from_file(config_path)?
    } else {
        Config::default()
    };

    // 合并命令行参数
    config.merge_with_args(&args);

    // 如果没有设置增益，使用默认值 0.0
    if args.gain.is_none() && config.gain == 0.0 {
        // 保持默认值 0.0
    }

    if args.verbose {
        println!("配置: {:#?}", config);
    }

    // 解码音频文件
    println!("正在解码音频文件: {}", input_path);
    let mut decoder = AudioDecoder::new();
    decoder.decode_file(input_path)?;

    println!("解码完成: 采样率={}Hz, 声道数={}, 样本数={}", 
             decoder.get_sample_rate(), 
             decoder.get_channels(), 
             decoder.get_samples().len());

    // 转换音频
    let converter = AudioConverter::new(config);
    let converted = converter.convert(
        decoder.get_samples(),
        decoder.get_sample_rate(),
        decoder.get_channels()
    )?;

    println!("转换完成: 采样率={}Hz, 声道数={}, 样本数={}", 
             converted.sample_rate, 
             converted.channels, 
             converted.samples.len());

    // 输出数组
    ArrayWriter::write_to_file(&converted, output_path)?;

    println!("处理完成！");
    Ok(())
}

