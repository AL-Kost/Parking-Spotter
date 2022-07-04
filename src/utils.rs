use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub camera_url: String,
    pub blur_kernel_size: i32,
    pub blur_sigma: f64,
    pub history_frames: i32,
    pub var_threshold: f64,
    pub detect_shadows: bool,
    pub learning_rate: f64,
    pub binary_threshold: f64,
    pub roi_x: i32,
    pub roi_y: i32,
    pub roi_width: i32,
    pub roi_height: i32,
    pub detection_threshold: f64,
    pub alert_url: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub email_from: String,
    pub email_to: String,
    pub email_subject: String,
    pub email_body: String,
    pub sound_file_path: String,
    pub detection_interval_ms: u64
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}