use crate::audio::converter::ConvertedAudio;
use crate::cli::args::OutputFormat;
use anyhow::Result;
use serde_json::json;

pub struct ArrayWriter;

impl ArrayWriter {
    pub fn write_to_file(audio: &ConvertedAudio, output_path: &str) -> Result<()> {
        let output = match audio.format {
            OutputFormat::F32 => Self::create_f32_output(audio),
            OutputFormat::F64 => Self::create_f64_output(audio),
            OutputFormat::I16 => Self::create_i16_output(audio),
            OutputFormat::I32 => Self::create_i32_output(audio),
        };

        std::fs::write(output_path, output)?;
        println!("数组已写入: {}", output_path);
        Ok(())
    }

    fn create_f32_output(audio: &ConvertedAudio) -> String {
        let metadata = json!({
            "sample_rate": audio.sample_rate,
            "channels": audio.channels,
            "length": audio.samples.len(),
            "format": "f32"
        });

        format!(
            "// 音频元数据: {}\n// 样本数组 (f32 格式)\nconst AUDIO_SAMPLES: [f32; {}] = [\n{}\n];",
            metadata,
            audio.samples.len(),
            audio.samples
                .iter()
                .map(|s| format!("    {:.6}", s))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }

    fn create_f64_output(audio: &ConvertedAudio) -> String {
        let samples_f64: Vec<f64> = audio.samples.iter().map(|&s| s as f64).collect();
        let metadata = json!({
            "sample_rate": audio.sample_rate,
            "channels": audio.channels,
            "length": samples_f64.len(),
            "format": "f64"
        });

        format!(
            "// 音频元数据: {}\n// 样本数组 (f64 格式)\nconst AUDIO_SAMPLES: [f64; {}] = [\n{}\n];",
            metadata,
            samples_f64.len(),
            samples_f64
                .iter()
                .map(|s| format!("    {:.6}", s))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }

    fn create_i16_output(audio: &ConvertedAudio) -> String {
        let samples_i16: Vec<i16> = audio.samples
            .iter()
            .map(|&s| (s.clamp(-1.0, 1.0) * 32767.0) as i16)
            .collect();
        
        let metadata = json!({
            "sample_rate": audio.sample_rate,
            "channels": audio.channels,
            "length": samples_i16.len(),
            "format": "i16"
        });

        format!(
            "// 音频元数据: {}\n// 样本数组 (i16 格式)\nconst AUDIO_SAMPLES: [i16; {}] = [\n{}\n];",
            metadata,
            samples_i16.len(),
            samples_i16
                .iter()
                .map(|s| format!("    {}", s))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }

    fn create_i32_output(audio: &ConvertedAudio) -> String {
        let samples_i32: Vec<i32> = audio.samples
            .iter()
            .map(|&s| (s.clamp(-1.0, 1.0) * 2147483647.0) as i32)
            .collect();
        
        let metadata = json!({
            "sample_rate": audio.sample_rate,
            "channels": audio.channels,
            "length": samples_i32.len(),
            "format": "i32"
        });

        format!(
            "// 音频元数据: {}\n// 样本数组 (i32 格式)\nconst AUDIO_SAMPLES: [i32; {}] = [\n{}\n];",
            metadata,
            samples_i32.len(),
            samples_i32
                .iter()
                .map(|s| format!("    {}", s))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }
}

