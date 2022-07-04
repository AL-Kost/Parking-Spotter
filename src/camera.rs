use opencv::{
    core,
    prelude::*,
    videoio,
};
use std::error::Error;

pub struct Camera {
    capture: videoio::VideoCapture,
}

impl Camera {
    pub fn new(config: &crate::utils::Config) -> Result<Self, Box<dyn Error>> {
        let mut capture = videoio::VideoCapture::new(config.camera_id, videoio::CAP_ANY)?;

        if !videoio::VideoCapture::is_opened(&capture)? {
            return Err(format!("Unable to open camera with ID: {}", config.camera_id).into());
        }

        // Optionally, set camera properties like resolution and frame rate
        capture.set(videoio::CAP_PROP_FRAME_WIDTH, config.camera_width)?;
        capture.set(videoio::CAP_PROP_FRAME_HEIGHT, config.camera_height)?;
        capture.set(videoio::CAP_PROP_FPS, config.camera_fps)?;

        Ok(Self { capture })
    }

    pub fn capture_frame(&mut self) -> Result<core::Mat, Box<dyn Error>> {
        let mut frame = core::Mat::default();
        self.capture.read(&mut frame)?;

        if frame.size()?.width > 0 && frame.size()?.height > 0 {
            Ok(frame)
        } else {
            Err("Failed to capture a valid frame".into())
        }
    }
}
