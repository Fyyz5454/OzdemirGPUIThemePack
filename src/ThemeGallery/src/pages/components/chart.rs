//! Chart demos: bar, line, and pie charts over sample data.

use gpui_kit::component::chart::{BarChart, LineChart, PieChart};
use gpui_kit::component::v_flex;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

const MONTHS: [(&str, f64); 6] = [
    ("Apr", 8.5),
    ("May", 12.0),
    ("Jun", 9.5),
    ("Jul", 14.5),
    ("Aug", 17.0),
    ("Sep", 13.5),
];

pub struct ChartPage;

impl ChartPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for ChartPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Chart.title(), PageKind::Chart.description(), cx);

        // Bar chart.
        page = page.child(
            common::section("Bar chart", cx).child(
                gpui_kit::div()
                    .w(px(460.))
                    .h(px(200.))
                    .child(
                        BarChart::new(MONTHS)
                            .band(|d| d.0)
                            .value(|d| d.1),
                    ),
            ),
        );

        // Line chart.
        page = page.child(
            common::section("Line chart", cx).child(
                gpui_kit::div()
                    .w(px(460.))
                    .h(px(200.))
                    .child(
                        LineChart::new(MONTHS)
                            .x(|d| d.0)
                            .y(|d| d.1),
                    ),
            ),
        );

        // Pie chart.
        page = page.child(
            common::section("Pie chart", cx).child(
                v_flex().w(px(240.)).h(px(240.)).child(
                    PieChart::new([4.0f32, 3.0, 2.0, 1.0]).value(|d| *d),
                ),
            ),
        );

        page
    }
}
