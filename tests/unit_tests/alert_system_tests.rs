use parking_space_detector::alert_system;
use parking_space_detector::utils;

// Test the initialization of the AlertSystem.
#[test]
fn test_alert_system_init() {
    // Load the configuration file.
    let config = utils::Config::from_file("../../config/settings.toml").unwrap();

    // Initialize the alert system.
    let alert_system_result = alert_system::AlertSystem::new(&config);

    // Check if the initialization was successful.
    assert!(
        alert_system_result.is_ok(),
        "Failed to initialize AlertSystem"
    );
}

// Test the sending of an email alert.
#[test]
fn test_send_email_alert() {
    // Load the configuration file.
    let config = utils::Config::from_file("../../config/settings.toml").unwrap();

    // Initialize the alert system.
    let mut alert_system = alert_system::AlertSystem::new(&config).unwrap();

    // Send an email alert.
    let send_email_result = alert_system.send_email_alert();

    // Check if the email was sent successfully.
    assert!(
        send_email_result.is_ok(),
        "Failed to send email alert"
    );
}

// Test the playing of a sound notification.
#[test]
fn test_play_sound_notification() {
    // Load the configuration file.
    let config = utils::Config::from_file("../../config/settings.toml").unwrap();

    // Initialize the alert system.
    let mut alert_system = alert_system::AlertSystem::new(&config).unwrap();

    // Play a sound notification.
    let play_sound_result = alert_system.play_sound_notification();

    // Check if the sound notification was played successfully.
    assert!(
        play_sound_result.is_ok(),
        "Failed to play sound notification"
    );
}
