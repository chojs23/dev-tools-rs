use anyhow::{Context, Result};
use image::{codecs::jpeg::JpegEncoder, imageops, DynamicImage, ImageFormat};
use std::io::Cursor;
use std::path::Path;

use super::types::{CompressionOptions, CropRect, ImageFormatType, ImageInfo, ResizeOptions};

use super::{calculate_aspect_ratio_dimensions, get_dimensions, load_image, save_image};

pub struct ImageProcessor {
    current_image: Option<DynamicImage>,
    original_image: Option<DynamicImage>,
    image_info: Option<ImageInfo>,
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageProcessor {
    pub fn new() -> Self {
        Self {
            current_image: None,
            original_image: None,
            image_info: None,
        }
    }

    /// Load an image from file path
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let image = load_image(&path)?;

        // Get file size
        let file_size = std::fs::metadata(&path).map(|m| m.len()).ok();

        // Get format from file extension
        let format =
            super::format_from_extension(&path).and_then(ImageFormatType::from_image_format);

        let mut info = ImageInfo::from_image(&image, file_size);
        info.format = format;

        self.original_image = Some(image.clone());
        self.current_image = Some(image);
        self.image_info = Some(info);

        Ok(())
    }

    /// Get current image reference
    pub fn current_image(&self) -> Option<&DynamicImage> {
        self.current_image.as_ref()
    }

    /// Get original image reference
    pub fn original_image(&self) -> Option<&DynamicImage> {
        self.original_image.as_ref()
    }

    /// Get image information
    pub fn image_info(&self) -> Option<&ImageInfo> {
        self.image_info.as_ref()
    }

    /// Reset current image to original
    pub fn reset_to_original(&mut self) {
        if let Some(original) = &self.original_image {
            self.current_image = Some(original.clone());
        }
    }

    /// Clear all images
    pub fn clear(&mut self) {
        self.current_image = None;
        self.original_image = None;
        self.image_info = None;
    }

    /// Load an image directly for testing purposes
    pub fn load_test_image(&mut self, image: DynamicImage) {
        self.current_image = Some(image.clone());
        self.original_image = Some(image);
    }

    /// Resize the current image
    pub fn resize(&mut self, options: &ResizeOptions) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let (original_width, original_height) = get_dimensions(image);

        let (new_width, new_height) = if options.maintain_aspect_ratio {
            calculate_aspect_ratio_dimensions(
                original_width,
                original_height,
                options.width,
                options.height,
            )
        } else {
            match (options.width, options.height) {
                (Some(w), Some(h)) => (w, h),
                (Some(w), None) => (w, original_height),
                (None, Some(h)) => (original_width, h),
                (None, None) => return Ok(()), // No resize needed
            }
        };

        if new_width == 0 || new_height == 0 {
            return Err(anyhow::anyhow!(
                "Invalid dimensions: {}x{}",
                new_width,
                new_height
            ));
        }

        let filter = options.filter.to_image_filter();
        let resized = image.resize(new_width, new_height, filter);

        self.current_image = Some(resized);
        Ok(())
    }

    /// Crop the current image
    pub fn crop(&mut self, crop_rect: &CropRect) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        if !crop_rect.is_valid_for_image(image) {
            return Err(anyhow::anyhow!(
                "Invalid crop rectangle: {}x{} at ({}, {}) for image {}x{}",
                crop_rect.width,
                crop_rect.height,
                crop_rect.x,
                crop_rect.y,
                image.width(),
                image.height()
            ));
        }

        let cropped = image.crop_imm(crop_rect.x, crop_rect.y, crop_rect.width, crop_rect.height);

        self.current_image = Some(cropped);
        Ok(())
    }

    /// Rotate the current image by 90 degrees clockwise
    pub fn rotate_90(&mut self) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let rotated = image.rotate90();
        self.current_image = Some(rotated);
        Ok(())
    }

    /// Rotate the current image by 180 degrees
    pub fn rotate_180(&mut self) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let rotated = image.rotate180();
        self.current_image = Some(rotated);
        Ok(())
    }

    /// Rotate the current image by 270 degrees clockwise (90 counter-clockwise)
    pub fn rotate_270(&mut self) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let rotated = image.rotate270();
        self.current_image = Some(rotated);
        Ok(())
    }

    /// Flip the current image horizontally
    pub fn flip_horizontal(&mut self) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let flipped = image.fliph();
        self.current_image = Some(flipped);
        Ok(())
    }

    /// Flip the current image vertically
    pub fn flip_vertical(&mut self) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let flipped = image.flipv();
        self.current_image = Some(flipped);
        Ok(())
    }

    /// Compress and save the current image
    pub fn compress_and_save<P: AsRef<Path>>(
        &self,
        path: P,
        options: &CompressionOptions,
    ) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let format = options.format.to_image_format();

        match format {
            ImageFormat::Jpeg => {
                // Use custom JPEG encoder for quality control
                let mut output = Vec::new();
                let mut encoder = JpegEncoder::new_with_quality(&mut output, options.quality);
                encoder
                    .encode(
                        image.as_bytes(),
                        image.width(),
                        image.height(),
                        image.color().into(),
                    )
                    .context("Failed to encode JPEG")?;

                std::fs::write(&path, output).with_context(|| {
                    format!("Failed to write compressed image to {:?}", path.as_ref())
                })?;
            }
            _ => {
                // For other formats, use standard save method
                save_image(image, &path, format)?;
            }
        }

        Ok(())
    }

    /// Save the current image with specified format
    pub fn save_as<P: AsRef<Path>>(&self, path: P, format: ImageFormatType) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        save_image(image, path, format.to_image_format())
    }

    /// Get the estimated file size for different compression options
    pub fn estimate_compressed_size(&self, options: &CompressionOptions) -> Result<usize> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let format = options.format.to_image_format();

        match format {
            ImageFormat::Jpeg => {
                let mut output = Vec::new();
                let mut encoder = JpegEncoder::new_with_quality(&mut output, options.quality);
                encoder
                    .encode(
                        image.as_bytes(),
                        image.width(),
                        image.height(),
                        image.color().into(),
                    )
                    .context("Failed to encode JPEG for size estimation")?;
                Ok(output.len())
            }
            _ => {
                // For other formats, encode to memory buffer
                let mut buffer = Cursor::new(Vec::new());
                image
                    .write_to(&mut buffer, format)
                    .context("Failed to encode image for size estimation")?;
                Ok(buffer.into_inner().len())
            }
        }
    }

    /// Apply brightness adjustment
    pub fn adjust_brightness(&mut self, value: i32) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let rgba_image = image.to_rgba8();
        let adjusted = imageops::brighten(&rgba_image, value);
        self.current_image = Some(DynamicImage::ImageRgba8(adjusted));
        Ok(())
    }

    /// Apply contrast adjustment
    pub fn adjust_contrast(&mut self, contrast: f32) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let rgba_image = image.to_rgba8();
        let adjusted = imageops::contrast(&rgba_image, contrast);
        self.current_image = Some(DynamicImage::ImageRgba8(adjusted));
        Ok(())
    }

    /// Convert to grayscale
    pub fn to_grayscale(&mut self) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let grayscale = image.grayscale();
        self.current_image = Some(grayscale);
        Ok(())
    }

    /// Apply blur effect
    pub fn blur(&mut self, sigma: f32) -> Result<()> {
        let image = self.current_image.as_ref().context("No image loaded")?;

        let blurred = image.blur(sigma);
        self.current_image = Some(blurred);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::image::FilterType;

    use super::*;
    use image::{ImageBuffer, Rgb};

    fn create_test_image() -> DynamicImage {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
            if (x + y) % 2 == 0 {
                Rgb([255, 255, 255])
            } else {
                Rgb([0, 0, 0])
            }
        });
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_image_processor_new() {
        let processor = ImageProcessor::new();
        assert!(processor.current_image().is_none());
        assert!(processor.original_image().is_none());
        assert!(processor.image_info().is_none());
    }

    #[test]
    fn test_resize_maintain_aspect_ratio() {
        let mut processor = ImageProcessor::new();
        processor.current_image = Some(create_test_image());

        let options = ResizeOptions {
            width: Some(50),
            height: None,
            maintain_aspect_ratio: true,
            filter: FilterType::Nearest,
        };

        processor.resize(&options).unwrap();
        let resized = processor.current_image().unwrap();
        assert_eq!((resized.width(), resized.height()), (50, 50));
    }

    #[test]
    fn test_crop() {
        let mut processor = ImageProcessor::new();
        processor.current_image = Some(create_test_image());

        let crop_rect = CropRect::new(10, 10, 50, 50);
        processor.crop(&crop_rect).unwrap();

        let cropped = processor.current_image().unwrap();
        assert_eq!((cropped.width(), cropped.height()), (50, 50));
    }

    #[test]
    fn test_invalid_crop() {
        let mut processor = ImageProcessor::new();
        processor.current_image = Some(create_test_image());

        let crop_rect = CropRect::new(50, 50, 100, 100); // Goes beyond image bounds
        let result = processor.crop(&crop_rect);
        assert!(result.is_err());
    }

    #[test]
    fn test_rotate() {
        let mut processor = ImageProcessor::new();
        processor.current_image = Some(create_test_image());

        let original_dims = (
            processor.current_image().unwrap().width(),
            processor.current_image().unwrap().height(),
        );

        processor.rotate_90().unwrap();
        let rotated_dims = (
            processor.current_image().unwrap().width(),
            processor.current_image().unwrap().height(),
        );

        // After 90 degree rotation, width and height should be swapped
        assert_eq!(original_dims, (rotated_dims.1, rotated_dims.0));
    }

    #[test]
    fn test_flip() {
        let mut processor = ImageProcessor::new();
        processor.current_image = Some(create_test_image());

        let original_dims = (
            processor.current_image().unwrap().width(),
            processor.current_image().unwrap().height(),
        );

        processor.flip_horizontal().unwrap();
        let flipped_dims = (
            processor.current_image().unwrap().width(),
            processor.current_image().unwrap().height(),
        );

        // Dimensions should remain the same after flipping
        assert_eq!(original_dims, flipped_dims);
    }
}

