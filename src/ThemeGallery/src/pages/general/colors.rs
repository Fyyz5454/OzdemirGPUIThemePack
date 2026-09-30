//! Fluent 2 color tokens: accent ramps, grey shades, and semantic colors,
//! rendered straight from the Rust token source.

use gpui_kit::component::h_flex;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Hsla, IntoElement, Render, Rgba, Window, px};
use ozdemirgpuithemepack::fluentui::colors::{self, AccentColor};
use ozdemirgpuithemepack::fluentui::Accent;

use crate::pages::common;
use crate::pages::PageKind;

pub struct ColorsPage;

impl ColorsPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for ColorsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Colors.title(),
            PageKind::Colors.description(),
            cx,
        );

        // Accent ramps: one row per accent, lightest → darkest.
        let mut ramps = common::example(cx).flex_col().items_start();
        for accent in Accent::ALL {
            ramps = ramps.child(ramp_row(accent.label(), &accent.color()));
        }
        page = page.child(common::section("Accent ramps", cx).child(ramps));

        // Grey shades.
        let mut greys = common::example(cx);
        for shade in [20, 40, 60, 80, 100, 120, 140, 160, 180, 200, 220] {
            greys = greys.child(swatch(
                &format!("grey {shade}"),
                colors::palette::grey(shade),
            ));
        }
        page = page.child(common::section("Grey shades", cx).child(greys));

        // Semantic colors.
        let semantic = common::example(cx)
            .child(swatch("warning", colors::palette::warning_primary()))
            .child(swatch("error", colors::palette::error_primary()))
            .child(swatch("success", colors::palette::success_primary()));
        page = page.child(common::section("Semantic colors", cx).child(semantic));

        page
    }
}

fn ramp_row(label: &str, ramp: &AccentColor) -> impl IntoElement + use<> {
    let shades = [
        ramp.lightest,
        ramp.lighter,
        ramp.light,
        ramp.normal,
        ramp.dark,
        ramp.darker,
        ramp.darkest,
    ];

    h_flex()
        .gap_2()
        .items_center()
        .child(
            gpui_kit::div()
                .w(px(90.))
                .text_size(px(12.))
                .child(label.to_string()),
        )
        .children(shades.map(|rgba| swatch("", rgba)))
}

fn swatch(label: &str, rgba: Rgba) -> impl IntoElement + use<> {
    let fill: Hsla = rgba.into();

    h_flex()
        .gap_1()
        .items_center()
        .child(gpui_kit::div().size_6().rounded_md().bg(fill).border_1().border_color(gpui_kit::black().opacity(0.1)))
        .when_some((!label.is_empty()).then(|| label.to_string()), |el, text| {
            el.child(gpui_kit::div().text_size(px(11.)).child(text))
        })
        .when(label.is_empty(), |el| {
            el.child(
                gpui_kit::div()
                    .text_size(px(10.))
                    .child(colors::hex_string(rgba)),
            )
        })
}
