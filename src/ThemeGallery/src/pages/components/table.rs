//! Static table demos: header, body rows, and cells.

use gpui_kit::component::table::{
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct TablePage;

impl TablePage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

const ROWS: [(&str, &str, &str); 6] = [
    ("FluentLightBlue", "#0078D4", "Light"),
    ("FluentDarkBlue", "#4CA0E0", "Dark"),
    ("FluentLightTeal", "#00B294", "Light"),
    ("FluentDarkTeal", "#4CC9B4", "Dark"),
    ("FluentLightPurple", "#744DA9", "Light"),
    ("FluentDarkPurple", "#9D82C2", "Dark"),
];

impl Render for TablePage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut body = TableBody::new();
        for (name, hex, mode) in ROWS.iter() {
            body = body.child(
                TableRow::new().children([
                    TableCell::new().child(*name),
                    TableCell::new().child(*hex),
                    TableCell::new().child(*mode),
                ]),
            );
        }

        let table = Table::new()
            .child(
                TableHeader::new().children([
                    TableHead::new().child("Variant"),
                    TableHead::new().child("Accent"),
                    TableHead::new().child("Mode"),
                ]),
            )
            .child(body);

        let mut page = common::page(PageKind::Table.title(), PageKind::Table.description(), cx);

        page = page.child(
            common::section("Theme variants", cx)
                .child(gpui_kit::div().w(px(480.)).child(table)),
        );

        page
    }
}
