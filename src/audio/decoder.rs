use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::fs::File;
use anyhow::Result;

pub struct AudioDecoder {
    sample_rate: u32,
    channels: u32,
    samples: Vec<f32>,
}

impl AudioDecoder {
    pub fn new() -> Self {
        Self {
            sample_rate: 0,
            channels: 0,
            samples: Vec::new(),
        }
    }

    pub fn decode_file(&mut self, path: &str) -> Result<()> {
        // 打开文件
        let file = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // 创建格式提示
        let mut hint = Hint::new();
        if let Some(extension) = std::path::Path::new(path).extension() {
            if let Some(ext_str) = extension.to_str() {
                hint.with_extension(ext_str);
            }
        }

        // 探测格式
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)?;

        let mut format = probed.format;

        // 找到音频轨道并提取所需信息
        let (track_id, codec_params) = {
            let track = format
                .tracks()
                .iter()
                .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
                .ok_or_else(|| anyhow::anyhow!("未找到音频轨道"))?;
            
            (track.id, track.codec_params.clone())
        }; // 这里结束了对 format 的不可变借用

        // 创建解码器
        let dec_opts: DecoderOptions = Default::default();
        let mut decoder = symphonia::default::get_codecs()
            .make(&codec_params, &dec_opts)?;

        // 获取音频参数
        self.sample_rate = codec_params.sample_rate.unwrap_or(44100);
        self.channels = codec_params.channels.unwrap().count() as u32;

        // 解码音频数据
        let mut sample_buf = None;

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(_) => break,
            };

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(audio_buf) => {
                    // 转换为样本缓冲区
                    if sample_buf.is_none() {
                        let spec = *audio_buf.spec();
                        let duration = audio_buf.capacity() as u64;
                        sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                    }

                    if let Some(buf) = &mut sample_buf {
                        buf.copy_interleaved_ref(audio_buf);
                        self.samples.extend_from_slice(buf.samples());
                    }
                }
                Err(e) => {
                    println!("解码错误: {}", e);
                    break;
                }
            }
        }

        println!("解码完成: {} 个样本", self.samples.len());
        Ok(())
    }

    pub fn get_samples(&self) -> &[f32] {
        &self.samples
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn get_channels(&self) -> u32 {
        self.channels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_audio_decoder_new() {
        let decoder = AudioDecoder::new();
        assert_eq!(decoder.get_sample_rate(), 0);
        assert_eq!(decoder.get_channels(), 0);
        assert_eq!(decoder.get_samples().len(), 0);
    }

    #[test]
    fn test_decode_nonexistent_file() {
        let mut decoder = AudioDecoder::new();
        let result = decoder.decode_file("nonexistent.wav");
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_empty_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"").unwrap();
        
        let mut decoder = AudioDecoder::new();
        let result = decoder.decode_file(temp_file.path().to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_invalid_audio_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"This is not an audio file").unwrap();
        
        let mut decoder = AudioDecoder::new();
        let result = decoder.decode_file(temp_file.path().to_str().unwrap());
        assert!(result.is_err());
    }

    // 生成测试用WAV文件的辅助函数
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

    #[test]
    fn test_decode_mono_wav() {
        let temp_file = create_test_wav_file(44100, 1, 0.1);
        let mut decoder = AudioDecoder::new();
        
        let result = decoder.decode_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(decoder.get_sample_rate(), 44100);
        assert_eq!(decoder.get_channels(), 1);
        assert!(decoder.get_samples().len() > 0);
    }

    #[test]
    fn test_decode_stereo_wav() {
        let temp_file = create_test_wav_file(48000, 2, 0.1);
        let mut decoder = AudioDecoder::new();
        
        let result = decoder.decode_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(decoder.get_sample_rate(), 48000);
        assert_eq!(decoder.get_channels(), 2);
        assert!(decoder.get_samples().len() > 0);
    }

    #[test]
    fn test_decode_different_sample_rates() {
        let sample_rates = vec![22050, 44100, 48000, 96000];
        
        for sample_rate in sample_rates {
            let temp_file = create_test_wav_file(sample_rate, 2, 0.05);
            let mut decoder = AudioDecoder::new();
            
            let result = decoder.decode_file(temp_file.path().to_str().unwrap());
            assert!(result.is_ok(), "Failed to decode file with sample rate {}", sample_rate);
            assert_eq!(decoder.get_sample_rate(), sample_rate);
        }
    }
}
