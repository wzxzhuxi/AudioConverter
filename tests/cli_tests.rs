use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_cli_no_args_shows_help() {
    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("audio-converter"));
}

#[test]
fn test_cli_help_flag() {
    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "AudioConverter - 音频文件转数组工具",
        ));
}

#[test]
fn test_cli_missing_input_file() {
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&["-o", temp_output.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("缺少输入文件参数"));
}

#[test]
fn test_cli_missing_output_file() {
    let temp_input = create_test_wav_file(44100, 2, 0.1);

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&["-i", temp_input.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("缺少输出文件参数"));
}

#[test]
fn test_cli_nonexistent_input_file() {
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&[
        "-i",
        "nonexistent.wav",
        "-o",
        temp_output.path().to_str().unwrap(),
    ])
    .assert()
    .failure();
}

#[test]
fn test_cli_successful_conversion() {
    let temp_input = create_test_wav_file(44100, 2, 0.1);
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&[
        "-i",
        temp_input.path().to_str().unwrap(),
        "-o",
        temp_output.path().to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("正在解码音频文件"))
    .stdout(predicate::str::contains("处理完成"));
}

#[test]
fn test_cli_with_format_option() {
    let temp_input = create_test_wav_file(44100, 2, 0.1);
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&[
        "-i",
        temp_input.path().to_str().unwrap(),
        "-o",
        temp_output.path().to_str().unwrap(),
        "-f",
        "i16",
    ])
    .assert()
    .success();

    // 验证输出文件格式
    let content = std::fs::read_to_string(temp_output.path()).unwrap();
    assert!(content.contains("[i16;"));
}

#[test]
fn test_cli_with_sample_rate_option() {
    let temp_input = create_test_wav_file(44100, 2, 0.1);
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&[
        "-i",
        temp_input.path().to_str().unwrap(),
        "-o",
        temp_output.path().to_str().unwrap(),
        "-s",
        "22050",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("转换完成: 采样率=22050Hz"));
}

#[test]
fn test_cli_with_channels_option() {
    let temp_input = create_test_wav_file(44100, 2, 0.1);
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&[
        "-i",
        temp_input.path().to_str().unwrap(),
        "-o",
        temp_output.path().to_str().unwrap(),
        "-c",
        "1",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains(
        "转换完成: 采样率=44100Hz, 声道数=1",
    ));
}

#[test]
fn test_cli_with_gain_option() {
    let temp_input = create_test_wav_file(44100, 2, 0.1);
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&[
        "-i",
        temp_input.path().to_str().unwrap(),
        "-o",
        temp_output.path().to_str().unwrap(),
        "-g",
        "6.0",
    ])
    .assert()
    .success();
}

#[test]
fn test_cli_with_verbose_option() {
    let temp_input = create_test_wav_file(44100, 2, 0.1);
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&[
        "-i",
        temp_input.path().to_str().unwrap(),
        "-o",
        temp_output.path().to_str().unwrap(),
        "-v",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("配置:"));
}

#[test]
fn test_cli_with_config_file() {
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

    let temp_input = create_test_wav_file(44100, 2, 0.1);
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    let output = cmd
        .args(&[
            "-i",
            temp_input.path().to_str().unwrap(),
            "-o",
            temp_output.path().to_str().unwrap(),
            "-C",
            config_file.path().to_str().unwrap(),
            "-v", // 添加详细输出
        ])
        .output()
        .unwrap();

    // 输出调试信息
    println!("命令执行状态: {}", output.status);
    println!("标准输出:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("错误输出:\n{}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "命令应该成功执行");

    // 读取输出文件并显示内容
    let content = std::fs::read_to_string(temp_output.path()).unwrap();
    println!(
        "输出文件内容(前200字符):\n{}",
        &content[..content.len().min(200)]
    );

    // 更详细的断言检查
    assert!(
        content.contains("AUDIO_SAMPLES"),
        "输出文件应包含 AUDIO_SAMPLES 声明"
    );

    // 检查数组类型
    if content.contains("[i16;") {
        println!("✓ 成功：找到 i16 数组类型");
    } else if content.contains("[f32;") {
        panic!("配置文件未生效：输出为 f32 而非 i16");
    } else {
        panic!("未找到预期的数组类型声明");
    }
}

#[test]
fn test_cli_invalid_format() {
    let temp_input = create_test_wav_file(44100, 2, 0.1);
    let temp_output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("audio-converter").unwrap();
    cmd.args(&[
        "-i",
        temp_input.path().to_str().unwrap(),
        "-o",
        temp_output.path().to_str().unwrap(),
        "-f",
        "invalid",
    ])
    .assert()
    .failure()
    .stderr(predicate::str::contains("invalid"));
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
