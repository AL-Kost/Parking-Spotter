# Parking-Spotter
Parking Spotter is a Rust application that helps you detect free parking spaces in your yard using a camera. The application processes the video feed in real-time and sends an email alert and plays a sound notification when a free parking space is detected. You can use any camera that suits your needs, such as the Logitech C920.

## Table of Contents

- [Getting Started](#getting-started)
    - [Dependencies and Prerequisites](#dependencies-and-prerequisites)
    - [Running the Project](#running-the-project)
- [Parking Space Detection](#parking-space-detection)
    - [Features](#features)
- [Contributing](#contributing)
- [License](#license)

## Getting started
Follow these instructions to build and run the Parking Spotter application on your local machine.

### Dependencies and Prerequisites

- [Rust](https://www.rust-lang.org/): 1.57 or newer
- [OpenCV](https://opencv.org/): 4.5 or newer
- [opencv-rust](https://github.com/twistedfall/opencv-rust) (Rust crate): 0.54 or newer
- Camera like Logitech C920

### Running the Project

1. Ensure you have Rust and OpenCV installed on your system.
2. Clone the repository.
3. Update the `Cargo.toml` file with the appropriate dependencies.
4. Run `cargo build` to build the project.
5. Run `cargo run --release` to run the project.

## Parking Space Detection
The application detects free parking spaces using a series of image processing techniques. First, it preprocesses the video frames by converting them to grayscale and applying Gaussian blur to reduce noise. Then, it uses background subtraction to generate a foreground mask and applies a threshold to create a binary image. The percentage of white pixels in the region of interest (ROI) is calculated and compared to a detection threshold to determine if a free parking space is available.

### Features
- Real-time video feed monitoring
- Parking space detection
- Email alerts
- Sound notifications

## Contributing
I welcome contributions to improve the Parking Spotter application. If you have ideas or suggestions, please feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for more information.