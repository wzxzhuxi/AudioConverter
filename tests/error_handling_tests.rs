use audio_converter::*;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_invalid_audio_file_handling() {
    let mut temp_file = NamedTempFile::new().unwrap();
    std::io::Write::write_all(&mut temp_file, b"This is not an audio file").unwrap();

    let mut decoder = AudioDecoder::new();
    let result = decoder.decode_file(temp_file.path().to_str().unwrap());

    assert!(result.is_err());
}

#[test]
fn test_empty_file_handling() {
    let temp_file = NamedTempFile::new().unwrap();

    let mut decoder = AudioDecoder::new();
    let result = decoder.decode_file(temp_file.path().to_str().unwrap());

    assert!(result.is_err());
}

#[test]
fn test_corrupted_wav_file_handling() {
    let mut temp_file = NamedTempFile::new().unwrap();
    // 写入部分WAV头但不完整
    std::io::Write::write_all(&mut temp_file, b"RIFF").unwrap();

    let mut decoder = AudioDecoder::new();
    let result = decoder.decode_file(temp_file.path().to_str().unwrap());

    assert!(result.is_err());
}

#[test]
fn test_invalid_config_file_handling() {
    let mut temp_file = NamedTempFile::new().unwrap();
    std::io::Write::write_all(&mut temp_file, b"{ invalid json }").unwrap();

    let result = Config::from_file(temp_file.path().to_str().unwrap());

    assert!(result.is_err());
}

#[test]
fn test_missing_config_file_handling() {
    let result = Config::from_file("nonexistent_config.json");

    assert!(result.is_err());
}

#[test]
fn test_write_to_readonly_directory() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let readonly_path = temp_dir.path().join("readonly");
    fs::create_dir(&readonly_path).unwrap();

    // 在某些系统上设置只读权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&readonly_path).unwrap().permissions();
        perms.set_mode(0o444);
        fs::set_permissions(&readonly_path, perms).unwrap();
    }

    let config = Config::default();
    let converter = AudioConverter::new(config);
    let test_samples = vec![0.1, 0.2, 0.3, 0.4];
    let converted = converter.convert(&test_samples, 44100, 2).unwrap();

    let output_path = readonly_path.join("output.rs");
    let result = ArrayWriter::write_to_file(&converted, output_path.to_str().unwrap());

    // 在某些系统上这可能会失败
    #[cfg(unix)]
    assert!(result.is_err());
}

#[test]
fn test_extremely_large_gain_handling() {
    let mut config = Config::default();
    config.gain = 100.0; // 极大的增益

    let converter = AudioConverter::new(config);
    let test_samples = vec![0.1, 0.2, 0.3, 0.4];
    let result = converter.convert(&test_samples, 44100, 2);

    // 应该成功但可能产生很大的值
    assert!(result.is_ok());
    let converted = result.unwrap();

    // 验证没有 NaN 或 Infinity
    for sample in &converted.samples {
        assert!(sample.is_finite());
    }
}

#[test]
fn test_zero_sample_rate_handling() {
    let config = Config::default();
    let converter = AudioConverter::new(config);
    let test_samples = vec![0.1, 0.2, 0.3, 0.4];

    // 零采样率应该被正确处理
    let result = converter.convert(&test_samples, 0, 2);

    // 这可能会失败或产生特殊行为
    if let Ok(converted) = result {
        assert_eq!(converted.sample_rate, 0);
    }
}

#[test]
fn test_zero_channels_handling() {
    let config = Config::default();
    let converter = AudioConverter::new(config);
    let test_samples = vec![0.1, 0.2, 0.3, 0.4];

    let result = converter.convert(&test_samples, 44100, 0);

    // 零声道应该被正确处理
    if let Ok(converted) = result {
        assert_eq!(converted.channels, 0);
    }
}

#[test]
fn test_negative_sample_rate_resampling() {
    let mut config = Config::default();
    config.sample_rate = Some(0); // 无效的目标采样率

    let converter = AudioConverter::new(config);
    let test_samples = vec![0.1, 0.2, 0.3, 0.4];
    let result = converter.convert(&test_samples, 44100, 2);

    // 应该处理无效的采样率
    if let Ok(converted) = result {
        assert_eq!(converted.sample_rate, 0);
    }
}
