//! OzdemirGPUIThemePack gallery — an AtlantaFX-sampler-style showcase.
//!
//! The gallery is the theme pack's product: a left sidebar navigates between
//! pages that exercise every gpui-kit control under the active Fluent
//! variant, and the title bar switches accent/darkness live.

mod app;
mod layout;
mod pages;

use app::{APP_NAME, GalleryApp};
use gpui_kit::component::{Root, TitleBar};
use gpui_kit::{
    AnyView, AppContext as _, Bounds, Entity, WindowBounds, WindowDecorations, size, px,
};
use ozdemirgpuithemepack::fluentui::Accent;
use ozdemirgpuithemepack::fluentui::theme as fluent_theme;

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::AllAssets)
        .run(|cx| {
            gpui_kit::init(cx);

            fluent_theme::apply(Accent::Blue, false, None, cx);

            let bounds = Bounds::centered(None, size(px(1200.), px(760.)), cx);
            let mut options = TitleBar::window_options();
            options.window_bounds = Some(WindowBounds::Windowed(bounds));
            // Use the gpui-kit TitleBar's client-side decorations instead of the
            // system (WM) title bar (X11: _MOTIF_WM_HINTS; without a compositor
            // gpui falls back to server-side decorations).
            options.window_decorations = Some(WindowDecorations::Client);
            if let Some(titlebar) = options.titlebar.as_mut() {
                titlebar.title = Some(APP_NAME.into());
            }

            cx.open_window(options, |window, cx| {
                let view: Entity<GalleryApp> = cx.new(|cx| GalleryApp::new(window, cx));
                cx.new(|cx| Root::new(AnyView::from(view), window, cx))
            })
            .expect("failed to open window");
        });
}
