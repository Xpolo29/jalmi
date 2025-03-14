use iced::{Color, Theme};

/// Represents the application's color scheme
pub struct AppColors {
    pub background: Color,
    pub text: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub error: Color,
}

impl AppColors {
    /// Creates the default color scheme
    pub fn default() -> Self {
        Self {
            background: Color::from_rgb(0.95, 0.95, 0.95),
            text: Color::from_rgb(0.2, 0.2, 0.2),
            primary: Color::from_rgb(0.4, 0.6, 0.8),
            secondary: Color::from_rgb(0.3, 0.7, 0.5),
            accent: Color::from_rgb(0.8, 0.5, 0.3),
            error: Color::from_rgb(0.8, 0.3, 0.3),
        }
    }
}

/// Font-related configurations
pub struct AppFonts {
    pub default_size: f32,
    pub header_size: f32,
    pub small_size: f32,
}

impl AppFonts {
    /// Creates the default font configuration
    pub fn default() -> Self {
        Self {
            default_size: 16.0,
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
        text: colors.text,
        primary: colors.primary,
        success: colors.secondary,
        danger: colors.error,
    };

    Theme::custom("AppTheme".to_string(), palette)
}

/// Helper functions for quick access to theme properties

pub fn background() -> Color {
    current_theme().colors.background
}

pub fn text() -> Color {
    current_theme().colors.text
}

pub fn primary() -> Color {
    current_theme().colors.primary
}

pub fn secondary() -> Color {
    current_theme().colors.secondary
}

pub fn accent() -> Color {
    current_theme().colors.accent
}

pub fn error() -> Color {
    current_theme().colors.error
}

pub fn default_padding() -> f32 {
    current_theme().spacing.padding
}

pub fn default_spacing() -> f32 {
    current_theme().spacing.spacing
}

pub fn default_font_size() -> f32 {
    current_theme().fonts.default_size
}
