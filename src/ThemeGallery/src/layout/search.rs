//! The search palette: a popover anchored to the title-bar search button that
//! filters and navigates gallery pages through a command palette.

use gpui_kit::assets::IconName as AssetsIcon;
use gpui_kit::component::command::{Command, CommandGroup, CommandItem};
use gpui_kit::component::popover::Popover;
use gpui_kit::component::{Icon, Sizable as _};
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::{Anchor, Focusable as _, Styled as _, px};

use crate::app::GalleryApp;
use crate::pages::{self, PageKind};

pub fn render(app: &GalleryApp, cx: &mut gpui_kit::Context<GalleryApp>) -> Popover {
    let cmd_state = app.search.clone();
    let app_handle = cx.entity();
    let focus = app.search.focus_handle(cx);

    Popover::new("search-popover")
        .anchor(Anchor::BottomRight)
        .open(app.search_open)
        .on_open_change(cx.listener(|this, open: &bool, _, cx| {
            this.search_open = *open;
            cx.notify();
        }))
        .trigger(
            Button::new("search-trigger")
                .ghost()
                .xsmall()
                .icon(Icon::new(AssetsIcon::Search))
                .tooltip("Search pages"),
        )
        .track_focus(&focus)
        .content(move |_, _, _| {
            let confirm_handle = app_handle.clone();
            Command::new(&cmd_state)
                .placeholder("Type a page name…")
                .bordered(false)
                .w(px(380.))
                .group(CommandGroup::new().label("General").items(items(pages::GENERAL)))
                .group(
                    CommandGroup::new()
                        .label("Components")
                        .items(items(pages::COMPONENTS)),
                )
                .on_confirm(move |index, _window, cx| {
                    confirm_handle.update(cx, |this, cx| {
                        if let Some(kind) = pages::page_at(&index) {
                            this.page = kind;
                        }
                        this.search_open = false;
                        cx.notify();
                    });
                })
        })
}

fn items(kinds: &[PageKind]) -> Vec<CommandItem> {
    kinds
        .iter()
        .map(|kind| CommandItem::new().label(kind.title()).icon(Icon::new(kind.icon())))
        .collect()
}
