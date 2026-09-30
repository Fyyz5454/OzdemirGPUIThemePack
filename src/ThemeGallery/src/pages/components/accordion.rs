//! Accordion demos: single and multiple open modes.

use gpui_kit::component::accordion::Accordion;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct AccordionPage;

impl AccordionPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for AccordionPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Accordion.title(),
            PageKind::Accordion.description(),
            cx,
        );

        page = page.child(
            common::section("Single-open mode", cx).child(
                gpui_kit::div().w(px(420.)).child(
                    Accordion::new("accordion-single")
                        .item(|item| item.title("Overview").child("Purpose and contents of the theme pack."))
                        .item(|item| item.title("Colors").child("8 accent ramps, grey shades, and semantic colors."))
                        .item(|item| {
                            item.title("Theme")
                                .child("16 variants: 8 accents × light/dark.")
                        }),
                ),
            ),
        );

        page = page.child(
            common::section("Multiple-open mode", cx).child(
                gpui_kit::div().w(px(420.)).child(
                    Accordion::new("accordion-multiple")
                        .multiple(true)
                        .item(|item| item.title("First").child("Can stay open at the same time."))
                        .item(|item| item.title("Second").child("These are independent.")),
                ),
            ),
        );

        page = page.child(
            common::section("Borderless", cx).child(
                gpui_kit::div().w(px(420.)).child(
                    Accordion::new("accordion-borderless")
                        .bordered(false)
                        .item(|item| item.title("Plain").child("A borderless look.")),
                ),
            ),
        );

        page
    }
}
