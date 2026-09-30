//! Generates the Fluent theme files (`themes/*.css`) from the Rust token module.
//!
//! Usage: `cargo run -p ozdemirgpuithemepack --bin GenThemes [-- target-dir]`
//! The default target directory is the `themes/` folder of the working directory.

use std::path::PathBuf;

use ozdemirgpuithemepack::fluentui::theme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("themes"));

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
