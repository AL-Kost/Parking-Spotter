use parking_space_detector::{camera, video_processing, utils};
use opencv::core;

// Test the preprocessing of a frame.
#[test]
fn test_preprocess_frame() {
    // Load the configuration file.
    let config = utils::Config::from_file("config/settings.toml").unwrap();

    // Initialize the camera.
    let mut camera = camera::Camera::new(&config).unwrap();

    // Capture a frame from the camera.
    let frame = camera.capture_frame().unwrap();

    // Preprocess the frame.
    let preprocessed_frame_result = video_processing::preprocess_frame(&frame, &config);

    // Check if the frame was preprocessed successfully.
    assert!(
        preprocessed_frame_result.is_ok(),
        "Failed to preprocess the frame"
    );

    // Get the preprocessed frame.
    let preprocessed_frame = preprocessed_frame_result.unwrap();

    // Check if the preprocessed frame is not empty.
    assert!(
        !preprocessed_frame.empty().unwrap(),
        "Preprocessed frame is empty"
    );

    // Check if the preprocessed frame has the same dimensions as the original frame.
    assert_eq!(
        preprocessed_frame.size().unwrap(),
        frame.size().unwrap(),
        "Preprocessed frame dimensions do not match the original frame dimensions"
    );

    // Check if the preprocessed frame is a single-channel (grayscale) image.
    assert_eq!(
        preprocessed_frame.channels().unwrap(),
        1,
        "Preprocessed frame is not a single-channel (grayscale) image"
    );
}
