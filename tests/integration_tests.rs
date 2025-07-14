use audio_converter::*;
use std::fs;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn test_full_wav_conversion_workflow() {
    // 创建测试WAV文件
    let temp_audio = create_test_wav_file(44100, 2, 0.1);
    let temp_output = NamedTempFile::new().unwrap();

    // 设置配置
    let config = Config::default();

    // 解码
    let mut decoder = AudioDecoder::new();
    decoder
        .decode_file(temp_audio.path().to_str().unwrap())
        .unwrap();

    // 转换
    let converter = AudioConverter::new(config);
    let converted = converter
        .convert(
            decoder.get_samples(),
            decoder.get_sample_rate(),
            decoder.get_channels(),
        )
        .unwrap();

    // 输出
    ArrayWriter::write_to_file(&converted, temp_output.path().to_str().unwrap()).unwrap();

    // 验证输出文件
    let output_content = fs::read_to_string(temp_output.path()).unwrap();
    assert!(output_content.contains("const AUDIO_SAMPLES"));
    assert!(output_content.contains("sample_rate"));
    assert!(output_content.contains("channels"));
}

#[test]
fn test_config_file_integration() {
    // 创建测试配置文件
    let config_content = r#"
    {
        "output_format": "I16",
        "sample_rate": 22050,
        "channels": 1,
        "gain": 3.0,
        "normalize": true,
        "output_settings": {
            "array_type": "Vec",
            "include_metadata": true,
            "compress": false
        }
    }
    "#;

    let mut config_file = NamedTempFile::new().unwrap();
    config_file.write_all(config_content.as_bytes()).unwrap();

    // 测试配置文件加载
    let config = Config::from_file(config_file.path().to_str().unwrap()).unwrap();
    assert!(matches!(config.output_format, OutputFormat::I16));
    assert_eq!(config.sample_rate, Some(22050));
    assert_eq!(config.channels, Some(1));
    assert_eq!(config.gain, 3.0);
    assert!(config.normalize);
}

#[test]
fn test_multiple_format_conversions() {
    let temp_audio = create_test_wav_file(44100, 2, 0.1);
    let temp_dir = TempDir::new().unwrap();

    let formats = vec![
        OutputFormat::F32,
        OutputFormat::F64,
        OutputFormat::I16,
        OutputFormat::I32,
    ];

    for format in formats {
        let mut config = Config::default();
        config.output_format = format.clone();

        let mut decoder = AudioDecoder::new();
        decoder
            .decode_file(temp_audio.path().to_str().unwrap())
            .unwrap();

        let converter = AudioConverter::new(config);
        let converted = converter
            .convert(
                decoder.get_samples(),
                decoder.get_sample_rate(),
                decoder.get_channels(),
            )
            .unwrap();

        let output_path = temp_dir.path().join(format!("output_{:?}.rs", format));
        ArrayWriter::write_to_file(&converted, output_path.to_str().unwrap()).unwrap();

        // 验证输出文件存在并包含正确格式
        let content = fs::read_to_string(&output_path).unwrap();
        match format {
            OutputFormat::F32 => assert!(content.contains("[f32;")),
            OutputFormat::F64 => assert!(content.contains("[f64;")),
            OutputFormat::I16 => assert!(content.contains("[i16;")),
            OutputFormat::I32 => assert!(content.contains("[i32;")),
        }
    }
}

// 辅助函数：创建测试WAV文件
fn create_test_wav_file(sample_rate: u32, channels: u16, duration_seconds: f32) -> NamedTempFile {
    let temp_file = NamedTempFile::new().unwrap();
    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(temp_file.path(), spec).unwrap();
    let samples_per_channel = (sample_rate as f32 * duration_seconds) as usize;

    for i in 0..samples_per_channel {
        let sample = (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / sample_rate as f32).sin();
        let amplitude = (sample * i16::MAX as f32) as i16;
        for _ in 0..channels {
            writer.write_sample(amplitude).unwrap();
        }
    }

    writer.finalize().unwrap();
    temp_file
}
