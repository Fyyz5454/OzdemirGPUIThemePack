//! Tooltip demos: buttons with tooltips at different placements.

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::{
    Icon, Placement, 
};
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct TooltipPage;

impl TooltipPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for TooltipPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Tooltip.title(),
            PageKind::Tooltip.description(),
            cx,
        );

        page = page.child(
            common::section("Button tooltips", cx).child(
                common::example(cx)
                    .child(
                        Button::new("tt-default")
                            .label("Default")
                            .tooltip("Appears on hover"),
                    )
                    .child(
                        Button::new("tt-bottom")
                            .label("Bottom")
                            .tooltip("Bottom placement")
                            .tooltip_placement(Placement::Bottom),
                    )
                    .child(
                        Button::new("tt-left")
                            .label("Left")
                            .tooltip("Left placement")
                            .tooltip_placement(Placement::Left),
                    )
                    .child(
                        Button::new("tt-right")
                            .label("Right")
                            .tooltip("Right placement")
                            .tooltip_placement(Placement::Right),
                    )
                    .child(
                        Button::new("tt-icon")
                            .ghost()
                            .icon(Icon::new(IconName::Info))
                            .tooltip("Info"),
                    ),
            ),
        );

        page
    }
}
