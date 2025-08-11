use anyhow::{anyhow, Result};
use std::path::Path;
use tokio::fs;
use base64::Engine;

use crate::core::claude::types::*;

pub struct FileHandler;

impl FileHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn process_file(&self, file_path: &Path) -> Result<ContentPart> {
        let file_info = FileInfo::new(file_path.to_path_buf())?;
        
        match file_info.file_type {
            FileType::Text => self.process_text_file(file_path).await,
            FileType::Image => self.process_image_file(file_path).await,
            FileType::Pdf => self.process_pdf_file(file_path).await,
            FileType::Unknown => {
                // Try to read as text first
                match self.process_text_file(file_path).await {
                    Ok(content) => Ok(content),
                    Err(_) => Err(anyhow!("Unsupported file type: {}", file_path.display())),
                }
            }
        }
    }

    async fn process_text_file(&self, file_path: &Path) -> Result<ContentPart> {
        let content = fs::read_to_string(file_path).await?;
        let file_name = file_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        Ok(ContentPart::Text {
            text: format!(
                "File: {}\n```\n{}\n```",
                file_name,
                content
            ),
        })
    }

    async fn process_image_file(&self, file_path: &Path) -> Result<ContentPart> {
        let image_data = fs::read(file_path).await?;
        let base64_data = base64::engine::general_purpose::STANDARD.encode(&image_data);
        
        let media_type = mime_guess::from_path(file_path)
            .first_or_octet_stream()
            .to_string();

        Ok(ContentPart::Image {
            source: ImageSource {
                source_type: "base64".to_string(),
                media_type,
                data: base64_data,
            },
        })
    }

    async fn process_pdf_file(&self, file_path: &Path) -> Result<ContentPart> {
        // First try to extract text from PDF
        match self.extract_pdf_text(file_path).await {
            Ok(text) => {
                let file_name = file_path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown.pdf");

                Ok(ContentPart::Text {
                    text: format!(
                        "PDF File: {}\nExtracted Text:\n```\n{}\n```",
                        file_name,
                        text
                    ),
                })
            }
            Err(_) => {
                // If text extraction fails, send as document
                let pdf_data = fs::read(file_path).await?;
                let base64_data = base64::engine::general_purpose::STANDARD.encode(&pdf_data);

                Ok(ContentPart::Document {
                    source: DocumentSource {
                        source_type: "base64".to_string(),
                        media_type: "application/pdf".to_string(),
                        data: base64_data,
                    },
                })
            }
        }
    }

    async fn extract_pdf_text(&self, file_path: &Path) -> Result<String> {
        // Use pdf-extract to extract text from PDF
        let bytes = fs::read(file_path).await?;
        
        tokio::task::spawn_blocking(move || {
            pdf_extract::extract_text_from_mem(&bytes)
                .map_err(|e| anyhow!("Failed to extract PDF text: {}", e))
        })
        .await?
    }

    pub fn get_supported_extensions() -> Vec<&'static str> {
        vec![
            // Text files
            "txt", "md", "rs", "py", "js", "ts", "html", "css", "json", "xml", "yaml", "yml", "toml",
            "c", "cpp", "h", "hpp", "java", "kt", "swift", "go", "rb", "php", "sh", "bat", "ps1",
            // Image files
            "jpg", "jpeg", "png", "gif", "bmp", "webp",
            // Document files
            "pdf",
        ]
    }

    pub fn is_supported_file(file_path: &Path) -> bool {
        if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
            Self::get_supported_extensions().contains(&extension.to_lowercase().as_str())
        } else {
            false
        }
    }

    pub async fn validate_file(&self, file_path: &Path) -> Result<FileInfo> {
        if !file_path.exists() {
            return Err(anyhow!("File does not exist: {}", file_path.display()));
        }

        let file_info = FileInfo::new(file_path.to_path_buf())?;

        // Check file size (limit to 20MB for safety)
        const MAX_FILE_SIZE: u64 = 20 * 1024 * 1024; // 20MB
        if file_info.size > MAX_FILE_SIZE {
            return Err(anyhow!(
                "File too large: {} bytes (max: {} bytes)",
                file_info.size,
                MAX_FILE_SIZE
            ));
        }

        if !Self::is_supported_file(file_path) {
            return Err(anyhow!(
                "Unsupported file type: {}",
                file_path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("unknown")
            ));
        }

        Ok(file_info)
    }
}

