use gpui_kit::{App, SharedString};

/// Font families preferred by the Fluent design, in priority order.
///
/// gpui resolves families live from the OS; when none is installed the
/// theme default applies.
pub const FLUENT_FONT_FAMILIES: [&str; 3] = [
    "Segoe UI Variable Text",
    "Segoe UI Variable",
    "Segoe UI",
];

/// Finds the first installed family in the priority list (pure, testable).
pub fn first_installed<'a>(priority: &[&'a str], installed: &[String]) -> Option<&'a str> {
    priority
        .iter()
        .copied()
        .find(|family| installed.iter().any(|name| name == family))
}

/// Resolves the Fluent font family within the application context.
pub fn resolve_fluent_font(cx: &App) -> Option<SharedString> {
    let installed = cx.text_system().all_font_names();
    first_installed(&FLUENT_FONT_FAMILIES, &installed).map(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| v.to_string()).collect()
    }

    #[test]
    fn picks_first_installed_in_priority_order() {
        let installed = names(&["Noto Sans", "Segoe UI"]);
        assert_eq!(
            first_installed(&FLUENT_FONT_FAMILIES, &installed),
            Some("Segoe UI")
        );

        let installed = names(&["Segoe UI Variable", "Segoe UI"]);
        assert_eq!(
            first_installed(&FLUENT_FONT_FAMILIES, &installed),
            Some("Segoe UI Variable")
        );
    }

    #[test]
    fn none_installed_yields_none() {
        let installed = names(&["Noto Sans", "DejaVu Sans"]);
        assert_eq!(first_installed(&FLUENT_FONT_FAMILIES, &installed), None);
    }
}
