mod camera;
mod video_processing;
mod parking_space_detection;
mod alert_system;
mod utils;

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration settings from the config file
    let config = utils::Config::from_file("../config/settings.toml")?;

    // Initialize the camera
    let mut camera = camera::Camera::new(&config)?;

    // Initialize the parking space detection module
    let mut parking_space_detector = parking_space_detection::ParkingSpaceDetector::new(&config);

    // Initialize the alert system
    let alert_system = alert_system::AlertSystem::new(&config)?;

    // Main detection loop
    loop {
        // Capture a frame from the camera
        let frame = camera.capture_frame()?;

        // Pre-process the frame
        let processed_frame = video_processing::preprocess_frame(&frame, &config)?;

        // Detect free parking spaces
        let has_free_space = parking_space_detector.detect_free_space(&processed_frame)?;

        // Send an alert if a free space is found
        if has_free_space {
            alert_system.send_alert().await?;
        }

        // Sleep for a specified interval before processing the next frame
        let interval = Duration::from_millis(config.detection_interval_ms);
        sleep(interval).await;
    }
}
