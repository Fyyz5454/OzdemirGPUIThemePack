//! Collapsible demos: trigger + expandable content.

use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::collapsible::Collapsible;
use gpui_kit::component::v_flex;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct CollapsiblePage {
    open: bool,
}

impl CollapsiblePage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self { open: false }
    }
}

impl Render for CollapsiblePage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let open = self.open;
        let mut page = common::page(
            PageKind::Collapsible.title(),
            PageKind::Collapsible.description(),
            cx,
        );

        page = page.child(
            common::section("Expandable region", cx).child(
                v_flex()
                    .w(px(360.))
                    .child(
                        Collapsible::new()
                            .open(open)
                            .child(
                                Button::new("collapsible-trigger")
                                    .label(if open { "Collapse" } else { "Expand" })
                                    .ghost()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.open = !this.open;
                                        cx.notify();
                                    })),
                            )
                            .content(
                                v_flex()
                                    .p_3()
                                    .child("This content expands and collapses when clicking the trigger."),
                            ),
                    ),
            ),
        );

        page = page.child(
            common::section("Open by default", cx).child(
                v_flex()
                    .w(px(360.))
                    .child(
                        Collapsible::new()
                            .open(true)
                            .child(
                                gpui_kit::div()
                                    .p_2()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .child("Always-visible header"),
                            )
                            .content(v_flex().p_3().child("Expanded content.")),
                    ),
            ),
        );

        page
    }
}
