use serde::{Deserialize, Serialize};
use crate::cli::args::OutputFormat;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub output_format: OutputFormat,
    pub sample_rate: Option<u32>,
    pub channels: Option<u32>,
    pub gain: f32,
    pub normalize: bool,
    pub output_settings: OutputSettings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputSettings {
    pub array_type: String,
    pub include_metadata: bool,
    pub compress: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::F32,
            sample_rate: None,
            channels: None,
            gain: 0.0,
            normalize: false,
            output_settings: OutputSettings {
                array_type: "Vec".to_string(),
                include_metadata: true,
                compress: false,
            },
        }
    }
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn merge_with_args(&mut self, args: &crate::cli::args::Args) {
        // 只有当命令行明确指定格式时才覆盖配置文件设置
        if let Some(format) = args.format {
            self.output_format = format;
        }
        
        if let Some(sr) = args.sample_rate {
            self.sample_rate = Some(sr);
        }
        
        if let Some(ch) = args.channels {
            self.channels = Some(ch);
        }
        
        // 只有当命令行明确指定增益时才覆盖配置文件设置
        if let Some(gain) = args.gain {
            self.gain = gain;
        }
    }
}

