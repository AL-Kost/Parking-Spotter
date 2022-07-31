use parking_space_detector::{
    camera, video_processing, parking_space_detection, alert_system, utils,
};
use opencv::core;
use std::sync::Arc;
use tokio::sync::Mutex;

// Test the integration of the main components of the Parking Spotter application.
#[tokio::test]
async fn test_integration() {
    // Load the configuration file.
    let config = utils::Config::from_file("../config/settings.toml").unwrap();

    // Initialize the camera.
    let mut camera = camera::Camera::new(&config).unwrap();

    // Initialize the parking space detection module.
    let mut parking_space_detector = parking_space_detection::ParkingSpaceDetector::new(&config);

    // Wrap the alert system in an Arc<Mutex<>> to share it safely across threads.
    let alert_system = Arc::new(Mutex::new(
        alert_system::AlertSystem::new(&config).unwrap()
    ));

    // Capture a frame from the camera.
    let frame = camera.capture_frame().unwrap();

    // Check if the frame is not empty.
    assert!(!frame.empty().unwrap());

    // Pre-process the frame.
    let processed_frame = video_processing::preprocess_frame(&frame, &config).unwrap();

    // Check if the processed frame is not empty.
    assert!(!processed_frame.empty().unwrap());

    // Detect free parking spaces.
    let has_free_space = parking_space_detector.detect_free_space(&processed_frame).unwrap();

    // If a free parking space is detected, send an alert.
    if has_free_space {
        let alert_system_clone = Arc::clone(&alert_system);
        let send_alert = async move {
            let mut alert_system_locked = alert_system_clone.lock().await;
            alert_system_locked.send_alert().await.unwrap();
        };
        tokio::spawn(send_alert);
    }

    // Verify that the alert system is in the expected state.
    {
        let alert_system_locked = alert_system.lock().await;
        let expected_state = if has_free_space {
            alert_system::AlertSystemState::AlertSent
        } else {
            alert_system::AlertSystemState::NoAlert
        };

        assert_eq!(alert_system_locked.get_state(), expected_state);
    }
}
