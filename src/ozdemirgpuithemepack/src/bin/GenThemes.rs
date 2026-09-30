//! Generates the Fluent theme files from the Rust token module into the
//! crate's embedded resources folder.
//!
//! Usage: `cargo run -p ozdemirgpuithemepack --bin GenThemes [-- target-dir]`
//! The default target directory is `src/ozdemirgpuithemepack/resources/themes`
//! (the crate's Java-style resources folder, embedded via `include_str!`).

use std::path::PathBuf;

use ozdemirgpuithemepack::fluentui::theme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/themes")
        });

    std::fs::create_dir_all(&out_dir)?;

    let mut count = 0;
    for variant in theme::variants() {
        let path = out_dir.join(format!(
            "{}.css",
            theme::theme_file_stem(variant.accent, variant.dark)
        ));
        std::fs::write(&path, theme::theme_css(variant.accent, variant.dark))?;
        println!("wrote: {}", path.display());
        count += 1;
    }

    println!("wrote {count} Fluent themes to {}", out_dir.display());
    Ok(())
}
