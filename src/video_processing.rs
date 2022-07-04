use opencv::{
    core::{self, Size},
    imgproc,
};
use std::error::Error;

pub fn preprocess_frame(frame: &core::Mat, config: &crate::utils::Config) -> Result<core::Mat, Box<dyn Error>> {
    let mut gray_frame = core::Mat::default();

    // Convert the frame to grayscale
    imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;

    // Apply Gaussian blur to reduce noise
    let mut blurred_frame = core::Mat::default();
    imgproc::gaussian_blur(
        &gray_frame,
        &mut blurred_frame,
        Size::new(config.blur_kernel_size, config.blur_kernel_size),
        config.blur_sigma,
        config.blur_sigma,
        0,
    )?;

    Ok(blurred_frame)
}
