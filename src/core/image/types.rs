use image::{imageops, DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormatType {
    Jpeg,
    Png,
    Gif,
    WebP,
    Tiff,
    Bmp,
    Ico,
}

impl ImageFormatType {
    pub fn to_image_format(self) -> ImageFormat {
        match self {
            ImageFormatType::Jpeg => ImageFormat::Jpeg,
            ImageFormatType::Png => ImageFormat::Png,
            ImageFormatType::Gif => ImageFormat::Gif,
            ImageFormatType::WebP => ImageFormat::WebP,
            ImageFormatType::Tiff => ImageFormat::Tiff,
            ImageFormatType::Bmp => ImageFormat::Bmp,
            ImageFormatType::Ico => ImageFormat::Ico,
        }
    }

    pub fn from_image_format(format: ImageFormat) -> Option<Self> {
        match format {
            ImageFormat::Jpeg => Some(ImageFormatType::Jpeg),
            ImageFormat::Png => Some(ImageFormatType::Png),
            ImageFormat::Gif => Some(ImageFormatType::Gif),
            ImageFormat::WebP => Some(ImageFormatType::WebP),
            ImageFormat::Tiff => Some(ImageFormatType::Tiff),
            ImageFormat::Bmp => Some(ImageFormatType::Bmp),
            ImageFormat::Ico => Some(ImageFormatType::Ico),
            _ => None,
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            ImageFormatType::Jpeg => "jpg",
            ImageFormatType::Png => "png",
            ImageFormatType::Gif => "gif",
            ImageFormatType::WebP => "webp",
            ImageFormatType::Tiff => "tiff",
            ImageFormatType::Bmp => "bmp",
            ImageFormatType::Ico => "ico",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            ImageFormatType::Jpeg => "JPEG",
            ImageFormatType::Png => "PNG",
            ImageFormatType::Gif => "GIF",
            ImageFormatType::WebP => "WebP",
            ImageFormatType::Tiff => "TIFF",
            ImageFormatType::Bmp => "BMP",
            ImageFormatType::Ico => "ICO",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            ImageFormatType::Jpeg,
            ImageFormatType::Png,
            ImageFormatType::Gif,
            ImageFormatType::WebP,
            ImageFormatType::Tiff,
            ImageFormatType::Bmp,
            ImageFormatType::Ico,
        ]
    }
}

impl Default for ImageFormatType {
    fn default() -> Self {
        ImageFormatType::Png
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterType {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl FilterType {
    pub fn to_image_filter(self) -> imageops::FilterType {
        match self {
            FilterType::Nearest => imageops::FilterType::Nearest,
            FilterType::Triangle => imageops::FilterType::Triangle,
            FilterType::CatmullRom => imageops::FilterType::CatmullRom,
            FilterType::Gaussian => imageops::FilterType::Gaussian,
            FilterType::Lanczos3 => imageops::FilterType::Lanczos3,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            FilterType::Nearest => "Nearest",
            FilterType::Triangle => "Triangle",
            FilterType::CatmullRom => "Catmull-Rom",
            FilterType::Gaussian => "Gaussian",
            FilterType::Lanczos3 => "Lanczos3",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            FilterType::Nearest,
            FilterType::Triangle,
            FilterType::CatmullRom,
            FilterType::Gaussian,
            FilterType::Lanczos3,
        ]
    }
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::Lanczos3
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl CropRect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn is_valid_for_image(&self, image: &DynamicImage) -> bool {
        let (img_width, img_height) = (image.width(), image.height());
        self.x < img_width
            && self.y < img_height
            && self.x + self.width <= img_width
            && self.y + self.height <= img_height
            && self.width > 0
            && self.height > 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub maintain_aspect_ratio: bool,
    pub filter: FilterType,
}

impl Default for ResizeOptions {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            maintain_aspect_ratio: true,
            filter: FilterType::Lanczos3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionOptions {
    pub quality: u8, // 0-100 for JPEG
    pub format: ImageFormatType,
}

impl Default for CompressionOptions {
    fn default() -> Self {
        Self {
            quality: 85,
            format: ImageFormatType::Jpeg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub format: Option<ImageFormatType>,
    pub file_size: Option<u64>,
    pub color_type: String,
}

impl ImageInfo {
    pub fn from_image(image: &DynamicImage, file_size: Option<u64>) -> Self {
        Self {
            width: image.width(),
            height: image.height(),
            format: None, // Will be set when loading from file
            file_size,
            color_type: format!("{:?}", image.color()),
        }
    }
}

