use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeRequest {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Vec<ContentPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image {
        source: ImageSource,
    },
    #[serde(rename = "document")]
    Document {
        source: DocumentSource,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String, // "base64"
    pub media_type: String,  // "image/jpeg", "image/png", etc.
    pub data: String,        // base64 encoded image data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSource {
    #[serde(rename = "type")]
    pub source_type: String, // "base64"
    pub media_type: String,  // "application/pdf", "text/plain", etc.
    pub data: String,        // base64 encoded document data
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClaudeResponse {
    pub id: String,
    pub model: String,
    pub role: String,
    pub content: Vec<ContentPart>,
    pub stop_reason: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Clone)]
pub enum FileType {
    Text,
    Image,
    Pdf,
    Unknown,
}

// Streaming response types
#[derive(Debug, Clone, Deserialize)]
pub struct StreamingEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(flatten)]
    pub data: StreamingEventData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StreamingEventData {
    MessageStart {
        message: StreamingMessage,
    },
    ContentBlockStart {
        index: u32,
        content_block: ContentBlock,
    },
    ContentBlockDelta {
        index: u32,
        delta: ContentDelta,
    },
    ContentBlockStop {
        index: u32,
    },
    MessageDelta {
        delta: MessageDelta,
        usage: Option<Usage>,
    },
    MessageStop,
    Ping,
    Error {
        error: ApiError,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamingMessage {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub role: String,
    pub content: Vec<ContentPart>,
    pub model: String,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentDelta {
    #[serde(rename = "type")]
    pub delta_type: String,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageDelta {
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

impl FileType {
    pub fn from_extension(extension: &str) -> Self {
        match extension.to_lowercase().as_str() {
            "txt" | "md" | "rs" | "py" | "js" | "ts" | "html" | "css" | "json" | "xml" | "yaml" | "yml" | "toml" => Self::Text,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" => Self::Image,
            "pdf" => Self::Pdf,
            _ => Self::Unknown,
        }
    }

    pub fn to_mime_type(&self) -> &'static str {
        match self {
            Self::Text => "text/plain",
            Self::Image => "image/jpeg", // Default, will be overridden by actual detection
            Self::Pdf => "application/pdf",
            Self::Unknown => "application/octet-stream",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: std::path::PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub name: String,
}

impl FileInfo {
    pub fn new(path: std::path::PathBuf) -> std::io::Result<Self> {
        let metadata = std::fs::metadata(&path)?;
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        Ok(Self {
            file_type: FileType::from_extension(extension),
            size: metadata.len(),
            name: path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            path,
        })
    }
}

