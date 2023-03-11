fn main() {
    // Try to find oniguruma using pkg-config first
    match pkg_config::Config::new().probe("oniguruma") {
        Ok(library) => {
            for include_path in library.include_paths {
                println!("cargo:include={}", include_path.to_string_lossy());
            }
        }
        // If pkg-config fails, leave it to the onig build script
        // See https://crates.io/crates/onig
        Err(_) => {}
    }
}