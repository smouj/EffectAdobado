//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "EffectAdobado";
pub const APP_SLUG: &str = "effectcraft";
pub const DISCORD: &str = "https://github.com/smouj/Adobado_Kit";
pub const WEBSITE: &str = "https://github.com/smouj/Adobado_Kit";
pub const APP_PAGE: &str = "https://github.com/smouj/EffectAdobado";
pub const GITHUB: &str = "https://github.com/smouj/EffectAdobado";
pub const ISSUES: &str = "https://github.com/smouj/EffectAdobado/issues";

/// Sibling apps of the Adobado Kit family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("PhotoCraft", "photocraft"),
    ("VectorCraft", "vectorcraft"),
    ("FilmCraft", "filmcraft"),
    ("LightCraft", "lightcraft"),
    ("PdfCraft", "pdfcraft"),
    ("DesignCraft", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://github.com/smouj/Adobado_Kit/apps/{slug}")
}
