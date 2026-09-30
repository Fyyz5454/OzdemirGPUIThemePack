//! Breadcrumb demos.

use gpui_kit::component::breadcrumb::{Breadcrumb, BreadcrumbItem};
use gpui_kit::component::v_flex;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct BreadcrumbPage;

impl BreadcrumbPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for BreadcrumbPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Breadcrumb.title(),
            PageKind::Breadcrumb.description(),
            cx,
        );

        page = page.child(
            common::section("Page path", cx).child(
                v_flex().gap_3().child(
                    Breadcrumb::new().children([
                        BreadcrumbItem::new("Home"),
                        BreadcrumbItem::new("Components"),
                        BreadcrumbItem::new("Breadcrumb"),
                    ]),
                ),
            ),
        );

        page = page.child(
            common::section("Deep path", cx).child(
                v_flex().gap_3().child(
                    Breadcrumb::new().children([
                        BreadcrumbItem::new("Gallery"),
                        BreadcrumbItem::new("General"),
                        BreadcrumbItem::new("Colors"),
                        BreadcrumbItem::new("Accent ramps"),
                    ]),
                ),
            ),
        );

        page
    }
}
