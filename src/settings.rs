use std::io::Result;

use crate::{backend::default_builder, ColorPalette};
use iced::Font;

#[cfg(target_os = "windows")]
const DEFAULT_SHELL: &str = "wsl.exe";

#[cfg(not(target_os = "windows"))]
const DEFAULT_SHELL: &str = "/bin/bash";

#[derive(Clone)]
pub struct Settings<B: BackendBuilder> {
    pub font: FontSettings,
    pub theme: ThemeSettings,
    pub backend: BackendSettings<B>,
}

impl<B> Settings<B>
where
    B: BackendBuilder,
{
    pub fn default() -> Settings<impl BackendBuilder> {
        Settings {
            font: FontSettings::default(),
            theme: ThemeSettings::default(),
            backend: BackendSettings {
                backend_builder: default_builder(DEFAULT_SHELL.to_string()),
            },
        }
    }
}

pub trait BackendBuilder {
    type Backend: alacritty_terminal::tty::EventedPty
        + alacritty_terminal::event::OnResize
        + Send
        + 'static;

    fn build(
        &self,
        id: u64,
        size: iced_core::Size<f32>,
    ) -> Result<Self::Backend>;
}

impl<B, T> BackendBuilder for B
where
    B: Fn(u64, iced_core::Size<f32>) -> Result<T>,
    T: alacritty_terminal::tty::EventedPty
        + alacritty_terminal::event::OnResize
        + Send
        + 'static,
{
    type Backend = T;

    fn build(
        &self,
        id: u64,
        size: iced_core::Size<f32>,
    ) -> Result<Self::Backend> {
        self(id, size)
    }
}

#[derive(Debug, Clone)]
pub struct BackendSettings<B> {
    pub backend_builder: B,
}

#[derive(Debug, Clone)]
pub struct FontSettings {
    pub size: f32,
    pub scale_factor: f32,
    pub font_type: Font,
}

impl Default for FontSettings {
    fn default() -> Self {
        Self {
            size: 14.0,
            scale_factor: 1.3,
            font_type: Font::MONOSPACE,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ThemeSettings {
    pub color_pallete: Box<ColorPalette>,
}

impl ThemeSettings {
    pub fn new(color_pallete: Box<ColorPalette>) -> Self {
        Self { color_pallete }
    }
}
