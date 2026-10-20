//! Support and project links (shown in Help ▸, the About dialog, the start screen and the
//! header support action).

pub const APP_NAME: &str = "EffectAdobado";
pub const APP_SLUG: &str = "EffectAdobado";
pub const SUPPORT: &str = "https://github.com/smouj/EffectAdobado/issues";
/// Compatibility alias for the former community-link constant.
pub const DISCORD: &str = SUPPORT;
pub const WEBSITE: &str = "https://smouj.github.io/Adobado_Kit/";
pub const APP_PAGE: &str = "https://github.com/smouj/EffectAdobado";
pub const GITHUB: &str = "https://github.com/smouj/EffectAdobado";
pub const ISSUES: &str = "https://github.com/smouj/EffectAdobado/issues";

/// Sibling apps of Adobado Kit: (repository name, app name).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("PhotoAdobado", "PhotoAdobado"),
    ("ArtAdobado", "ArtAdobado"),
    ("FilmAdobado", "FilmAdobado"),
    ("LightAdobado", "LightAdobado"),
    ("SoundAdobado", "SoundAdobado"),
    ("DesignAdobado", "DesignAdobado"),
    ("PdfAdobado", "PdfAdobado"),
];

pub fn github_of(repository: &str) -> String {
    format!("https://github.com/smouj/{repository}")
}
pub fn page_of(repository: &str) -> String {
    github_of(repository)
}
