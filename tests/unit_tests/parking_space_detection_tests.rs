use parking_space_detector::{
    camera, video_processing, parking_space_detection, utils
};
use opencv::core;

// Test the detection of free parking spaces.
#[test]
fn test_detect_free_space() {
    // Load the configuration file.
    let config = utils::Config::from_file("../../config/settings.toml").unwrap();

    // Initialize the camera.
    let mut camera = camera::Camera::new(&config).unwrap();

    // Capture a frame from the camera.
    let frame = camera.capture_frame().unwrap();

    // Preprocess the frame.
    let preprocessed_frame = video_processing::preprocess_frame(&frame, &config).unwrap();

    // Initialize the parking space detection module.
    let mut parking_space_detector = parking_space_detection::ParkingSpaceDetector::new(&config);

    // Detect if there is a free parking space in the frame.
    let detect_free_space_result = parking_space_detector.detect_free_space(&preprocessed_frame);

    // Check if the detection was successful.
    assert!(
        detect_free_space_result.is_ok(),
        "Failed to detect free parking space"
    );

    // Get the detection result (true if a free space is detected, false otherwise).
    let has_free_space = detect_free_space_result.unwrap();

    // Print the detection result for informational purposes.
    println!("Free parking space detected: {}", has_free_space);
}
