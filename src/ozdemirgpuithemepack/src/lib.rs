//! OzdemirGPUIThemePack — Fluent UI theme pack for gpui-kit.
//!
//! The token source of truth is the [`fluentui`] module (Rust-first):
//! Fluent 2 design tokens, typography and font resolution, and the bridge
//! into gpui-kit's `ThemeConfig`. The generated theme sheets are CSS
//! custom-property files embedded in the crate (Java-style resources) and
//! exposed by the [`resources`] module; the [`css`] module parses them —
//! embedded or from disk — back into gpui-kit theme configurations.

pub mod css;
pub mod fluentui;
pub mod resources;
