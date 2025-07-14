//! 生成测试音频文件的工具

use hound::{SampleFormat, WavSpec, WavWriter};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("正在生成测试音频文件...");

    // 1. 生成单声道16位WAV文件
    generate_mono_wav("audio/test_mono_16bit.wav", 44100, 1.0)?;

    // 2. 生成立体声44100Hz WAV文件
    generate_stereo_wav("audio/test_stereo_44100.wav", 44100, 2.0)?;

    // 3. 生成短时长WAV文件
    generate_mono_wav("audio/test_short.wav", 22050, 0.1)?;

    // 4. 生成静音WAV文件
    generate_silence_wav("audio/test_silence.wav", 44100, 1.0)?;

    // 5. 生成不同采样率的文件
    generate_mono_wav("audio/test_48khz.wav", 48000, 1.0)?;
    generate_mono_wav("audio/test_22khz.wav", 22050, 1.0)?;

    println!("✅ 测试音频文件生成完成！");
    Ok(())
}

fn generate_mono_wav(
    filename: &str,
    sample_rate: u32,
    duration: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(filename, spec)?;
    let samples = (sample_rate as f32 * duration) as usize;

    for i in 0..samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude)?;
    }

    writer.finalize()?;
    println!("生成: {}", filename);
    Ok(())
}

fn generate_stereo_wav(
    filename: &str,
    sample_rate: u32,
    duration: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(filename, spec)?;
    let samples = (sample_rate as f32 * duration) as usize;

    for i in 0..samples {
        let t = i as f32 / sample_rate as f32;
        let left = (t * 440.0 * 2.0 * PI).sin();
        let right = (t * 880.0 * 2.0 * PI).sin();

        let left_amplitude = (left * i16::MAX as f32) as i16;
        let right_amplitude = (right * i16::MAX as f32) as i16;

        writer.write_sample(left_amplitude)?;
        writer.write_sample(right_amplitude)?;
    }

    writer.finalize()?;
    println!("生成: {}", filename);
    Ok(())
}

fn generate_silence_wav(
    filename: &str,
    sample_rate: u32,
    duration: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(filename, spec)?;
    let samples = (sample_rate as f32 * duration) as usize;

    for _ in 0..samples {
        writer.write_sample(0i16)?; // 左声道
        writer.write_sample(0i16)?; // 右声道
    }

    writer.finalize()?;
    println!("生成: {}", filename);
    Ok(())
}
