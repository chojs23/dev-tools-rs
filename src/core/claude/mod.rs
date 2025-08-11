use anyhow::{anyhow, Result};
use reqwest::Client;
use std::path::PathBuf;
use tokio::fs;

pub mod file_handler;
pub mod types;

use file_handler::FileHandler;
use types::*;

#[derive(Clone, Debug)]
pub struct ClaudeApi {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ClaudeApi {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }

    pub async fn send_message(&self, request: ClaudeRequest) -> Result<ClaudeResponse> {
        let url = format!("{}/messages", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Claude API error: {}", error_text));
        }

        let claude_response: ClaudeResponse = response.json().await?;
        Ok(claude_response)
    }

    pub async fn send_with_files(
        &self,
        text_prompt: String,
        files: Vec<PathBuf>,
        project_path: Option<PathBuf>,
    ) -> Result<ClaudeResponse> {
        let file_handler = FileHandler::new();
        let mut content_parts = vec![];

        // Add text prompt
        content_parts.push(ContentPart::Text {
            text: text_prompt.clone(),
        });

        // Process files
        for file_path in files {
            match file_handler.process_file(&file_path).await {
                Ok(file_content) => {
                    content_parts.push(file_content);
                }
                Err(e) => {
                    eprintln!("Error processing file {:?}: {}", file_path, e);
                    // Continue with other files even if one fails
                }
            }
        }

        // Add project context if provided
        if let Some(project_path) = project_path {
            let project_context = self.analyze_project_structure(&project_path).await?;
            content_parts.push(ContentPart::Text {
                text: format!("\n\nProject Context:\n{}", project_context),
            });
        }

        let request = ClaudeRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            messages: vec![Message {
                role: "user".to_string(),
                content: content_parts,
            }],
            system: Some(self.get_agent_system_prompt()),
            stream: Some(false),
        };

        self.send_message(request).await
    }

    pub async fn send_with_conversation(
        &self,
        text_prompt: String,
        files: Vec<PathBuf>,
        project_path: Option<PathBuf>,
        conversation_history: Vec<Message>,
    ) -> Result<ClaudeResponse> {
        let file_handler = FileHandler::new();
        let mut content_parts = vec![];

        // Add text prompt
        content_parts.push(ContentPart::Text {
            text: text_prompt.clone(),
        });

        // Process files (only for the current message)
        for file_path in files {
            match file_handler.process_file(&file_path).await {
                Ok(file_content) => {
                    content_parts.push(file_content);
                }
                Err(e) => {
                    eprintln!("Error processing file {:?}: {}", file_path, e);
                    // Continue with other files even if one fails
                }
            }
        }

        // Add project context if provided (only for the current message)
        if let Some(project_path) = project_path {
            let project_context = self.analyze_project_structure(&project_path).await?;
            content_parts.push(ContentPart::Text {
                text: format!("\n\nProject Context:\n{}", project_context),
            });
        }

        // Build complete conversation history
        let mut messages = conversation_history;
        println!("Conversation history length: {}", messages.len());

        // Add current user message
        messages.push(Message {
            role: "user".to_string(),
            content: content_parts,
        });

        let request = ClaudeRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            messages,
            system: Some(self.get_agent_system_prompt()),
            stream: Some(false),
        };

        self.send_message(request).await
    }

    pub async fn send_with_conversation_streaming(
        &self,
        text_prompt: String,
        files: Vec<PathBuf>,
        project_path: Option<PathBuf>,
        conversation_history: Vec<Message>,
        mut stream_callback: impl FnMut(String) + Send + 'static,
    ) -> Result<ClaudeResponse> {
        use futures::StreamExt;

        let file_handler = FileHandler::new();
        let mut content_parts = vec![];

        // Add text prompt
        content_parts.push(ContentPart::Text {
            text: text_prompt.clone(),
        });

        // Process files (only for the current message)
        for file_path in files {
            match file_handler.process_file(&file_path).await {
                Ok(file_content) => {
                    content_parts.push(file_content);
                }
                Err(e) => {
                    eprintln!("Error processing file {:?}: {}", file_path, e);
                    // Continue with other files even if one fails
                }
            }
        }

        // Add project context if provided (only for the current message)
        if let Some(project_path) = project_path {
            let project_context = self.analyze_project_structure(&project_path).await?;
            content_parts.push(ContentPart::Text {
                text: format!("\n\nProject Context:\n{}", project_context),
            });
        }

        // Build complete conversation history
        let mut messages = conversation_history;
        println!("Conversation history length: {}", messages.len());

        // Add current user message
        messages.push(Message {
            role: "user".to_string(),
            content: content_parts,
        });

        let request = ClaudeRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            messages,
            system: Some(self.get_agent_system_prompt()),
            stream: Some(true),
        };

        self.send_message_streaming(request, stream_callback).await
    }

    pub async fn send_message_streaming(
        &self,
        request: ClaudeRequest,
        mut stream_callback: impl FnMut(String) + Send + 'static,
    ) -> Result<ClaudeResponse> {
        use futures::StreamExt;

        let url = format!("{}/messages", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Claude API error: {}", error_text));
        }

        let mut stream = response.bytes_stream();
        let mut accumulated_response = String::new();
        let mut message_id = String::new();
        let mut model = String::new();
        let mut usage = Usage {
            input_tokens: 0,
            output_tokens: 0,
        };

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let chunk_str = String::from_utf8_lossy(&chunk);

            // Parse Server-Sent Events
            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..]; // Remove "data: " prefix

                    if data == "[DONE]" {
                        break;
                    }

                    if let Ok(event) = serde_json::from_str::<StreamingEventData>(data) {
                        match event {
                            StreamingEventData::MessageStart { message } => {
                                message_id = message.id;
                                model = message.model;
                                usage = message.usage;
                            }
                            StreamingEventData::ContentBlockDelta { delta, .. } => {
                                if let Some(text) = delta.text {
                                    accumulated_response.push_str(&text);
                                    stream_callback(text);
                                }
                            }
                            StreamingEventData::MessageDelta { usage: Some(u), .. } => {
                                usage = u;
                            }
                            StreamingEventData::Error { error } => {
                                return Err(anyhow!(
                                    "Claude API streaming error: {}",
                                    error.message
                                ));
                            }
                            _ => {} // Ignore other event types
                        }
                    }
                }
            }
        }

        // Return a complete response
        Ok(ClaudeResponse {
            id: message_id,
            model,
            role: "assistant".to_string(),
            content: vec![ContentPart::Text {
                text: accumulated_response,
            }],
            stop_reason: Some("end_turn".to_string()),
            usage,
        })
    }

    async fn analyze_project_structure(&self, project_path: &PathBuf) -> Result<String> {
        let mut structure = String::new();
        structure.push_str(&format!(
            "Project directory: {}\n\n",
            project_path.display()
        ));

        // Get basic project structure
        if let Ok(entries) = fs::read_dir(project_path).await {
            structure.push_str("Project structure:\n");
            let mut entries_vec = vec![];

            let mut entries = entries;
            while let Some(entry) = entries.next_entry().await? {
                entries_vec.push(entry);
            }

            for entry in entries_vec {
                let path = entry.path();
                let name = path.file_name().unwrap().to_string_lossy();

                if path.is_dir() {
                    structure.push_str(&format!("📁 {}/\n", name));
                } else {
                    structure.push_str(&format!("📄 {}\n", name));
                }
            }
        }

        // Look for common project files
        let common_files = [
            "package.json",
            "Cargo.toml",
            "requirements.txt",
            "pom.xml",
            "build.gradle",
            "Makefile",
            "README.md",
            ".gitignore",
        ];

        structure.push_str("\nProject configuration files found:\n");
        for file in common_files {
            let file_path = project_path.join(file);
            if file_path.exists() {
                structure.push_str(&format!("✅ {}\n", file));

                // Read content of key files
                if file == "package.json" || file == "Cargo.toml" {
                    if let Ok(content) = fs::read_to_string(&file_path).await {
                        structure
                            .push_str(&format!("\n{} content:\n```\n{}\n```\n", file, content));
                    }
                }
            }
        }

        Ok(structure)
    }

    fn get_agent_system_prompt(&self) -> String {
        r#"You are an expert software development assistant with agent capabilities. Your role is to:

1. Analyze provided files (documents, images, PDFs) and understand the requirements
2. Examine project structure and existing codebase
3. Generate high-quality, production-ready code that follows best practices
4. Provide detailed explanations and documentation for your implementations
5. Consider error handling, testing, and maintainability

When generating code:
- Follow the existing code style and patterns in the project
- Include proper error handling and validation
- Add comprehensive comments and documentation
- Consider performance and security implications
- Suggest additional files or modifications needed
- Provide clear instructions for integration

When analyzing documents:
- Extract key requirements and specifications
- Identify technical constraints and dependencies
- Suggest optimal implementation approaches
- Flag potential issues or ambiguities

Your responses should be structured, professional, and actionable."#.to_string()
    }
}

#[derive(Default, Clone, Debug)]
pub struct ClaudeProcessor {
    pub api_key: String,
    pub current_conversation: Vec<Message>,
    pub project_path: Option<PathBuf>,
    pub selected_files: Vec<PathBuf>,
    pub last_response: Option<String>,
    pub is_processing: bool,
}

impl ClaudeProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }

    pub fn set_project_path(&mut self, path: PathBuf) {
        self.project_path = Some(path);
    }

    pub fn add_file(&mut self, file_path: PathBuf) {
        if !self.selected_files.contains(&file_path) {
            self.selected_files.push(file_path);
        }
    }

    pub fn remove_file(&mut self, file_path: &PathBuf) {
        self.selected_files.retain(|f| f != file_path);
    }

    pub fn clear_files(&mut self) {
        self.selected_files.clear();
    }

    pub async fn send_request(&mut self, prompt: String) -> Result<String> {
        if self.api_key.is_empty() {
            return Err(anyhow!("API key is required"));
        }

        self.is_processing = true;

        let api = ClaudeApi::new(self.api_key.clone());

        // Add user message to conversation history before sending
        let user_message_content = vec![ContentPart::Text {
            text: prompt.clone(),
        }];

        // Use send_with_conversation to include chat history
        let response = api
            .send_with_conversation(
                prompt,
                self.selected_files.clone(),
                self.project_path.clone(),
                self.current_conversation.clone(),
            )
            .await;

        self.is_processing = false;

        match response {
            Ok(resp) => {
                if let Some(content) = resp.content.first() {
                    if let ContentPart::Text { text } = content {
                        self.last_response = Some(text.clone());

                        // Add user message to conversation history (this was sent to API)
                        self.current_conversation.push(Message {
                            role: "user".to_string(),
                            content: user_message_content,
                        });

                        // Add assistant response to conversation history
                        self.current_conversation.push(Message {
                            role: "assistant".to_string(),
                            content: vec![ContentPart::Text { text: text.clone() }],
                        });

                        Ok(text.clone())
                    } else {
                        Err(anyhow!("Unexpected response format"))
                    }
                } else {
                    Err(anyhow!("Empty response from Claude"))
                }
            }
            Err(e) => {
                eprintln!("Error sending request: {}", e);
                Err(anyhow!("Failed to get response from Claude API"))
            }
        }
    }

    pub fn clear_conversation(&mut self) {
        self.current_conversation.clear();
        self.last_response = None;
    }
}
