use eframe::egui::{Align, Button, Color32, ComboBox, Layout, Resize, ScrollArea, TextEdit, Ui};

use crate::{
    context::FrameCtx,
    core::crypto::{
        symmetric::aes::AesKeySize, CipherMode, CryptoAlgorithm, CryptoOperation, OutputEncoding,
        RsaKeySize,
    },
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
            ComboBox::from_id_salt("crypto_algorithm")
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
                CryptoAlgorithm::RSA => {
                    ComboBox::from_id_salt("crypto_operation")
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
                    ComboBox::from_id_salt("crypto_operation")
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
        if ctx.app.crypto.input.algorithm.is_symmetric() {
            ui.horizontal(|ui| {
                ui.label("Mode:");
                let current_mode = ctx.app.crypto.input.mode.unwrap_or(CipherMode::CBC);
                ComboBox::from_id_salt("crypto_mode")
                    .selected_text(current_mode.to_string())
                    .show_ui(ui, |ui| {
                        for mode in CipherMode::variants() {
                            if ui
                                .selectable_value(
                                    &mut ctx.app.crypto.input.mode,
                                    Some(*mode),
                                    mode.to_string(),
                                )
                                .clicked()
                            {
                                // Clear IV when switching to ECB mode
                                if *mode == CipherMode::ECB {
                                    ctx.app.crypto.input.iv = None;
                                }
                            }
                        }
                    });

                // Add AES key size selection for AES algorithm
                if matches!(ctx.app.crypto.input.algorithm, CryptoAlgorithm::AES) {
                    ui.add_space(SPACE);
                    ui.label("Key Size:");
                    let current_key_size = ctx
                        .app
                        .crypto
                        .input
                        .aes_key_size
                        .unwrap_or(AesKeySize::Aes256);
                    ComboBox::from_id_salt("aes_key_size")
                        .selected_text(current_key_size.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.aes_key_size,
                                Some(AesKeySize::Aes128),
                                AesKeySize::Aes128.to_string(),
                            );
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.aes_key_size,
                                Some(AesKeySize::Aes192),
                                AesKeySize::Aes192.to_string(),
                            );
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.aes_key_size,
                                Some(AesKeySize::Aes256),
                                AesKeySize::Aes256.to_string(),
                            );
                        });
                }
            });
        } else if matches!(ctx.app.crypto.input.algorithm, CryptoAlgorithm::RSA) {
            // Add RSA key size selection and cipher type display
            ui.horizontal(|ui| {
                ui.label("Cipher Type:");
                ui.label("RSA/ECB/PKCS1");
                
                ui.add_space(SPACE);
                ui.label("Key Size:");
                let current_key_size = ctx
                    .app
                    .crypto
                    .input
                    .rsa_key_size
                    .unwrap_or(RsaKeySize::Rsa2048);
                ComboBox::from_id_salt("rsa_key_size")
                    .selected_text(current_key_size.to_string())
                    .show_ui(ui, |ui| {
                        for key_size in RsaKeySize::variants() {
                            ui.selectable_value(
                                &mut ctx.app.crypto.input.rsa_key_size,
                                Some(*key_size),
                                key_size.to_string(),
                            );
                        }
                    });
            });
        }
    }

    fn render_encoding_selection(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        // Only show encoding selection for encrypt and sign operations
        if matches!(
            ctx.app.crypto.input.operation,
            CryptoOperation::Encrypt | CryptoOperation::Sign
        ) && matches!(
            ctx.app.crypto.input.algorithm,
            CryptoAlgorithm::AES
                | CryptoAlgorithm::DES
                | CryptoAlgorithm::TripleDES
                | CryptoAlgorithm::RSA
        ) {
            ui.horizontal(|ui| {
                ui.label("Output Format:");
                ui.add_space(SPACE);

                for encoding in OutputEncoding::variants() {
                    ui.radio_value(
                        &mut ctx.app.crypto.input.encoding,
                        *encoding,
                        encoding.to_string(),
                    );
                }
            });
        }
    }

    fn render_key_inputs(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        if ctx.app.crypto.input.algorithm.is_symmetric() {
            ui.horizontal(|ui| {
                ui.label("Key :");
                ui.add_space(HALF_SPACE);
                if ui.button("Generate").clicked() {
                    if let Err(e) = ctx.app.crypto.generate_random_key() {
                        ctx.app.crypto.error = Some(format!("Key generation failed: {e}"))
                    }
                }
            });

            let key_hint = match ctx.app.crypto.input.algorithm {
                CryptoAlgorithm::AES => match ctx.app.crypto.input.aes_key_size {
                    Some(size) => match size {
                        AesKeySize::Aes128 => "16 bytes characters",
                        AesKeySize::Aes192 => "24 bytes characters",
                        AesKeySize::Aes256 => "32 bytes characters",
                    },
                    None => "Select key size first",
                },
                CryptoAlgorithm::DES => "8 bytes characters",
                CryptoAlgorithm::TripleDES => "24 bytes characters",
                _ => "",
            };

            Resize::default()
                .id_salt("key_input")
                .default_width(200.0)
                .max_height(SPACE)
                .max_width(200.0)
                .show(ui, |ui| {
                    ScrollArea::horizontal()
                        .id_salt("key_input_scroll")
                        .max_height(ui.available_height() * 0.1)
                        .show(ui, |ui| {
                            ui.add(
                                TextEdit::multiline(&mut ctx.app.crypto.input.key)
                                    .hint_text(key_hint),
                            );
                        });
                });

            // IV for CBC mode
            if ctx.app.crypto.input.mode == Some(CipherMode::CBC) {
                ui.add_space(HALF_SPACE);
                ui.horizontal(|ui| {
                    ui.label("IV (hex):");
                    ui.add_space(HALF_SPACE);
                    if ui.button("Generate").clicked() {
                        if let Err(e) = ctx.app.crypto.generate_random_iv() {
                            ctx.app.crypto.error = Some(format!("IV generation failed: {e}"))
                        }
                    }
                });

                let iv_hint = match ctx.app.crypto.input.algorithm {
                    CryptoAlgorithm::AES => "16 bytes",
                    CryptoAlgorithm::DES | CryptoAlgorithm::TripleDES => "8 bytes",
                    _ => "",
                };

                Resize::default()
                    .id_salt("iv_input")
                    .default_width(200.0)
                    .max_height(SPACE)
                    .max_width(200.0)
                    .show(ui, |ui| {
                        ScrollArea::horizontal()
                            .id_salt("iv_input_scroll")
                            .max_height(ui.available_height() * 0.1)
                            .show(ui, |ui| {
                                let mut iv = ctx.app.crypto.input.iv.clone().unwrap_or_default();
                                ui.add(TextEdit::multiline(&mut iv).hint_text(iv_hint));
                                ctx.app.crypto.input.iv =
                                    if iv.is_empty() { None } else { Some(iv) };
                            });
                    });
            }
        } else {
            // Asymmetric key inputs
            ui.horizontal(|ui| {
                if ui.button("Generate Keypair").clicked() {
                    if let Err(e) = ctx.app.crypto.generate_random_key() {
                        ctx.app.crypto.error = Some(format!("Keypair generation failed: {}", e));
                    }
                }
            });

            let mut public_key = ctx.app.crypto.input.public_key.clone().unwrap_or_default();
            let hint = match ctx.app.crypto.input.algorithm {
                CryptoAlgorithm::RSA => "PEM format RSA public key",
                _ => "",
            };

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Public Key:");
                    ui.add_space(HALF_SPACE);

                    Resize::default()
                        .id_salt("public_key_input")
                        .default_width(300.0)
                        .max_height(300.0)
                        .show(ui, |ui| {
                            ScrollArea::vertical()
                                .id_salt("public_key_input_scroll")
                                .stick_to_bottom(false)
                                .drag_to_scroll(false)
                                .show(ui, |ui| {
                                    ui.with_layout(
                                        Layout::top_down(Align::Min)
                                            .with_main_justify(true)
                                            .with_cross_justify(true),
                                        |ui| {
                                            ui.add(
                                                TextEdit::multiline(&mut public_key)
                                                    .hint_text(hint),
                                            );
                                        },
                                    );
                                });
                        });
                });

                ctx.app.crypto.input.public_key = if public_key.is_empty() {
                    None
                } else {
                    Some(public_key)
                };

                let mut private_key = ctx.app.crypto.input.private_key.clone().unwrap_or_default();
                let hint = match ctx.app.crypto.input.algorithm {
                    CryptoAlgorithm::RSA => "PEM format RSA private key",
                    _ => "",
                };

                ui.vertical(|ui| {
                    ui.label("Private Key:");
                    ui.add_space(HALF_SPACE);

                    Resize::default()
                        .id_salt("private_key_input")
                        .default_width(300.0)
                        .max_height(300.0)
                        .show(ui, |ui| {
                            ScrollArea::vertical()
                                .id_salt("private_key_input_scroll")
                                .stick_to_bottom(false)
                                .drag_to_scroll(false)
                                .show(ui, |ui| {
                                    ui.with_layout(
                                        Layout::top_down(Align::Min)
                                            .with_main_justify(true)
                                            .with_cross_justify(true),
                                        |ui| {
                                            ui.add(
                                                TextEdit::multiline(&mut private_key)
                                                    .hint_text(hint),
                                            );
                                        },
                                    );
                                });
                        });
                });

                ctx.app.crypto.input.private_key = if private_key.is_empty() {
                    None
                } else {
                    Some(private_key)
                };

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
                    ctx.app.crypto.input.signature = if signature.is_empty() {
                        None
                    } else {
                        Some(signature)
                    };
                }
            });
        }
    }

    fn render_process_button(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.add_space(SPACE);
        ui.horizontal(|ui| {
            if ui
                .add(Button::new("Process").min_size([120.0, 30.0].into()))
                .clicked()
            {
                ctx.app.crypto.clear_output();
                if let Err(e) = ctx.app.crypto.process() {
                    // Error is already set in the processor
                    eprintln!("Cryptography processing error: {}", e);
                }
            }

            ui.add_space(SPACE);

            if ui
                .add(Button::new("Clear").min_size([80.0, 30.0].into()))
                .clicked()
            {
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
                    ui.add_space(HALF_SPACE);

                    self.render_encoding_selection(ctx, ui);
                    ui.add_space(SPACE);

                    // Key inputs
                    self.render_key_inputs(ctx, ui);
                    ui.add_space(SPACE);

                    // Input/Output
                    let input_label = match ctx.app.crypto.input.operation {
                        CryptoOperation::Encrypt => "Plaintext",
                        //TODO: select input format for decryption
                        CryptoOperation::Decrypt => "Ciphertext (hex or base64)",
                        CryptoOperation::Sign => "Message to sign",
                        CryptoOperation::Verify => "Original message",
                    };

                    let output_label = match ctx.app.crypto.input.operation {
                        CryptoOperation::Encrypt => match ctx.app.crypto.input.algorithm {
                            CryptoAlgorithm::AES
                            | CryptoAlgorithm::DES
                            | CryptoAlgorithm::TripleDES => match ctx.app.crypto.input.encoding {
                                OutputEncoding::Hex => "Ciphertext (hex)",
                                OutputEncoding::Base64 => "Ciphertext (base64)",
                            },
                            _ => "Ciphertext",
                        },
                        CryptoOperation::Decrypt => "Plaintext",
                        CryptoOperation::Sign => match ctx.app.crypto.input.encoding {
                            OutputEncoding::Hex => "Signature (hex)",
                            OutputEncoding::Base64 => "Signature (base64)",
                        },
                        CryptoOperation::Verify => "Verification result",
                    };

                    let input_hint = match ctx.app.crypto.input.operation {
                        CryptoOperation::Decrypt => "Enter ciphertext (hex or base64 format)",
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
