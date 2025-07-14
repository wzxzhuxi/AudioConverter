use audio_converter::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use tempfile::NamedTempFile;

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

fn bench_audio_decoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("audio_decoding");

    let durations = vec![0.1, 0.5, 1.0, 5.0];

    for duration in durations {
        let temp_file = create_test_wav_file(44100, 2, duration);

        group.bench_with_input(
            BenchmarkId::new("decode_wav", format!("{}s", duration)),
            &duration,
            |b, _| {
                b.iter(|| {
                    let mut decoder = AudioDecoder::new();
                    decoder
                        .decode_file(black_box(temp_file.path().to_str().unwrap()))
                        .unwrap();
                });
            },
        );
    }

    group.finish();
}

fn bench_audio_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("audio_conversion");

    let sample_sizes = vec![1024, 4096, 16384, 65536];

    for size in sample_sizes {
        let test_samples: Vec<f32> = (0..size)
            .map(|i| (i as f32 / size as f32) * 2.0 - 1.0)
            .collect();
        let config = Config::default();
        let converter = AudioConverter::new(config);

        group.bench_with_input(BenchmarkId::new("convert_basic", size), &size, |b, _| {
            b.iter(|| {
                converter
                    .convert(black_box(&test_samples), 44100, 2)
                    .unwrap();
            });
        });
    }

    group.finish();
}

fn bench_resampling(c: &mut Criterion) {
    let mut group = c.benchmark_group("resampling");

    let test_samples: Vec<f32> = (0..44100)
        .map(|i| (i as f32 / 44100.0) * 2.0 - 1.0)
        .collect();

    let target_rates = vec![22050, 48000, 96000];

    for rate in target_rates {
        let mut config = Config::default();
        config.sample_rate = Some(rate);
        let converter = AudioConverter::new(config);

        group.bench_with_input(BenchmarkId::new("resample_to", rate), &rate, |b, _| {
            b.iter(|| {
                converter
                    .convert(black_box(&test_samples), 44100, 2)
                    .unwrap();
            });
        });
    }

    group.finish();
}

fn bench_channel_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("channel_conversion");

    let stereo_samples: Vec<f32> = (0..88200)
        .map(|i| (i as f32 / 44100.0) * 2.0 - 1.0)
        .collect();
    let mono_samples: Vec<f32> = (0..44100)
        .map(|i| (i as f32 / 44100.0) * 2.0 - 1.0)
        .collect();

    // 立体声转单声道
    let mut config = Config::default();
    config.channels = Some(1);
    let converter = AudioConverter::new(config);

    group.bench_function("stereo_to_mono", |b| {
        b.iter(|| {
            converter
                .convert(black_box(&stereo_samples), 44100, 2)
                .unwrap();
        });
    });

    // 单声道转立体声
    let mut config = Config::default();
    config.channels = Some(2);
    let converter = AudioConverter::new(config);

    group.bench_function("mono_to_stereo", |b| {
        b.iter(|| {
            converter
                .convert(black_box(&mono_samples), 44100, 1)
                .unwrap();
        });
    });

    group.finish();
}

fn bench_output_formats(c: &mut Criterion) {
    let mut group = c.benchmark_group("output_formats");

    let test_samples: Vec<f32> = (0..44100)
        .map(|i| (i as f32 / 44100.0) * 2.0 - 1.0)
        .collect();

    let formats = vec![
        OutputFormat::F32,
        OutputFormat::F64,
        OutputFormat::I16,
        OutputFormat::I32,
    ];

    for format in formats {
        let mut config = Config::default();
        config.output_format = format.clone();
        let converter = AudioConverter::new(config);

        group.bench_with_input(
            BenchmarkId::new("convert_to", format!("{:?}", format)),
            &format,
            |b, _| {
                b.iter(|| {
                    converter
                        .convert(black_box(&test_samples), 44100, 2)
                        .unwrap();
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_audio_decoding,
    bench_audio_conversion,
    bench_resampling,
    bench_channel_conversion,
    bench_output_formats
);
criterion_main!(benches);
