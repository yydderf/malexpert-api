use once_cell::sync::Lazy;

// runtime adjustable
pub static BASE_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("MALEXP_BASE_URL")
        .unwrap_or_else(|_| "http://10.0.1.3:12344".to_string())
});

pub const CATALOG: &'static str = "/catalog";
pub const RUN: &'static str = "/run";
