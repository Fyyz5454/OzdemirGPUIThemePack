//! Sheet demos: side panels opened via `window.open_sheet`.

use gpui_kit::component::button::Button;
use gpui_kit::component::WindowExt as _;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};
use gpui_kit::component::Placement;

use crate::pages::common;
use crate::pages::PageKind;

pub struct SheetPage;

impl SheetPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for SheetPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Sheet.title(), PageKind::Sheet.description(), cx);

        page = page.child(
            common::section("Right sheet", cx).child(
                common::example(cx).child(
                    Button::new("sheet-right")
                        .label("Right")
                        .on_click(|_, window, cx| {
                            window.open_sheet(cx, |sheet, _, _| {
                                sheet
                                    .title("Settings")
                                    .child(
                                        gpui_kit::div()
                                            .p_4()
                                            .child("Panel content sliding in from the right edge."),
                                    )
                            });
                        }),
                ),
            ),
        );

        page = page.child(
            common::section("Other placements", cx).child(
                common::example(cx)
                    .child(
                        Button::new("sheet-left")
                            .label("Left")
                            .on_click(|_, window, cx| {
                                window.open_sheet_at(Placement::Left, cx, |sheet, _, _| {
                                    sheet.title("Left panel").size(px(280.)).child(
                                        gpui_kit::div().p_4().child("Slides in from the left edge."),
                                    )
                                });
                            }),
                    )
                    .child(
                        Button::new("sheet-bottom")
                            .label("Bottom")
                            .on_click(|_, window, cx| {
                                window.open_sheet_at(Placement::Bottom, cx, |sheet, _, _| {
                                    sheet.title("Bottom panel").size(px(220.)).child(
                                        gpui_kit::div().p_4().child("Slides in from the bottom edge."),
                                    )
                                });
                            }),
                    ),
            ),
        );

        page
    }
}
