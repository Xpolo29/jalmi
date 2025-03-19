use iced::{Color, Theme};
/// Represents the application's color scheme
pub struct AppColors {
    pub top: Color,
    pub bottom: Color,
    pub bubble_left: Color,
    pub bubble_right: Color,
    pub model: Color,
    pub text_input: Color,
    pub font_color: Color,
    pub background: Color,
    pub border: Color,
    pub error: Color,
}

impl AppColors {
    /// Creates the default color scheme
    pub fn default() -> Self {
        Self {
            top: Color::from_rgb(0.25, 0.35, 0.55),       // Deep blue-gray for top area
            bottom: Color::from_rgb(0.25, 0.35, 0.55),       // Deep blue-gray for top area
            bubble_left: Color::from_rgb(0.81, 0.99, 0.74), // Very Pale Green
            bubble_right: Color::from_rgb(0.961, 0.961, 0.863), // Beige
            model: Color::from_rgb(1.0, 1.0, 1.0),   // Pure white for text input background
            text_input: Color::from_rgb(1.0, 1.0, 1.0),   // Pure white for text input background
            font_color: Color::from_rgb(0.15, 0.15, 0.2), // Dark navy for text
            background: Color::from_rgb(0.3, 0.3, 0.3), // Medium Gray (#717171), 
            error: Color::from_rgb(1.0, 0.4, 0.8),        // Flashy pink (to identify when used incorrectly)
            border: Color::from_rgb(0.3, 0.3, 0.4),       // Dark slate gray for borders
        }
    }
}

/// Font-related configurations
pub struct AppFonts {
    pub font_size: f32,
    pub header_size: f32,
    pub small_size: f32,
}

impl AppFonts {
    /// Creates the default font configuration
    pub fn default() -> Self {
        Self {
            font_size: 16.0,
            header_size: 24.0,
            small_size: 12.0,
        }
    }
}

/// Spacing and sizing configurations
pub struct AppSpacing {
    pub padding: f32,
    pub spacing: f32,
    pub border_radius: f32,
}

impl AppSpacing {
    /// Creates the default spacing configuration
    pub fn default() -> Self {
        Self {
            padding: 10.0,
            spacing: 8.0,
            border_radius: 4.0,
        }
    }
}

/// The complete application theme
pub struct AppTheme {
    pub colors: AppColors,
    pub fonts: AppFonts,
    pub spacing: AppSpacing,
}

impl AppTheme {
    /// Creates the default application theme
    pub fn default() -> Self {
        Self {
            colors: AppColors::default(),
            fonts: AppFonts::default(),
            spacing: AppSpacing::default(),
        }
    }
}

/// Global access to the current theme
pub fn current_theme() -> AppTheme {
    AppTheme::default()
}

/// Converts our application theme to an Iced theme
pub fn get_default_theme() -> Theme {
    let colors = AppColors::default();
    
    // Convert to Iced palette
    let palette = iced::theme::Palette {
        background: colors.background,
        text: colors.font_color,
        primary: colors.bubble_right,
        success: colors.model,
        danger: colors.error,
    };

    Theme::custom("AppTheme".to_string(), palette)
}

/// Helper functions for quick access to theme properties

pub fn top() -> Color {
    current_theme().colors.top
}

pub fn bottom() -> Color {
    current_theme().colors.bottom
}

pub fn bubble_left() -> Color {
    current_theme().colors.bubble_left
}

pub fn bubble_right() -> Color {
    current_theme().colors.bubble_right
}

pub fn model() -> Color {
    current_theme().colors.model
}

pub fn text_input() -> Color {
    current_theme().colors.text_input
}

pub fn font_color() -> Color {
    current_theme().colors.font_color
}

pub fn background() -> Color {
    current_theme().colors.background
}

pub fn error() -> Color {
    current_theme().colors.error
}

pub fn font_size() -> f32 {
    current_theme().fonts.font_size
}

pub fn default_padding() -> f32 {
    current_theme().spacing.padding
}

pub fn default_spacing() -> f32 {
    current_theme().spacing.spacing
}

pub fn border() -> Color {
    current_theme().colors.border
}