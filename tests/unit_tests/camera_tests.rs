use parking_space_detector::{camera, utils};
use opencv::core;

// Test the initialization of the camera.
#[test]
fn test_camera_initialization() {
    // Load the configuration file.
    let config = utils::Config::from_file("config/settings.toml").unwrap();

    // Initialize the camera.
    let camera_result = camera::Camera::new(&config);

    // Check if the camera was initialized successfully.
    assert!(camera_result.is_ok(), "Failed to initialize the camera");
}

// Test capturing a frame from the camera.
#[test]
fn test_camera_capture_frame() {
    // Load the configuration file.
    let config = utils::Config::from_file("config/settings.toml").unwrap();

    // Initialize the camera.
    let mut camera = camera::Camera::new(&config).unwrap();

    // Capture a frame from the camera.
    let frame_result = camera.capture_frame();

    // Check if the frame was captured successfully.
    assert!(
        frame_result.is_ok(),
        "Failed to capture a frame from the camera"
    );

    // Get the captured frame.
    let frame = frame_result.unwrap();

    // Check if the captured frame is not empty.
    assert!(!frame.empty().unwrap(), "Captured frame is empty");
}

// Test the camera's properties.
#[test]
fn test_camera_properties() {
    // Load the configuration file.
    let config = utils::Config::from_file("config/settings.toml").unwrap();

    // Initialize the camera.
    let mut camera = camera::Camera::new(&config).unwrap();

    // Get the camera's properties.
    let width = camera.capture.get(opencv::videoio::CAP_PROP_FRAME_WIDTH).unwrap();
    let height = camera.capture.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT).unwrap();
    let fps = camera.capture.get(opencv::videoio::CAP_PROP_FPS).unwrap();

    // Check if the camera's properties are set correctly.
    assert_eq!(width, config.camera_width as f64);
    assert_eq!(height, config.camera_height as f64);
    assert_eq!(fps, config.camera_fps as f64);
}
