//! 音频格式处理专项测试
//! 测试各种音频格式的解码、转换和输出功能

use audio_converter::*;
use hound::{SampleFormat, WavSpec, WavWriter};
use std::io::Write;
use tempfile::NamedTempFile;

/// 创建不同规格的测试WAV文件
fn create_wav_with_spec(spec: WavSpec, duration_seconds: f32, frequency: f32) -> NamedTempFile {
    let temp_file = NamedTempFile::new().unwrap();
    let mut writer = WavWriter::create(temp_file.path(), spec).unwrap();
    let samples_per_channel = (spec.sample_rate as f32 * duration_seconds) as usize;

    for i in 0..samples_per_channel {
        let t = i as f32 / spec.sample_rate as f32;
        let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin();

        match spec.sample_format {
            SampleFormat::Int => match spec.bits_per_sample {
                16 => {
                    let amplitude = (sample * i16::MAX as f32) as i16;
                    for _ in 0..spec.channels {
                        writer.write_sample(amplitude).unwrap();
                    }
                }
                24 => {
                    let amplitude = (sample * 8388607.0) as i32; // 24-bit max
                    for _ in 0..spec.channels {
                        writer.write_sample(amplitude).unwrap();
                    }
                }
                32 => {
                    let amplitude = (sample * i32::MAX as f32) as i32;
                    for _ in 0..spec.channels {
                        writer.write_sample(amplitude).unwrap();
                    }
                }
                _ => panic!("Unsupported bit depth"),
            },
            SampleFormat::Float => {
                for _ in 0..spec.channels {
                    writer.write_sample(sample).unwrap();
                }
            }
        }
    }

    writer.finalize().unwrap();
    temp_file
}

#[test]
fn test_various_sample_rates() {
    let sample_rates = vec![8000, 16000, 22050, 44100, 48000, 96000, 192000];

    for sample_rate in sample_rates {
        let spec = WavSpec {
            channels: 2,
            sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let temp_file = create_wav_with_spec(spec, 0.1, 440.0);
        let mut decoder = AudioDecoder::new();

        let result = decoder.decode_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok(), "Failed to decode {}Hz audio", sample_rate);
        assert_eq!(decoder.get_sample_rate(), sample_rate);
        assert!(decoder.get_samples().len() > 0);
    }
}

#[test]
fn test_various_channel_configurations() {
    let channel_configs = vec![
        (1, "单声道"),
        (2, "立体声"),
        (3, "2.1声道"),
        (4, "4.0环绕声"),
        (6, "5.1环绕声"),
        (8, "7.1环绕声"),
    ];

    for (channels, description) in channel_configs {
        let spec = WavSpec {
            channels,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let temp_file = create_wav_with_spec(spec, 0.1, 440.0);
        let mut decoder = AudioDecoder::new();

        let result = decoder.decode_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok(), "Failed to decode {} audio", description);
        assert_eq!(decoder.get_channels(), channels as u32);

        // 验证样本数是声道数的整数倍
        assert_eq!(decoder.get_samples().len() % channels as usize, 0);
    }
}

#[test]
fn test_various_bit_depths() {
    let bit_depths = vec![16, 24, 32];

    for bits_per_sample in bit_depths {
        let spec = WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample,
            sample_format: SampleFormat::Int,
        };

        let temp_file = create_wav_with_spec(spec, 0.1, 440.0);
        let mut decoder = AudioDecoder::new();

        let result = decoder.decode_file(temp_file.path().to_str().unwrap());
        assert!(
            result.is_ok(),
            "Failed to decode {}-bit audio",
            bits_per_sample
        );
        assert!(decoder.get_samples().len() > 0);

        // 验证样本值在合理范围内
        for &sample in decoder.get_samples() {
            assert!(sample.abs() <= 1.1, "Sample out of range: {}", sample);
        }
    }
}

#[test]
fn test_float_format_audio() {
    let spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let temp_file = create_wav_with_spec(spec, 0.1, 440.0);
    let mut decoder = AudioDecoder::new();

    let result = decoder.decode_file(temp_file.path().to_str().unwrap());
    assert!(result.is_ok(), "Failed to decode float format audio");
    assert!(decoder.get_samples().len() > 0);
}

#[test]
fn test_extremely_short_audio() {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    // 创建仅1毫秒的音频文件
    let temp_file = create_wav_with_spec(spec, 0.001, 1000.0);
    let mut decoder = AudioDecoder::new();

    let result = decoder.decode_file(temp_file.path().to_str().unwrap());
    assert!(result.is_ok(), "Failed to decode extremely short audio");

    // 验证至少有一些样本
    assert!(decoder.get_samples().len() >= 1);
}

#[test]
fn test_format_conversion_accuracy() {
    let test_samples = vec![0.0, 0.1, -0.1, 0.5, -0.5, 0.9, -0.9];

    let formats = vec![
        OutputFormat::F32,
        OutputFormat::F64,
        OutputFormat::I16,
        OutputFormat::I32,
    ];

    for format in formats {
        let mut config = Config::default();
        config.output_format = format;

        let converter = AudioConverter::new(config);
        let result = converter.convert(&test_samples, 44100, 1);

        assert!(result.is_ok(), "Failed to convert to {:?} format", format);
        let converted = result.unwrap();

        // 验证样本数保持不变
        assert_eq!(converted.samples.len(), test_samples.len());
        assert_eq!(converted.format, format);

        // 改进的精度验证逻辑
        for (i, &original) in test_samples.iter().enumerate() {
            let converted_value = converted.samples[i];
            let error = (converted_value - original).abs();

            // 根据格式设置合理的精度期望
            let max_error = match format {
                OutputFormat::F32 => 0.0001,  // 32位浮点
                OutputFormat::F64 => 0.00001, // 64位浮点
                OutputFormat::I16 => 0.001,   // 16位整数
                OutputFormat::I32 => 0.00001, // 32位整数
            };

            assert!(
                error <= max_error,
                "Precision loss too high for {:?} format: sample[{}] original={}, converted={}, error={:.8}, max_allowed={}",
                format,
                i,
                original,
                converted_value,
                error,
                max_error
            );
        }
    }
}

#[test]
fn test_audio_format_metadata_preservation() {
    let specs = vec![
        WavSpec {
            channels: 1,
            sample_rate: 22050,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        },
        WavSpec {
            channels: 2,
            sample_rate: 48000,
            bits_per_sample: 24,
            sample_format: SampleFormat::Int,
        },
        WavSpec {
            channels: 6,
            sample_rate: 96000,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        },
    ];

    for spec in specs {
        let temp_file = create_wav_with_spec(spec, 0.1, 440.0);
        let mut decoder = AudioDecoder::new();

        let result = decoder.decode_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());

        // 验证元数据正确保存
        assert_eq!(decoder.get_sample_rate(), spec.sample_rate);
        assert_eq!(decoder.get_channels(), spec.channels as u32);

        // 测试转换后元数据输出
        let config = Config::default();
        let converter = AudioConverter::new(config);
        let converted = converter
            .convert(
                decoder.get_samples(),
                decoder.get_sample_rate(),
                decoder.get_channels(),
            )
            .unwrap();

        let temp_output = NamedTempFile::new().unwrap();
        ArrayWriter::write_to_file(&converted, temp_output.path().to_str().unwrap()).unwrap();

        let content = std::fs::read_to_string(temp_output.path()).unwrap();
        assert!(content.contains(&format!("\"sample_rate\":{}", spec.sample_rate)));
        assert!(content.contains(&format!("\"channels\":{}", spec.channels)));
    }
}

#[test]
fn test_corrupted_wav_header_handling() {
    // 创建具有损坏头部的文件
    let mut temp_file = NamedTempFile::new().unwrap();

    // 写入无效的WAV头部
    temp_file.write_all(b"RIFF\x00\x00\x00\x00WAVE").unwrap();
    temp_file.write_all(b"fmt \x10\x00\x00\x00").unwrap();
    temp_file
        .write_all(b"\x01\x00\x02\x00\x44\xAC\x00\x00")
        .unwrap();
    // 故意截断文件

    let mut decoder = AudioDecoder::new();
    let result = decoder.decode_file(temp_file.path().to_str().unwrap());

    // 应该优雅地处理错误
    assert!(result.is_err(), "Should fail on corrupted WAV header");
}
