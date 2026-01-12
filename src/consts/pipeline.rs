use once_cell::sync::Lazy;
use std::path::PathBuf;

// initialize after first access
// would work under different OSes
pub static MALEXP_CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from(crate::consts::path::PROJECT_ROOT).join("external/malexp/common/argument_config")
});
pub const MALEXP_ANALYZE_CONFIG: &'static str = "analyze.ini";
pub const MALEXP_ENCODE_CONFIG: &'static str = "encode.ini";
pub const MALEXP_EXPAND_CONFIG: &'static str = "expand.ini";
pub const MALEXP_AUGMENT_CONFIG: &'static str = "augment.ini";
pub const MALEXP_DETECT_CONFIG: &'static str = "detect.ini";
pub const MALEXP_EXPLAIN_CONFIG: &'static str = "explain.ini";
