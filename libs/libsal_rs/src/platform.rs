//! Platform-specific utilities.
//!
//! Centralises OS detection, user-directory resolution, and any
//! runtime feature-gating (e.g. AMD vs Intel SIMD paths).

/// Returns the canonical user documents directory.
pub fn documents_dir() -> std::path::PathBuf {
    dirs_fallback()
}

/// Returns the application configuration directory.
pub fn config_dir(app_name: &str) -> std::path::PathBuf {
    let mut base = dirs_fallback();
    base.push(app_name);
    base
}

fn dirs_fallback() -> std::path::PathBuf {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("C:\\Users\\Public\\Documents"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME")
            .map(|h| std::path::PathBuf::from(h).join("Documents"))
            .unwrap_or_else(|_| std::path::PathBuf::from("/tmp"))
    }
}
