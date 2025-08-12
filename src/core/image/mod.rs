use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};
use std::path::Path;

pub mod processor;
pub mod types;

pub use processor::ImageProcessor;
pub use types::*;

/// Load an image from file path
pub fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
    let img = image::open(&path)
        .with_context(|| format!("Failed to load image from {:?}", path.as_ref()))?;
    Ok(img)
}

/// Save an image to file path with specified format
pub fn save_image<P: AsRef<Path>>(
    image: &DynamicImage,
    path: P,
    format: ImageFormat,
) -> Result<()> {
    image
        .save_with_format(&path, format)
        .with_context(|| format!("Failed to save image to {:?}", path.as_ref()))?;
    Ok(())
}

/// Get image format from file extension
pub fn format_from_extension<P: AsRef<Path>>(path: P) -> Option<ImageFormat> {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .and_then(|ext| match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
            "png" => Some(ImageFormat::Png),
            "gif" => Some(ImageFormat::Gif),
            "webp" => Some(ImageFormat::WebP),
            "tiff" | "tif" => Some(ImageFormat::Tiff),
            "bmp" => Some(ImageFormat::Bmp),
            "ico" => Some(ImageFormat::Ico),
            _ => None,
        })
}

/// Get image dimensions as (width, height)
pub fn get_dimensions(image: &DynamicImage) -> (u32, u32) {
    (image.width(), image.height())
}

/// Calculate new dimensions maintaining aspect ratio
pub fn calculate_aspect_ratio_dimensions(
    original_width: u32,
    original_height: u32,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> (u32, u32) {
    match (target_width, target_height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => {
            let ratio = original_height as f64 / original_width as f64;
            let h = (w as f64 * ratio).round() as u32;
            (w, h)
        }
        (None, Some(h)) => {
            let ratio = original_width as f64 / original_height as f64;
            let w = (h as f64 * ratio).round() as u32;
            (w, h)
        }
        (None, None) => (original_width, original_height),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from_extension() {
        assert_eq!(format_from_extension("test.jpg"), Some(ImageFormat::Jpeg));
        assert_eq!(format_from_extension("test.png"), Some(ImageFormat::Png));
        assert_eq!(format_from_extension("test.unknown"), None);
    }

    #[test]
    fn test_calculate_aspect_ratio_dimensions() {
        // Original: 100x50
        assert_eq!(
            calculate_aspect_ratio_dimensions(100, 50, Some(200), None),
            (200, 100)
        );
        assert_eq!(
            calculate_aspect_ratio_dimensions(100, 50, None, Some(100)),
            (200, 100)
        );
        assert_eq!(
            calculate_aspect_ratio_dimensions(100, 50, Some(200), Some(200)),
            (200, 200)
        );
        assert_eq!(
            calculate_aspect_ratio_dimensions(100, 50, None, None),
            (100, 50)
        );
    }
}

