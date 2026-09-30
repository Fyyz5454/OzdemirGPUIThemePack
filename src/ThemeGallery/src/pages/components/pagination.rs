//! Pagination demos.

use gpui_kit::component::pagination::Pagination;
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct PaginationPage {
    page: usize,
}

impl PaginationPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self { page: 1 }
    }
}

impl Render for PaginationPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Pagination.title(),
            PageKind::Pagination.description(),
            cx,
        );

        let current = self.page;

        page = page.child(
            common::section(&format!("Page {} of 20", current), cx).child(
                common::example(cx).child(
                    Pagination::new("pagination-demo")
                        .current_page(current)
                        .total_pages(20)
                        .on_click(cx.listener(|this, page: &usize, _, cx| {
                            this.page = *page;
                            cx.notify();
                        })),
                ),
            ),
        );

        page = page.child(
            common::section("Compact", cx).child(
                common::example(cx).child(
                    Pagination::new("pagination-compact")
                        .current_page(current.min(8))
                        .total_pages(8)
                        .compact()
                        .on_click(cx.listener(|this, page: &usize, _, cx| {
                            this.page = (*page).min(8);
                            cx.notify();
                        })),
                ),
            ),
        );

        page
    }
}
