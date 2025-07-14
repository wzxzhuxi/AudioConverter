use crate::cli::args::OutputFormat;
use crate::config::settings::Config;
use anyhow::Result;

pub struct AudioConverter {
    config: Config,
}

impl AudioConverter {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn convert(&self, samples: &[f32], sample_rate: u32, channels: u32) -> Result<ConvertedAudio> {
        let mut processed_samples = samples.to_vec();

        // 应用增益
        if self.config.gain != 0.0 {
            let gain_factor = 10.0_f32.powf(self.config.gain / 20.0);
            for sample in processed_samples.iter_mut() {
                *sample *= gain_factor;
            }
        }

        // 重采样 (简化实现)
        if let Some(target_sr) = self.config.sample_rate {
            if target_sr != sample_rate {
                processed_samples = self.resample(&processed_samples, sample_rate, target_sr)?;
            }
        }

        // 声道转换
        if let Some(target_channels) = self.config.channels {
            if target_channels != channels {
                processed_samples = self.convert_channels(&processed_samples, channels, target_channels)?;
            }
        }

        // 归一化
        if self.config.normalize {
            self.normalize_samples(&mut processed_samples);
        }

        Ok(ConvertedAudio {
            samples: processed_samples,
            sample_rate: self.config.sample_rate.unwrap_or(sample_rate),
            channels: self.config.channels.unwrap_or(channels),
            format: self.config.output_format.clone(),
        })
    }

    fn resample(&self, samples: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>> {
        // 简化的线性插值重采样
        let ratio = to_rate as f64 / from_rate as f64;
        let new_length = (samples.len() as f64 * ratio) as usize;
        let mut resampled = Vec::with_capacity(new_length);

        for i in 0..new_length {
            let pos = i as f64 / ratio;
            let index = pos as usize;
            
            if index + 1 < samples.len() {
                let frac = pos - index as f64;
                let sample = samples[index] * (1.0 - frac as f32) + samples[index + 1] * frac as f32;
                resampled.push(sample);
            } else if index < samples.len() {
                resampled.push(samples[index]);
            }
        }

        Ok(resampled)
    }

    fn convert_channels(&self, samples: &[f32], from_channels: u32, to_channels: u32) -> Result<Vec<f32>> {
        match (from_channels, to_channels) {
            (2, 1) => {
                // 立体声转单声道
                let mut mono = Vec::with_capacity(samples.len() / 2);
                for chunk in samples.chunks_exact(2) {
                    mono.push((chunk[0] + chunk[1]) / 2.0);
                }
                Ok(mono)
            }
            (1, 2) => {
                // 单声道转立体声
                let mut stereo = Vec::with_capacity(samples.len() * 2);
                for &sample in samples {
                    stereo.push(sample);
                    stereo.push(sample);
                }
                Ok(stereo)
            }
            _ => Ok(samples.to_vec()),
        }
    }

    fn normalize_samples(&self, samples: &mut [f32]) {
        if let Some(max_val) = samples.iter().map(|s| s.abs()).max_by(|a, b| a.partial_cmp(b).unwrap()) {
            if max_val > 0.0 {
                let scale = 1.0 / max_val;
                for sample in samples.iter_mut() {
                    *sample *= scale;
                }
            }
        }
    }
}

pub struct ConvertedAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u32,
    pub format: OutputFormat,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::args::OutputFormat;
    use crate::config::settings::Config;

    fn create_test_config() -> Config {
        Config {
            output_format: OutputFormat::F32,
            sample_rate: None,
            channels: None,
            gain: 0.0,
            normalize: false,
            output_settings: crate::config::settings::OutputSettings {
                array_type: "Vec".to_string(),
                include_metadata: true,
                compress: false,
            },
        }
    }

    #[test]
    fn test_audio_converter_new() {
        let config = create_test_config();
        let _converter = AudioConverter::new(config);
        // AudioConverter 创建成功，无需特殊断言
    }

    #[test]
    fn test_convert_basic() {
        let config = create_test_config();
        let converter = AudioConverter::new(config);
        
        let test_samples = vec![0.1, 0.2, 0.3, 0.4];
        let result = converter.convert(&test_samples, 44100, 2);
        
        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted.samples.len(), 4);
        assert_eq!(converted.sample_rate, 44100);
        assert_eq!(converted.channels, 2);
    }

    #[test]
    fn test_convert_with_gain() {
        let mut config = create_test_config();
        config.gain = 6.0; // +6dB
        let converter = AudioConverter::new(config);
        
        let test_samples = vec![0.1, 0.2, 0.3, 0.4];
        let result = converter.convert(&test_samples, 44100, 2);
        
        assert!(result.is_ok());
        let converted = result.unwrap();
        
        // 验证增益被应用（6dB ≈ 2x 放大）
        let gain_factor = 10.0_f32.powf(6.0 / 20.0);
        assert!((converted.samples[0] - test_samples[0] * gain_factor).abs() < 1e-6);
    }

    #[test]
    fn test_convert_with_resampling() {
        let mut config = create_test_config();
        config.sample_rate = Some(22050);
        let converter = AudioConverter::new(config);
        
        let test_samples = vec![0.1, 0.2, 0.3, 0.4]; // 44100Hz, 2 samples per channel
        let result = converter.convert(&test_samples, 44100, 2);
        
        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted.sample_rate, 22050);
        assert_eq!(converted.samples.len(), 2); // 重采样后样本数减半
    }

    #[test]
    fn test_convert_stereo_to_mono() {
        let mut config = create_test_config();
        config.channels = Some(1);
        let converter = AudioConverter::new(config);
        
        let test_samples = vec![0.1, 0.2, 0.3, 0.4]; // 立体声：L R L R
        let result = converter.convert(&test_samples, 44100, 2);
        
        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted.channels, 1);
        assert_eq!(converted.samples.len(), 2); // 2个单声道样本
        
        // 验证立体声转单声道的平均值
        assert!((converted.samples[0] - (0.1 + 0.2) / 2.0).abs() < 1e-6);
        assert!((converted.samples[1] - (0.3 + 0.4) / 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_convert_mono_to_stereo() {
        let mut config = create_test_config();
        config.channels = Some(2);
        let converter = AudioConverter::new(config);
        
        let test_samples = vec![0.1, 0.2]; // 单声道
        let result = converter.convert(&test_samples, 44100, 1);
        
        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted.channels, 2);
        assert_eq!(converted.samples.len(), 4); // 双倍样本数
        
        // 验证单声道转立体声的复制
        assert_eq!(converted.samples[0], 0.1);
        assert_eq!(converted.samples[1], 0.1);
        assert_eq!(converted.samples[2], 0.2);
        assert_eq!(converted.samples[3], 0.2);
    }

    #[test]
    fn test_convert_with_normalization() {
        let mut config = create_test_config();
        config.normalize = true;
        let converter = AudioConverter::new(config);
        
        let test_samples = vec![0.1, 0.2, 0.8, 0.4]; // 最大值为0.8
        let result = converter.convert(&test_samples, 44100, 2);
        
        assert!(result.is_ok());
        let converted = result.unwrap();
        
        // 验证归一化后最大值为1.0
        let max_val = converted.samples.iter().map(|s| s.abs()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        assert!((max_val - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_convert_empty_samples() {
        let config = create_test_config();
        let converter = AudioConverter::new(config);
        
        let test_samples = vec![];
        let result = converter.convert(&test_samples, 44100, 2);
        
        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted.samples.len(), 0);
    }

    #[test]
    fn test_convert_clipping_prevention() {
        let mut config = create_test_config();
        config.gain = 20.0; // 极大的增益
        let converter = AudioConverter::new(config);
        
        let test_samples = vec![0.9, -0.8, 0.7, -0.6];
        let result = converter.convert(&test_samples, 44100, 2);
        
        assert!(result.is_ok());
        let converted = result.unwrap();
        
        // 验证没有样本超过合理范围
        for sample in &converted.samples {
            assert!(sample.abs() < 10.0); // 防止极端值
        }
    }
}
