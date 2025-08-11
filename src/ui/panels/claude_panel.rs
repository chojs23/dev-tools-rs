use eframe::egui::{RichText, ScrollArea, TextEdit, Ui, Color32, Button};
use std::path::PathBuf;

use crate::{
    context::FrameCtx,
    core::claude::ClaudeProcessor,
    ui::traits::UiPanel,
};

// Enum to handle different types of streaming events
#[derive(Debug, Clone)]
pub enum StreamingEvent {
    Delta(String),          // New text chunk
    Complete(String),       // Final complete response
    Error(String),          // Error message
}

// Struct to represent a detected file operation from Claude's response
#[derive(Debug, Clone)]
pub struct FileOperation {
    pub operation_type: FileOperationType,
    pub file_path: String,
    pub content: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum FileOperationType {
    Create,
    Update,
    Replace,
}

pub struct ClaudePanel {
    pub processor: ClaudeProcessor,
    pub prompt_input: String,
    pub response_output: String,
    pub api_key_input: String,
    pub project_path_input: String,
    pub show_api_key: bool,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub selected_files_display: String,
    pub streaming_receiver: Option<std::sync::mpsc::Receiver<StreamingEvent>>,
    pub is_streaming: bool,
    pub stream_buffer: String, // Buffer to accumulate streaming text
    // Retry logic fields
    pub retry_count: u32,
    pub max_retries: u32,
    pub last_failed_request: Option<String>, // Store the last failed prompt for retry
    pub retry_delay_ms: u64,
    pub is_retrying: bool,
    pub last_error: Option<String>,
    pub retry_receiver: Option<std::sync::mpsc::Receiver<()>>, // For retry delay timing
    // Timeout handling fields
    pub request_timeout_seconds: u64,
    pub stream_start_time: Option<std::time::Instant>,
    pub enable_timeout: bool,
    pub timeout_receiver: Option<std::sync::mpsc::Receiver<()>>, // For timeout detection
    // Agentic mode fields
    pub agentic_mode_enabled: bool,
    pub detected_file_operations: Vec<FileOperation>,
    pub auto_apply_changes: bool,
    pub show_file_operations: bool,
    pub backup_enabled: bool,
}

impl ClaudePanel {
    pub fn new() -> Self {
        Self {
            processor: ClaudeProcessor::new(),
            prompt_input: String::new(),
            response_output: String::new(),
            api_key_input: String::new(),
            project_path_input: String::new(),
            show_api_key: false,
            error_message: None,
            success_message: None,
            selected_files_display: String::new(),
            streaming_receiver: None,
            is_streaming: false,
            stream_buffer: String::new(),
            retry_count: 0,
            max_retries: 3,
            last_failed_request: None,
            retry_delay_ms: 2000, // 2 second initial delay
            is_retrying: false,
            last_error: None,
            retry_receiver: None,
            // Timeout handling initialization
            request_timeout_seconds: 120, // 2 minute default timeout
            stream_start_time: None,
            enable_timeout: true,
            timeout_receiver: None,
            // Agentic mode initialization
            agentic_mode_enabled: false,
            detected_file_operations: Vec::new(),
            auto_apply_changes: false,
            show_file_operations: true,
            backup_enabled: true,
        }
    }

    fn update_files_display(&mut self) {
        self.selected_files_display = if self.processor.selected_files.is_empty() {
            "No files selected".to_string()
        } else {
            self.processor
                .selected_files
                .iter()
                .map(|path| {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("Unknown")
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
    }

    fn render_api_key_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("🔑 Claude API Configuration").strong());
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("API Key:");
                    if self.show_api_key {
                        ui.text_edit_singleline(&mut self.api_key_input);
                    } else {
                        let mut masked = "*".repeat(self.api_key_input.len().min(20));
                        ui.add_enabled(false, TextEdit::singleline(&mut masked));
                    }
                    
                    if ui.button(if self.show_api_key { "👁" } else { "👁‍🗨" }).clicked() {
                        self.show_api_key = !self.show_api_key;
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("💾 Save API Key").clicked() {
                        if !self.api_key_input.is_empty() {
                            self.processor.set_api_key(self.api_key_input.clone());
                            self.success_message = Some("API key saved successfully!".to_string());
                            self.error_message = None;
                        } else {
                            self.error_message = Some("API key cannot be empty".to_string());
                            self.success_message = None;
                        }
                    }

                    if ui.button("🗑 Clear").clicked() {
                        self.api_key_input.clear();
                        self.processor.set_api_key(String::new());
                    }
                });

                // Status indicator
                ui.horizontal(|ui| {
                    let status = if self.processor.api_key.is_empty() {
                        ("❌", "Not configured", Color32::RED)
                    } else {
                        ("✅", "Configured", Color32::GREEN)
                    };
                    ui.label(RichText::new(format!("{} Status: {}", status.0, status.1)).color(status.2));
                });
            });
        });
    }

    fn render_project_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("📁 Project Configuration").strong());
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Project Path:");
                    ui.text_edit_singleline(&mut self.project_path_input);
                    
                    if ui.button("📂 Browse").clicked() {
                        // TODO: Implement file dialog for project selection
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                self.project_path_input = path.display().to_string();
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("💾 Set Project Path").clicked() && !self.project_path_input.is_empty() {
                            let path = PathBuf::from(&self.project_path_input);
                            if path.exists() && path.is_dir() {
                                self.processor.set_project_path(path);
                                self.success_message = Some("Project path set successfully!".to_string());
                                self.error_message = None;
                            } else {
                                self.error_message = Some("Invalid project path".to_string());
                                self.success_message = None;
                            }
                        }

                    if ui.button("🗑 Clear").clicked() {
                        self.project_path_input.clear();
                        self.processor.project_path = None;
                    }
                });

                // Current project status
                if let Some(ref path) = self.processor.project_path {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("✅ Project:").color(Color32::GREEN));
                        ui.label(path.display().to_string());
                    });
                }
            });
        });
    }

    fn render_files_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("📎 File Attachments").strong());
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("📁 Add Files").clicked() {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let supported_extensions = crate::core::claude::file_handler::FileHandler::get_supported_extensions();
                                                let _filter_string = supported_extensions.join(",");
                            
                            if let Some(paths) = rfd::FileDialog::new()
                                .add_filter("Supported Files", &supported_extensions)
                                .pick_files()
                            {
                                for path in paths {
                                    self.processor.add_file(path);
                                }
                                self.update_files_display();
                            }
                        }
                    }

                    if ui.button("🗑 Clear All").clicked() {
                        self.processor.clear_files();
                        self.update_files_display();
                    }
                });

                // Display selected files
                ui.label(format!("Selected files: {}", self.selected_files_display));

                // List files with remove buttons
                if !self.processor.selected_files.is_empty() {
                    ScrollArea::vertical()
                        .max_height(100.0)
                        .id_salt("file_list")
                        .show(ui, |ui| {
                            let mut to_remove = None;
                            for (i, file_path) in self.processor.selected_files.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    let file_name = file_path
                                        .file_name()
                                        .and_then(|name| name.to_str())
                                        .unwrap_or("Unknown");
                                    
                                    ui.label(format!("📄 {}", file_name));
                                    
                                    if ui.small_button("❌").clicked() {
                                        to_remove = Some(i);
                                    }
                                });
                            }
                            
                            if let Some(index) = to_remove {
                                self.processor.selected_files.remove(index);
                                self.update_files_display();
                            }
                        });
                }
            });
        });
    }

    fn render_chat_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("💬 Claude AI Assistant").strong());
                ui.separator();

                // Prompt input
                ui.label("Your prompt:");
                ui.add(
                    TextEdit::multiline(&mut self.prompt_input)
                        .desired_rows(4)
                        .desired_width(f32::INFINITY)
                        .hint_text("Enter your question or request here...")
                );

                ui.horizontal(|ui| {
                    let send_button = Button::new("🚀 Send to Claude")
                        .min_size([120.0, 30.0].into());
                    
                    let can_send = !self.processor.api_key.is_empty() 
                        && !self.prompt_input.trim().is_empty() 
                        && !self.processor.is_processing;

                    if ui.add_enabled(can_send, send_button).clicked() {
                        self.send_request();
                    }

                    if self.processor.is_processing {
                        ui.label("⏳ Processing...");
                    }

                    if ui.button("🗑 Clear Chat").clicked() {
                        self.processor.clear_conversation();
                        self.response_output.clear();
                        self.prompt_input.clear();
                    }
                });

                ui.separator();

                // Response output
                ui.label("Claude's Response:");
                ScrollArea::vertical()
                    .max_height(300.0)
                    .id_salt("claude_response")
                    .show(ui, |ui| {
                        ui.add(
                            TextEdit::multiline(&mut self.response_output)
                                .desired_width(f32::INFINITY)
                                .desired_rows(10)
                                .interactive(false)
                        );
                    });

                // Copy response button
                if !self.response_output.is_empty() && ui.button("📋 Copy Response").clicked() {
                                                #[cfg(not(target_arch = "wasm32"))]
                                                {
                                                    if let Err(e) = save_to_clipboard(self.response_output.clone()) {
                                self.error_message = Some(format!("Failed to copy: {}", e));
                            } else {
                                self.success_message = Some("Response copied to clipboard!".to_string());
                            }
                        }
                    }
            });
        });
    }

    fn render_messages(&mut self, ui: &mut Ui) {
        // Error messages
        if let Some(error) = self.error_message.clone() {
            ui.horizontal(|ui| {
                ui.label(RichText::new("❌").color(Color32::RED));
                ui.label(RichText::new(error).color(Color32::RED));
                if ui.small_button("✖").clicked() {
                    self.error_message = None;
                }
            });
        }

        // Success messages
        if let Some(success) = self.success_message.clone() {
            ui.horizontal(|ui| {
                ui.label(RichText::new("✅").color(Color32::GREEN));
                ui.label(RichText::new(success).color(Color32::GREEN));
                if ui.small_button("✖").clicked() {
                    self.success_message = None;
                }
            });
        }
    }

    fn send_request(&mut self) {
        if self.processor.api_key.is_empty() {
            self.error_message = Some("API key is required".to_string());
            return;
        }

        if self.prompt_input.trim().is_empty() {
            self.error_message = Some("Please enter a prompt".to_string());
            return;
        }

        let prompt = self.prompt_input.clone();
        let prompt_for_display = prompt.clone(); // Clone for display purposes
        
        // Store the request for potential retry
        self.last_failed_request = Some(prompt.clone());
        
        // Reset retry state if this is a new request (not a retry)
        if !self.is_retrying {
            self.retry_count = 0;
            self.last_error = None;
        }
        
        self.processor.is_processing = true;
        self.is_streaming = true;
        self.stream_buffer.clear();
        
        // Start timeout timer if enabled
        if self.enable_timeout {
            self.stream_start_time = Some(std::time::Instant::now());
            
            let timeout_seconds = self.request_timeout_seconds;
            let (timeout_tx, timeout_rx) = std::sync::mpsc::channel();
            
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(timeout_seconds));
                let _ = timeout_tx.send(());
            });
            
            self.timeout_receiver = Some(timeout_rx);
        }
        
        let status_msg = if self.is_retrying {
            format!("🔄 Retrying request (attempt {}/{})...", self.retry_count + 1, self.max_retries)
        } else {
            "⏳ Connecting to Claude...".to_string()
        };
        self.response_output = status_msg;
        
        // Create a channel for streaming communication between async task and UI
        let (tx, rx) = std::sync::mpsc::channel();
        let processor = self.processor.clone();
        let retry_count = self.retry_count;

        // Spawn thread with tokio runtime for async streaming operation
        std::thread::spawn(move || {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = tx.send(StreamingEvent::Error(format!("Failed to create tokio runtime: {}", e)));
                    return;
                }
            };
            
            let stream_tx = tx.clone();
            let result = rt.block_on(async move {
                use crate::core::claude::ClaudeApi;
                
                let api = ClaudeApi::new(processor.api_key.clone());
                
                // Use streaming API with callback
                api.send_with_conversation_streaming(
                    prompt,
                    processor.selected_files.clone(),
                    processor.project_path.clone(),
                    processor.current_conversation.clone(),
                    move |delta: String| {
                        let _ = stream_tx.send(StreamingEvent::Delta(delta));
                    }
                ).await
            });
            
            match result {
                Ok(response) => {
                    if let Some(content) = response.content.first() {
                        if let crate::core::claude::types::ContentPart::Text { text } = content {
                            let _ = tx.send(StreamingEvent::Complete(text.clone()));
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Claude API error (attempt {}): {}", retry_count + 1, e);
                    let _ = tx.send(StreamingEvent::Error(error_msg));
                }
            }
        });

        // Store the receiver for polling in the UI update loop
        self.streaming_receiver = Some(rx);
        
        self.response_output = format!(
            "🚀 Streaming request to Claude...\n• Prompt: {}\n• Files: {}\n• Project: {}\n• Attempt: {}/{}\n\n⏳ Waiting for response...",
            if self.prompt_input.is_empty() { &prompt_for_display } else { &self.prompt_input },
            self.selected_files_display,
            self.processor.project_path.as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.retry_count + 1,
            self.max_retries
        );

        if !self.is_retrying {
            self.prompt_input.clear();
        }
        
        let success_msg = if self.is_retrying {
            format!("🔄 Retrying streaming request (attempt {}/{})...", self.retry_count + 1, self.max_retries)
        } else {
            "Streaming request sent to Claude AI!".to_string()
        };
        self.success_message = Some(success_msg);
        self.error_message = None;
    }

    fn handle_timeout(&mut self) {
        // Clean up streaming state
        self.processor.is_processing = false;
        self.is_streaming = false;
        self.streaming_receiver = None;
        self.timeout_receiver = None;
        self.stream_start_time = None;
        
        // Reset retry state since timeout is a terminal condition
        self.is_retrying = false;
        self.retry_receiver = None;
        
        let timeout_seconds = self.request_timeout_seconds;
        self.response_output = format!(
            "⏰ Request timed out after {} seconds\n\n❌ The streaming request took too long to complete.\n\n💡 You can try:\n• Check your internet connection\n• Verify Claude's servers are responding\n• Try a shorter or simpler prompt\n• Increase the timeout duration if needed",
            timeout_seconds
        );
        
        self.error_message = Some(format!(
            "Request timed out after {} seconds. Try again with a shorter prompt or check your connection.",
            timeout_seconds
        ));
        self.success_message = None;
    }

    fn check_streaming_response(&mut self) {
        // Check for timeout
        if let Some(ref timeout_receiver) = self.timeout_receiver {
            if let Ok(()) = timeout_receiver.try_recv() {
                // Timeout occurred, handle it
                self.handle_timeout();
                return; // Exit early since we're handling timeout
            }
        }
        
        // Check if retry delay has completed
        if let Some(ref retry_receiver) = self.retry_receiver {
            if let Ok(()) = retry_receiver.try_recv() {
                // Retry delay completed, trigger retry
                self.retry_receiver = None; // Clean up the receiver
                
                // Retry the request
                if let Some(ref last_request) = self.last_failed_request.clone() {
                    self.prompt_input = last_request.clone();
                    self.send_request();
                }
                return; // Exit early since we're starting a new request
            }
        }
        
        if let Some(ref receiver) = self.streaming_receiver {
            // Process all available streaming events
            let mut events_processed = 0;
            const MAX_EVENTS_PER_FRAME: usize = 10; // Limit events per frame to avoid UI lag
            
            while events_processed < MAX_EVENTS_PER_FRAME {
                match receiver.try_recv() {
                    Ok(event) => {
                        events_processed += 1;
                        match event {
                            StreamingEvent::Delta(delta) => {
                                // Append the delta to our stream buffer
                                self.stream_buffer.push_str(&delta);
                                
                                // Update the response output with accumulated text
                                self.response_output = format!(
                                    "🔄 Streaming response from Claude...\n\n{}{}",
                                    self.stream_buffer,
                                    if self.is_streaming { " ▋" } else { "" } // Show cursor while streaming
                                );
                            }
                            StreamingEvent::Complete(complete_text) => {
                                // Streaming complete, finalize the response
                                self.processor.is_processing = false;
                                self.is_streaming = false;
                                self.streaming_receiver = None;
                                
                                // Update conversation history
                                self.update_conversation_history(complete_text.clone());
                                
                                // Final response without streaming cursor
                                self.response_output = complete_text;
                                self.success_message = Some("✅ Response received from Claude AI!".to_string());
                                self.error_message = None;
                                
                                return; // Exit early since streaming is complete
                            }
                            StreamingEvent::Error(error) => {
                                // Store the error for potential retry
                                self.last_error = Some(error.clone());
                                
                                // Check if we should retry
                                if self.retry_count < self.max_retries {
                                    self.retry_count += 1;
                                    self.is_retrying = true;
                                    
                                    // Clean up current streaming state
                                    self.processor.is_processing = false;
                                    self.is_streaming = false;
                                    self.streaming_receiver = None;
                                    
                                    // Show retry message
                                    self.response_output = format!(
                                        "⚠️ Request failed (attempt {}/{}): {}\n\n🔄 Retrying in {} seconds...",
                                        self.retry_count,
                                        self.max_retries,
                                        error,
                                        self.retry_delay_ms / 1000
                                    );
                                    
                                    // Schedule retry after delay
                                    let delay_ms = self.retry_delay_ms;
                                    let (retry_tx, retry_rx) = std::sync::mpsc::channel();
                                    
                                    std::thread::spawn(move || {
                                        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
                                        let _ = retry_tx.send(());
                                    });
                                    
                                    // Store retry receiver for checking
                                    self.retry_receiver = Some(retry_rx);
                                    
                                    self.error_message = Some(format!(
                                        "Request failed (attempt {}/{}). Retrying in {} seconds...",
                                        self.retry_count,
                                        self.max_retries,
                                        self.retry_delay_ms / 1000
                                    ));
                                    
                                    return;
                                } else {
                                    // Max retries reached, give up
                                    self.processor.is_processing = false;
                                    self.is_streaming = false;
                                    self.streaming_receiver = None;
                                    self.is_retrying = false;
                                    
                                    self.response_output = format!(
                                        "❌ Request failed after {} attempts:\n\nLast error: {}\n\n💡 You can try:\n• Check your internet connection\n• Verify your API key is valid\n• Try again later if Claude's servers are busy",
                                        self.max_retries,
                                        error
                                    );
                                    
                                    self.error_message = Some(format!(
                                        "Failed after {} attempts. Last error: {}",
                                        self.max_retries,
                                        error
                                    ));
                                    self.success_message = None;
                                    
                                    return;
                                }
                            }
                        }
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        // No more events available, break the loop
                        break;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        // Channel disconnected, clean up
                        self.processor.is_processing = false;
                        self.is_streaming = false;
                        self.streaming_receiver = None;
                        
                        if self.stream_buffer.is_empty() {
                            self.error_message = Some("Connection to Claude was lost".to_string());
                            self.response_output = "❌ Connection lost during streaming".to_string();
                        } else {
                            // Use whatever we received so far
                            self.response_output = self.stream_buffer.clone();
                            self.success_message = Some("⚠️ Partial response received (connection ended early)".to_string());
                        }
                        break;
                    }
                }
            }
        }
    }

    fn update_conversation_history(&mut self, complete_response: String) {
        // Add user message to conversation history (based on the last prompt)
        // Note: We can't easily recover the original prompt here, so we'll need to store it
        // This is a limitation of the current design - we should store the user message when sending
        
        // Parse response for file operations if agentic mode is enabled
        if self.agentic_mode_enabled {
            self.parse_file_operations_from_response(&complete_response);
        }
        
        // Add assistant response to conversation history
        self.processor.current_conversation.push(crate::core::claude::types::Message {
            role: "assistant".to_string(),
            content: vec![crate::core::claude::types::ContentPart::Text { 
                text: complete_response 
            }],
        });
    }

    fn parse_file_operations_from_response(&mut self, response: &str) {
        // Clear previous operations
        self.detected_file_operations.clear();
        
        // Parse for file operations using common patterns Claude uses
        self.parse_explicit_file_mentions(response);
        self.parse_code_blocks_with_file_paths(response);
        self.parse_file_creation_statements(response);
    }

    fn parse_explicit_file_mentions(&mut self, response: &str) {
        use regex::Regex;
        
        // Pattern for explicit file operations like "create file src/main.rs" or "update config.json"
        let file_op_patterns = vec![
            (r"(?i)(?:create|creating|new)\s+(?:file\s+)?([^\s\n]+\.(?:rs|py|js|ts|json|toml|yaml|yml|md|txt|html|css|jsx|tsx|vue|svelte))", FileOperationType::Create),
            (r"(?i)(?:update|updating|modify|modifying|edit|editing)\s+(?:file\s+)?([^\s\n]+\.(?:rs|py|js|ts|json|toml|yaml|yml|md|txt|html|css|jsx|tsx|vue|svelte))", FileOperationType::Update),
            (r"(?i)(?:replace|replacing)\s+(?:file\s+)?([^\s\n]+\.(?:rs|py|js|ts|json|toml|yaml|yml|md|txt|html|css|jsx|tsx|vue|svelte))", FileOperationType::Replace),
        ];

        for (pattern, op_type) in file_op_patterns {
            if let Ok(re) = Regex::new(pattern) {
                for captures in re.captures_iter(response) {
                    if let Some(file_path) = captures.get(1) {
                        let path_str = file_path.as_str().trim();
                        
                        // Look for content in nearby code blocks or explicit content mentions
                        let content = self.extract_content_for_file(response, path_str);
                        let description = format!("{:?} {}", op_type, path_str);
                        
                        self.detected_file_operations.push(FileOperation {
                            operation_type: op_type.clone(),
                            file_path: path_str.to_string(),
                            content,
                            description,
                        });
                    }
                }
            }
        }
    }

    fn parse_code_blocks_with_file_paths(&mut self, response: &str) {
        use regex::Regex;
        
        // Pattern for code blocks with file paths as headers
        // Examples: ```rust src/main.rs or ```javascript config.js
        let code_block_pattern = r"```(?:\w+\s+)?([^\s\n]+\.(?:rs|py|js|ts|json|toml|yaml|yml|md|txt|html|css|jsx|tsx|vue|svelte))\n((?:(?!```)[\s\S])*?)```";
        
        if let Ok(re) = Regex::new(code_block_pattern) {
            for captures in re.captures_iter(response) {
                if let (Some(file_path), Some(content)) = (captures.get(1), captures.get(2)) {
                    let path_str = file_path.as_str().trim();
                    let content_str = content.as_str().trim();
                    
                    // Determine operation type based on context
                    let op_type = if self.file_exists_in_project(path_str) {
                        FileOperationType::Update
                    } else {
                        FileOperationType::Create
                    };
                    
                    let description = format!("Code block for {}", path_str);
                    
                    self.detected_file_operations.push(FileOperation {
                        operation_type: op_type,
                        file_path: path_str.to_string(),
                        content: content_str.to_string(),
                        description,
                    });
                }
            }
        }
    }

    fn parse_file_creation_statements(&mut self, response: &str) {
        use regex::Regex;
        
        // Pattern for statements like "Here's the content for file.rs:" followed by content
        let creation_pattern = r"(?i)(?:here'?s?\s+(?:the\s+)?(?:content|code)\s+for\s+)([^\s\n:]+\.(?:rs|py|js|ts|json|toml|yaml|yml|md|txt|html|css|jsx|tsx|vue|svelte)):?\s*\n((?:(?!```|Here's|Let me)[\s\S])*?)";
        
        if let Ok(re) = Regex::new(creation_pattern) {
            for captures in re.captures_iter(response) {
                if let (Some(file_path), Some(content)) = (captures.get(1), captures.get(2)) {
                    let path_str = file_path.as_str().trim();
                    let content_str = content.as_str().trim();
                    
                    if !content_str.is_empty() {
                        let op_type = if self.file_exists_in_project(path_str) {
                            FileOperationType::Update
                        } else {
                            FileOperationType::Create
                        };
                        
                        let description = format!("Content for {}", path_str);
                        
                        self.detected_file_operations.push(FileOperation {
                            operation_type: op_type,
                            file_path: path_str.to_string(),
                            content: content_str.to_string(),
                            description,
                        });
                    }
                }
            }
        }
    }

    fn extract_content_for_file(&self, response: &str, file_path: &str) -> String {
        use regex::Regex;
        
        // Look for code blocks near the file mention
        let escaped_path = regex::escape(file_path);
        let pattern = format!(r"(?s){0}.*?```(?:\w+)?\n(.*?)```", escaped_path);
        
        if let Ok(re) = Regex::new(&pattern) {
            if let Some(captures) = re.captures(response) {
                if let Some(content) = captures.get(1) {
                    return content.as_str().trim().to_string();
                }
            }
        }
        
        // If no code block found, return empty string
        String::new()
    }

    fn file_exists_in_project(&self, file_path: &str) -> bool {
        if let Some(ref project_path) = self.processor.project_path {
            let full_path = project_path.join(file_path);
            full_path.exists()
        } else {
            false
        }
    }
}

impl ClaudePanel {
    fn render_agentic_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("🤖 Agentic Mode").strong());
                ui.separator();

                // Agentic mode controls
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.agentic_mode_enabled, "Enable Agentic Mode");
                    ui.label("(Allow Claude to suggest file operations)");
                });

                if self.agentic_mode_enabled {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.auto_apply_changes, "Auto-apply changes");
                        ui.checkbox(&mut self.backup_enabled, "Create backups");
                    });

                    ui.separator();

                    // File operations section
                    if !self.detected_file_operations.is_empty() {
                        ui.label(RichText::new(format!("🔍 Detected {} file operation(s):", self.detected_file_operations.len())).strong());
                        
                        ScrollArea::vertical()
                            .max_height(200.0)
                            .id_salt("file_operations")
                            .show(ui, |ui| {
                                let mut operations_to_apply = Vec::new();
                                let mut operations_to_remove = Vec::new();
                                let mut operations_to_preview = Vec::new();

                                for (i, operation) in self.detected_file_operations.iter().enumerate() {
                                    ui.group(|ui| {
                                        ui.vertical(|ui| {
                                            // Operation header
                                            ui.horizontal(|ui| {
                                                let icon = match operation.operation_type {
                                                    FileOperationType::Create => "📄",
                                                    FileOperationType::Update => "✏️",
                                                    FileOperationType::Replace => "🔄",
                                                };
                                                
                                                let color = match operation.operation_type {
                                                    FileOperationType::Create => Color32::GREEN,
                                                    FileOperationType::Update => Color32::BLUE,
                                                    FileOperationType::Replace => Color32::YELLOW,
                                                };

                                                ui.label(RichText::new(format!("{} {:?}", icon, operation.operation_type)).color(color));
                                                ui.label(RichText::new(&operation.file_path).strong());
                                            });

                                            // Description
                                            ui.label(format!("📝 {}", operation.description));

                                            // Content preview (first few lines if content exists)
                                            if !operation.content.is_empty() {
                                                let preview_lines: Vec<&str> = operation.content.lines().take(3).collect();
                                                let preview_text = if operation.content.lines().count() > 3 {
                                                    format!("{}\n... ({} more lines)", preview_lines.join("\n"), operation.content.lines().count() - 3)
                                                } else {
                                                    preview_lines.join("\n")
                                                };

                                                ui.group(|ui| {
                                                    ui.vertical(|ui| {
                                                        ui.label("📋 Content Preview:");
                                                        ui.add(
                                                            TextEdit::multiline(&mut preview_text.clone())
                                                                .desired_rows(3)
                                                                .interactive(false)
                                                                .font(eframe::egui::TextStyle::Monospace)
                                                        );
                                                    });
                                                });
                                            }

                                            // Action buttons
                                            ui.horizontal(|ui| {
                                                if !operation.content.is_empty() {
                                                    let apply_button = Button::new("✅ Apply")
                                                        .fill(Color32::from_rgb(0, 100, 0));
                                                    
                                                    if ui.add(apply_button).clicked() {
                                                        operations_to_apply.push(i);
                                                    }
                                                }

                                                if ui.button("👁 Preview Full Content").clicked() {
                                                                                    // Store the operation data to avoid borrowing issues
                                                                                    let preview_path = operation.file_path.clone();
                                                                                    let preview_content = operation.content.clone();
                                                                                    
                                                                                    // Show content preview without borrowing self
                                                                                    operations_to_preview.push((preview_path, preview_content));
                                                }

                                                let dismiss_button = Button::new("❌ Dismiss")
                                                    .fill(Color32::from_rgb(100, 0, 0));
                                                
                                                if ui.add(dismiss_button).clicked() {
                                                    operations_to_remove.push(i);
                                                }
                                            });
                                        });
                                    });
                                    ui.add_space(5.0);
                                }

                                // Apply operations (in reverse order to maintain indices)
                                for &i in operations_to_apply.iter().rev() {
                                    if let Some(operation) = self.detected_file_operations.get(i) {
                                        self.apply_file_operation(operation.clone());
                                        self.detected_file_operations.remove(i);
                                    }
                                }

                                // Remove dismissed operations (in reverse order to maintain indices)
                                for &i in operations_to_remove.iter().rev() {
                                    self.detected_file_operations.remove(i);
                                }

                                // Handle preview operations
                                for (preview_path, preview_content) in operations_to_preview {
                                    self.show_content_preview_direct(&preview_path, &preview_content);
                                }
                            });

                        // Bulk actions
                        if !self.detected_file_operations.is_empty() {
                            ui.separator();
                            ui.horizontal(|ui| {
                                let apply_all_button = Button::new("✅ Apply All")
                                    .fill(Color32::from_rgb(0, 120, 0));
                                
                                if ui.add(apply_all_button).clicked() {
                                    let operations = self.detected_file_operations.clone();
                                    for operation in operations {
                                        self.apply_file_operation(operation);
                                    }
                                    self.detected_file_operations.clear();
                                }

                                let dismiss_all_button = Button::new("❌ Dismiss All")
                                    .fill(Color32::from_rgb(120, 0, 0));
                                
                                if ui.add(dismiss_all_button).clicked() {
                                    self.detected_file_operations.clear();
                                }
                            });
                        }
                    } else if self.agentic_mode_enabled {
                        ui.label("💡 No file operations detected in the last response.");
                        ui.label("Claude will suggest file operations when relevant.");
                    }
                } else {
                    ui.label("💡 Enable agentic mode to allow Claude to suggest file operations.");
                }
            });
        });
    }

    fn show_content_preview(&mut self, operation: &FileOperation) {
        // For now, we'll show the content in the response output
        // In a more advanced implementation, this could open a separate window
        self.response_output = format!(
            "📋 Content Preview for: {}\n\n{}\n\n{}",
            operation.file_path,
            "─".repeat(50),
            operation.content
        );
        self.success_message = Some(format!("Showing content preview for {}", operation.file_path));
    }

    fn show_content_preview_direct(&mut self, file_path: &str, content: &str) {
        // Show content preview directly from path and content parameters
        self.response_output = format!(
            "📋 Content Preview for: {}\n\n{}\n\n{}",
            file_path,
            "─".repeat(50),
            content
        );
        self.success_message = Some(format!("Showing content preview for {}", file_path));
    }

    fn apply_file_operation(&mut self, operation: FileOperation) {
        if let Some(ref project_path) = self.processor.project_path.clone() {
            let file_path = project_path.join(&operation.file_path);
            
            // Create backup if enabled and file exists
            if self.backup_enabled && file_path.exists() {
                if let Err(e) = self.create_backup(&file_path) {
                    self.error_message = Some(format!("Failed to create backup: {}", e));
                    return;
                }
            }

            // Create directory if it doesn't exist
            if let Some(parent) = file_path.parent() {
                if !parent.exists() {
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        self.error_message = Some(format!("Failed to create directory: {}", e));
                        return;
                    }
                }
            }

            // Write the file
            match std::fs::write(&file_path, &operation.content) {
                Ok(()) => {
                    let action = match operation.operation_type {
                        FileOperationType::Create => "created",
                        FileOperationType::Update => "updated", 
                        FileOperationType::Replace => "replaced",
                    };
                    
                    self.success_message = Some(format!(
                        "✅ Successfully {} file: {}",
                        action,
                        operation.file_path
                    ));
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!(
                        "Failed to write file {}: {}",
                        operation.file_path,
                        e
                    ));
                }
            }
        } else {
            self.error_message = Some("No project path set. Please set a project path first.".to_string());
        }
    }

    fn create_backup(&self, file_path: &std::path::Path) -> Result<(), std::io::Error> {
        let backup_path = file_path.with_extension(
            format!("{}.backup.{}", 
                file_path.extension().and_then(|s| s.to_str()).unwrap_or(""),
                chrono::Utc::now().format("%Y%m%d_%H%M%S")
            )
        );
        
        std::fs::copy(file_path, backup_path)?;
        Ok(())
    }
}

impl UiPanel for ClaudePanel {
    fn display(&mut self, _ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        // Check for streaming responses
        self.check_streaming_response();
        
        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.heading("🤖 Claude AI Helper");
                ui.separator();

                // Render messages first
                self.render_messages(ui);

                ui.add_space(10.0);

                // API Configuration
                self.render_api_key_section(ui);
                ui.add_space(10.0);

                // Project Configuration
                self.render_project_section(ui);
                ui.add_space(10.0);

                // File Attachments
                self.render_files_section(ui);
                ui.add_space(10.0);

                // Chat Interface
                self.render_chat_section(ui);
                ui.add_space(10.0);

                // Agentic Mode Section
                self.render_agentic_section(ui);
            });
        });
    }
}

// Clipboard helper function
#[cfg(not(target_arch = "wasm32"))]
fn save_to_clipboard(text: String) -> Result<(), Box<dyn std::error::Error>> {
    use arboard::Clipboard;
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn save_to_clipboard(_text: String) -> Result<(), Box<dyn std::error::Error>> {
    // WASM clipboard support would require web_sys
    Err("Clipboard not supported on WASM".into())
}

