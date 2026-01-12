use once_cell::sync::Lazy;

pub const TIMEOUT_SECS: u64 = 10;

// runtime adjustable
pub static MALEXP_API_BASE_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("MALEXP_API_BASE_URL")
        .unwrap_or_else(|_| "http://10.0.1.3:12344".to_string())
});

pub const MALEXP_API_CATALOG: &'static str = "/catalog";
