use opencv::{
    core::{self, Rect},
    imgproc,
    video,
};
use std::error::Error;

pub struct ParkingSpaceDetector {
    background_subtractor: video::BackgroundSubtractorMOG2,
    detection_threshold: f64
}

impl ParkingSpaceDetector {
    pub fn new(config: &crate::utils::Config) -> Self {
        let background_subtractor = video::BackgroundSubtractorMOG2::create(
            config.history_frames,
            config.var_threshold,
            config.detect_shadows
        )
            .unwrap();

        let detection_threshold = config.detection_threshold;

        Self {
            background_subtractor,
            detection_threshold
        }
    }

    pub fn detect_free_space(&mut self, frame: &core::Mat) -> Result<bool, Box<dyn Error>> {
        let mut fg_mask = core::Mat::default();
        self.background_subtractor.apply(frame, &mut fg_mask, config.learning_rate)?;

        // Threshold the foreground mask to create a binary image
        imgproc::threshold(&fg_mask, &mut fg_mask, config.binary_threshold, 255.0, imgproc::THRESH_BINARY)?;

        // Define a region of interest (ROI) for the parking space
        let roi = Rect::new(config.roi_x, config.roi_y, config.roi_width, config.roi_height);
        let parking_space_roi = fg_mask.roi(roi)?;

        // Calculate the percentage of white pixels in the ROI
        let white_pixel_count = parking_space_roi.count_non_zero()?;
        let total_pixels = roi.width * roi.height;
        let white_pixel_percentage = (white_pixel_count as f64) / (total_pixels as f64);

        // Compare the percentage of white pixels to the detection threshold
        Ok(white_pixel_percentage < self.detection_threshold)
    }
}
