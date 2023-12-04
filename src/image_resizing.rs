use std::fs;

pub fn resize_image(image_path: &str) {
    // Load the original image
    let img = image
        ::open(image_path)
        .expect(&format!("Failed to open image with path: {}", image_path));

    // Specify the target width and height for resizing
    let target_width = 80;
    let target_height = 100;

    // Resize the image using Lanczos3 filtering (high-quality)
    let resized_img = img.resize(
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3
    );

    // Save the resized image
    resized_img
        .save(image_path.replace("original", "resized"))
        .expect("Failed to save resized image");
}

pub fn resized_image_exists(image_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(image_path.replace("original", "resized")) {
        // The file exists if there are no errors when obtaining metadata
        metadata.is_file()
    } else {
        // An error occurred, indicating that the file doesn't exist or other issues
        false
    }
}
