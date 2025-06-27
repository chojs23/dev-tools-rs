use eframe::egui::{ComboBox, ScrollArea, TextEdit, Ui, Color32, Button};

use crate::{
    context::FrameCtx,
    core::crypto::{CryptoAlgorithm, CryptoOperation, CipherMode},
    ui::{
        components::{input_output_box::InputOutputBox, DOUBLE_SPACE, HALF_SPACE, SPACE},
        traits::UiPanel,
    },
};

#[derive(Debug, Default)]
pub struct CryptographyPanel;

impl CryptographyPanel {
    pub fn new() -> Self {
        Self::default()
    }

    fn render_algorithm_selection(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Algorithm:");
            ComboBox::from_id_source("crypto_algorithm")
                .selected_text(ctx.app.crypto.input.algorithm.to_string())
                .show_ui(ui, |ui| {
                    for algorithm in CryptoAlgorithm::variants() {
                        ui.selectable_value(
                            &mut ctx.app.crypto.input.algorithm,
                            *algorithm,
                            algorithm.to_string(),
                        );
                    }
                });
        });
    }

    fn render_operation_selection(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Operation:");
            match ctx.app.crypto.input.algorithm {
                CryptoAlgorithm::ECDSA => {
                    ComboBox::from_id_source("crypto_operation")
                        .selected_text(ctx.app.crypto.input.operation.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.operation,
                                CryptoOperation::Sign,
                                CryptoOperation::Sign.to_string(),
                            );
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.operation,
                                CryptoOperation::Verify,
                                CryptoOperation::Verify.to_string(),
                            );
                        });
                }
                CryptoAlgorithm::RSA => {
                    ComboBox::from_id_source("crypto_operation")
                        .selected_text(ctx.app.crypto.input.operation.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.operation,
                                CryptoOperation::Encrypt,
                                CryptoOperation::Encrypt.to_string(),
                            );
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.operation,
                                CryptoOperation::Decrypt,
                                CryptoOperation::Decrypt.to_string(),
                            );
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.operation,
                                CryptoOperation::Sign,
                                CryptoOperation::Sign.to_string(),
                            );
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.operation,
                                CryptoOperation::Verify,
                                CryptoOperation::Verify.to_string(),
                            );
                        });
                }
                _ => {
                    ComboBox::from_id_source("crypto_operation")
                        .selected_text(ctx.app.crypto.input.operation.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.operation,
                                CryptoOperation::Encrypt,
                                CryptoOperation::Encrypt.to_string(),
                            );
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.operation,
                                CryptoOperation::Decrypt,
                                CryptoOperation::Decrypt.to_string(),
                            );
                        });
                }
            }
        });
    }

    fn render_mode_selection(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        if ctx.app.crypto.input.algorithm.is_symmetric() 
            && !matches!(ctx.app.crypto.input.algorithm, CryptoAlgorithm::RC4) 
        {
            ui.horizontal(|ui| {
                ui.label("Mode:");
                let current_mode = ctx.app.crypto.input.mode.unwrap_or(CipherMode::CBC);
                ComboBox::from_id_source("crypto_mode")
                    .selected_text(current_mode.to_string())
                    .show_ui(ui, |ui| {
                        for mode in CipherMode::variants() {
                            if ui.selectable_value(
                                &mut ctx.app.crypto.input.mode,
                                Some(*mode),
                                mode.to_string(),
                            ).clicked() {
                                // Clear IV when switching to ECB mode
                                if *mode == CipherMode::ECB {
                                    ctx.app.crypto.input.iv = None;
                                }
                            }
                        }
                    });
            });
        }
    }

    fn render_key_inputs(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        if ctx.app.crypto.input.algorithm.is_symmetric() {
            ui.horizontal(|ui| {
                ui.label("Key (hex):");
                ui.add_space(HALF_SPACE);
                if ui.button("Generate").clicked() {
                    if let Err(e) = ctx.app.crypto.generate_random_key() {
                        ctx.app.crypto.error = Some(format!("Key generation failed: {}", e));
                    }
                }
            });
            
            let key_hint = match ctx.app.crypto.input.algorithm {
                CryptoAlgorithm::AES => "32 hex characters (16 bytes)",
                CryptoAlgorithm::DES => "16 hex characters (8 bytes)",
                CryptoAlgorithm::TripleDES => "48 hex characters (24 bytes)",
                CryptoAlgorithm::RC4 => "1-512 hex characters (up to 256 bytes)",
                _ => "",
            };
            
            ui.add(
                TextEdit::multiline(&mut ctx.app.crypto.input.key)
                    .hint_text(key_hint)
                    .desired_width(ui.available_width()),
            );
            
            // IV for CBC mode
            if ctx.app.crypto.input.mode == Some(CipherMode::CBC) {
                ui.add_space(HALF_SPACE);
                ui.horizontal(|ui| {
                    ui.label("IV (hex):");
                    ui.add_space(HALF_SPACE);
                    if ui.button("Generate").clicked() {
                        if let Err(e) = ctx.app.crypto.generate_random_iv() {
                            ctx.app.crypto.error = Some(format!("IV generation failed: {}", e));
                        }
                    }
                });
                
                let iv_hint = match ctx.app.crypto.input.algorithm {
                    CryptoAlgorithm::AES => "32 hex characters (16 bytes)",
                    CryptoAlgorithm::DES | CryptoAlgorithm::TripleDES => "16 hex characters (8 bytes)",
                    _ => "",
                };
                
                let mut iv = ctx.app.crypto.input.iv.clone().unwrap_or_default();
                ui.add(
                    TextEdit::multiline(&mut iv)
                        .hint_text(iv_hint)
                        .desired_width(ui.available_width()),
                );
                ctx.app.crypto.input.iv = if iv.is_empty() { None } else { Some(iv) };
            }
        } else {
            // Asymmetric key inputs
            match ctx.app.crypto.input.operation {
                CryptoOperation::Encrypt | CryptoOperation::Verify => {
                    ui.horizontal(|ui| {
                        ui.label("Public Key:");
                        ui.add_space(HALF_SPACE);
                        if ui.button("Generate Keypair").clicked() {
                            if let Err(e) = ctx.app.crypto.generate_random_key() {
                                ctx.app.crypto.error = Some(format!("Keypair generation failed: {}", e));
                            }
                        }
                    });
                    
                    let mut public_key = ctx.app.crypto.input.public_key.clone().unwrap_or_default();
                    let hint = match ctx.app.crypto.input.algorithm {
                        CryptoAlgorithm::RSA => "PEM format RSA public key",
                        CryptoAlgorithm::ECDSA => "Hex encoded public key",
                        _ => "",
                    };
                    
                    ui.add(
                        TextEdit::multiline(&mut public_key)
                            .hint_text(hint)
                            .desired_rows(6)
                            .desired_width(ui.available_width()),
                    );
                    ctx.app.crypto.input.public_key = if public_key.is_empty() { None } else { Some(public_key) };
                }
                CryptoOperation::Decrypt | CryptoOperation::Sign => {
                    ui.horizontal(|ui| {
                        ui.label("Private Key:");
                        ui.add_space(HALF_SPACE);
                        if ui.button("Generate Keypair").clicked() {
                            if let Err(e) = ctx.app.crypto.generate_random_key() {
                                ctx.app.crypto.error = Some(format!("Keypair generation failed: {}", e));
                            }
                        }
                    });
                    
                    let mut private_key = ctx.app.crypto.input.private_key.clone().unwrap_or_default();
                    let hint = match ctx.app.crypto.input.algorithm {
                        CryptoAlgorithm::RSA => "PEM format RSA private key",
                        CryptoAlgorithm::ECDSA => "Hex encoded private key",
                        _ => "",
                    };
                    
                    ui.add(
                        TextEdit::multiline(&mut private_key)
                            .hint_text(hint)
                            .desired_rows(6)
                            .desired_width(ui.available_width()),
                    );
                    ctx.app.crypto.input.private_key = if private_key.is_empty() { None } else { Some(private_key) };
                }
            }
            
            // Signature input for verification
            if ctx.app.crypto.input.operation == CryptoOperation::Verify {
                ui.add_space(HALF_SPACE);
                ui.label("Signature (hex):");
                let mut signature = ctx.app.crypto.input.signature.clone().unwrap_or_default();
                ui.add(
                    TextEdit::multiline(&mut signature)
                        .hint_text("Hex encoded signature")
                        .desired_width(ui.available_width()),
                );
                ctx.app.crypto.input.signature = if signature.is_empty() { None } else { Some(signature) };
            }
        }
    }

    fn render_process_button(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.add_space(SPACE);
        ui.horizontal(|ui| {
            if ui.add(Button::new("Process").min_size([120.0, 30.0].into())).clicked() {
                ctx.app.crypto.clear_output();
                if let Err(e) = ctx.app.crypto.process() {
                    // Error is already set in the processor
                    eprintln!("Cryptography processing error: {}", e);
                }
            }
            
            ui.add_space(SPACE);
            
            if ui.add(Button::new("Clear").min_size([80.0, 30.0].into())).clicked() {
                ctx.app.crypto.input.input_text.clear();
                ctx.app.crypto.clear_output();
            }
        });
    }

    fn render_error_display(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        if let Some(error) = &ctx.app.crypto.error {
            ui.add_space(HALF_SPACE);
            ui.colored_label(Color32::RED, error);
        }
    }
}

impl UiPanel for CryptographyPanel {
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Algorithm and operation selection
                    self.render_algorithm_selection(ctx, ui);
                    ui.add_space(HALF_SPACE);
                    
                    self.render_operation_selection(ctx, ui);
                    ui.add_space(HALF_SPACE);
                    
                    self.render_mode_selection(ctx, ui);
                    ui.add_space(SPACE);
                    
                    // Key inputs
                    self.render_key_inputs(ctx, ui);
                    ui.add_space(SPACE);
                    
                    // Input/Output
                    let input_label = match ctx.app.crypto.input.operation {
                        CryptoOperation::Encrypt => "Plaintext",
                        CryptoOperation::Decrypt => "Ciphertext (hex)",
                        CryptoOperation::Sign => "Message to sign",
                        CryptoOperation::Verify => "Original message",
                    };
                    
                    let output_label = match ctx.app.crypto.input.operation {
                        CryptoOperation::Encrypt => "Ciphertext (hex)",
                        CryptoOperation::Decrypt => "Plaintext",
                        CryptoOperation::Sign => "Signature (hex)",
                        CryptoOperation::Verify => "Verification result",
                    };
                    
                    let input_hint = match ctx.app.crypto.input.operation {
                        CryptoOperation::Decrypt => "Enter hex-encoded ciphertext",
                        CryptoOperation::Sign => "Enter message to sign",
                        CryptoOperation::Verify => "Enter original message that was signed",
                        _ => "Enter text to encrypt",
                    };
                    
                    InputOutputBox::new(input_label, output_label)
                        .with_input_hint(input_hint)
                        .render(
                            &mut ctx.app.crypto.input.input_text,
                            &mut ctx.app.crypto.output,
                            ui,
                        );
                    
                    self.render_process_button(ctx, ui);
                    self.render_error_display(ctx, ui);
                    
                    ui.add_space(DOUBLE_SPACE);
                });
            });
    }
}

