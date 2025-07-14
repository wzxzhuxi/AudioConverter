use audio_converter::*;
use rstest::*;
use test_case::test_case;

#[rstest]
#[case(22050, 1, 0.1)]
#[case(44100, 2, 0.5)]
#[case(48000, 1, 1.0)]
#[case(96000, 2, 0.2)]
fn test_audio_decoding_properties(
    #[case] sample_rate: u32,
    #[case] channels: u16,
    #[case] duration: f32,
) {
    let temp_file = create_test_wav_file(sample_rate, channels, duration);
    let mut decoder = AudioDecoder::new();

    let result = decoder.decode_file(temp_file.path().to_str().unwrap());

    assert!(result.is_ok());
    assert_eq!(decoder.get_sample_rate(), sample_rate);
    assert_eq!(decoder.get_channels(), channels as u32);

    let expected_samples = (sample_rate as f32 * duration * channels as f32) as usize;
    let actual_samples = decoder.get_samples().len();

    // 允许小量误差
    assert!(
        (actual_samples as f32 - expected_samples as f32).abs() < expected_samples as f32 * 0.1
    );
}

#[test_case(OutputFormat::F32)]
#[test_case(OutputFormat::F64)]
#[test_case(OutputFormat::I16)]
#[test_case(OutputFormat::I32)]
fn test_output_format_conversion(format: OutputFormat) {
    let test_samples = vec![0.0, 0.5, -0.5, 1.0, -1.0];
    let mut config = Config::default();
    config.output_format = format;

    let converter = AudioConverter::new(config);
    let result = converter.convert(&test_samples, 44100, 1);

    assert!(result.is_ok());
    let converted = result.unwrap();
    assert_eq!(converted.samples.len(), test_samples.len());
    assert_eq!(converted.format, format);
}

#[rstest]
#[case(-12.0)]
#[case(-6.0)]
#[case(0.0)]
#[case(6.0)]
#[case(12.0)]
fn test_gain_application(#[case] gain_db: f32) {
    let test_samples = vec![0.1, 0.2, 0.3, 0.4];
    let mut config = Config::default();
    config.gain = gain_db;

    let converter = AudioConverter::new(config);
    let result = converter.convert(&test_samples, 44100, 2);

    assert!(result.is_ok());
    let converted = result.unwrap();

    if gain_db != 0.0 {
        let gain_factor = 10.0_f32.powf(gain_db / 20.0);
        let expected_first = test_samples[0] * gain_factor;
        assert!((converted.samples[0] - expected_first).abs() < 1e-6);
    }
}

#[rstest]
#[case(22050, 44100)]
#[case(44100, 22050)]
#[case(44100, 48000)]
#[case(48000, 44100)]
fn test_resampling_properties(#[case] from_rate: u32, #[case] to_rate: u32) {
    let samples_per_second = 100;
    let test_samples: Vec<f32> = (0..samples_per_second)
        .map(|i| (i as f32 / samples_per_second as f32) * 2.0 - 1.0)
        .collect();

    let mut config = Config::default();
    config.sample_rate = Some(to_rate);

    let converter = AudioConverter::new(config);
    let result = converter.convert(&test_samples, from_rate, 1);

    assert!(result.is_ok());
    let converted = result.unwrap();
    assert_eq!(converted.sample_rate, to_rate);

    // 验证重采样后的长度大致正确
    let expected_length = (test_samples.len() as f64 * to_rate as f64 / from_rate as f64) as usize;
    let actual_length = converted.samples.len();
    assert!((actual_length as f64 - expected_length as f64).abs() < expected_length as f64 * 0.1);
}

#[rstest]
#[case(1, 2)]
#[case(2, 1)]
fn test_channel_conversion_properties(#[case] from_channels: u32, #[case] to_channels: u32) {
    let test_samples = if from_channels == 1 {
        vec![0.1, 0.2, 0.3, 0.4]
    } else {
        vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]
    };

    let mut config = Config::default();
    config.channels = Some(to_channels);

    let converter = AudioConverter::new(config);
    let result = converter.convert(&test_samples, 44100, from_channels);

    assert!(result.is_ok());
    let converted = result.unwrap();
    assert_eq!(converted.channels, to_channels);

    // 验证样本数量变化
    if from_channels == 1 && to_channels == 2 {
        assert_eq!(converted.samples.len(), test_samples.len() * 2);
    } else if from_channels == 2 && to_channels == 1 {
        assert_eq!(converted.samples.len(), test_samples.len() / 2);
    }
}

fn create_test_wav_file(
    sample_rate: u32,
    channels: u16,
    duration_seconds: f32,
) -> tempfile::NamedTempFile {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
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
