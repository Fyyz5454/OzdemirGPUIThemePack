//! OzdemirGPUIThemePack gallery — live Fluent theme preview on gpui-kit.
//!
//! The gallery is the theme pack's showcase: every gpui-kit control is
//! rendered under the active Fluent variant, and the accent/brightness
//! switcher applies a new variant live (this is the product; there is no
//! separate "app theme").

use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::progress::Progress;
use gpui_kit::component::radio::RadioGroup;
use gpui_kit::component::select::{Select, SelectEvent, SelectState};
use gpui_kit::component::slider::{Slider, SliderEvent, SliderState};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{ActiveTheme as _, IndexPath, Root, TitleBar};
use gpui_kit::{
    AnyView, AppContext as _, Bounds, Context, Entity, IntoElement, ParentElement as _, Render,
    SharedString, Styled as _, Subscription, Window, WindowBounds, WindowDecorations, div, px,
    size,
};
use ozdemirgpuithemepack::fluentui::theme as fluent_theme;
use ozdemirgpuithemepack::fluentui::Accent;

const APP_NAME: &str = "OzdemirGPUIThemePack";
const VIEW_KINDS: [&str; 3] = ["Liste", "Izgara", "Ayrıntılar"];

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);

            fluent_theme::apply(Accent::Blue, false, None, cx);

            let bounds = Bounds::centered(None, size(px(860.), px(620.)), cx);
            let mut options = TitleBar::window_options();
            options.window_bounds = Some(WindowBounds::Windowed(bounds));
            // Use the gpui-kit TitleBar's client-side decorations instead of the
            // system (WM) title bar (X11: _MOTIF_WM_HINTS; without a compositor
            // gpui falls back to server-side decorations).
            options.window_decorations = Some(WindowDecorations::Client);
            if let Some(titlebar) = options.titlebar.as_mut() {
                titlebar.title = Some(APP_NAME.into());
            }

            cx.open_window(options, |window, cx| {
                let view: Entity<ThemeGalleryApp> = cx.new(|cx| ThemeGalleryApp::new(window, cx));
                cx.new(|cx| Root::new(AnyView::from(view), window, cx))
            })
            .expect("failed to open window");
        });
}

struct GalleryState {
    checked: bool,
    view: Option<usize>,
    density: f32,
    last: String,
}

impl Default for GalleryState {
    fn default() -> Self {
        Self {
            checked: false,
            view: Some(0),
            density: 0.0,
            last: String::new(),
        }
    }
}

struct ThemeGalleryApp {
    accent: Accent,
    dark: bool,
    gallery: GalleryState,
    search: Entity<InputState>,
    variant: Entity<SelectState<Vec<SharedString>>>,
    density: Entity<SliderState>,
    _subscriptions: Vec<Subscription>,
}

impl ThemeGalleryApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search = cx.new(|cx| InputState::new(window, cx).placeholder("Ara..."));

        let variant_names: Vec<SharedString> = fluent_theme::variants()
            .map(|v| fluent_theme::theme_name(v.accent, v.dark).into())
            .collect();
        let variant = cx.new(|cx| {
            SelectState::new(variant_names, Some(IndexPath::new(0)), window, cx)
        });

        let density = cx.new(|_| SliderState::new().min(0.0).max(100.0).default_value(0.0f32));

        let subscriptions = vec![
            cx.subscribe_in(&variant, window, |this, _, event, window, cx| {
                if let SelectEvent::Confirm(Some(value)) = event
                    && let Some(v) = fluent_theme::variants()
                        .find(|v| fluent_theme::theme_name(v.accent, v.dark) == value.as_ref())
                {
                    this.accent = v.accent;
                    this.dark = v.dark;
                    this.gallery.last = format!("tema: {value}");
                    this.apply_theme(window, cx);
                }
            }),
            cx.subscribe_in(&density, window, |this, _, event, _, cx| {
                if let SliderEvent::Change(value) = event {
                    this.gallery.density = value.start();
                    cx.notify();
                }
            }),
        ];

        Self {
            accent: Accent::Blue,
            dark: false,
            gallery: GalleryState::default(),
            search,
            variant,
            density,
            _subscriptions: subscriptions,
        }
    }

    fn apply_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        fluent_theme::apply(self.accent, self.dark, Some(window), cx);
        cx.notify();
    }

    fn primary_brush(&self) -> gpui_kit::Hsla {
        self.accent.color().default_brush_for(self.dark).into()
    }

    fn theme_column(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let gallery = &self.gallery;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .flex_1()
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(cx.theme().muted_foreground)
                    .child(format!(
                        "Aktif tema: {}",
                        fluent_theme::theme_name(self.accent, self.dark)
                    )),
            )
            .child(
                RadioGroup::new("vurgu-grubu")
                    .children(Accent::ALL.map(|a| a.label()))
                    .selected_index(Some(
                        Accent::ALL
                            .iter()
                            .position(|a| *a == self.accent)
                            .unwrap_or(0),
                    ))
                    .on_change(cx.listener(|this, value: &usize, window, cx| {
                        if let Some(accent) = Accent::ALL.get(*value).copied() {
                            this.accent = accent;
                            this.gallery.last = format!("vurgu: {}", accent.label());
                            this.apply_theme(window, cx);
                        }
                    })),
            )
            .child(
                Switch::new("sw-koyu")
                    .label("Koyu tema")
                    .checked(self.dark)
                    .on_change(cx.listener(|this, value: &bool, window, cx| {
                        this.dark = *value;
                        this.gallery.last = if *value {
                            "koyu tema".into()
                        } else {
                            "açık tema".into()
                        };
                        this.apply_theme(window, cx);
                    })),
            )
            .child(
                Checkbox::new("chk-onizleme")
                    .label("Canlı önizleme")
                    .checked(gallery.checked)
                    .on_change(cx.listener(|this, value, _, cx| {
                        this.gallery.checked = *value;
                        cx.notify();
                    })),
            )
            .child(
                Button::new("btn-sifirla")
                    .label("Varsayılana dön")
                    .ghost()
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.accent = Accent::Blue;
                        this.dark = false;
                        this.gallery.last = "varsayılan".into();
                        this.apply_theme(window, cx);
                    })),
            )
    }

    fn widget_column(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let gallery = &self.gallery;

        div()
            .flex()
            .flex_col()
            .items_start()
            .gap_3()
            .flex_1()
            .child(Input::new(&self.search).w(px(220.)))
            .child(Select::new(&self.variant).w(px(220.)))
            .child(Slider::new(&self.density).w(px(220.)))
            .child(
                Progress::new("yoğunluk")
                    .value(gallery.density / 100.0)
                    .w(px(220.)),
            )
            .child(
                Button::new("btn-bilgi")
                    .label("Bilgi")
                    .ghost()
                    .tooltip("Tema ve bileşen bilgisi"),
            )
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(cx.theme().muted_foreground)
                    .child(format!(
                        "Son eylem: {}",
                        if gallery.last.is_empty() {
                            "—"
                        } else {
                            &gallery.last
                        }
                    )),
            )
    }

    fn controls_column(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let gallery = &self.gallery;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .flex_1()
            .child(
                Button::new("btn-standart")
                    .label("Standart")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.gallery.last = "standart tıklandı".into();
                        cx.notify();
                    })),
            )
            .child(
                Button::new("btn-subtle")
                    .label("Subtle")
                    .ghost()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.gallery.last = "subtle tıklandı".into();
                        cx.notify();
                    })),
            )
            .child(
                Button::new("btn-link")
                    .label("Bağlantı")
                    .link()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.gallery.last = "bağlantı tıklandı".into();
                        cx.notify();
                    })),
            )
            .child(
                RadioGroup::new("gorunum-grubu")
                    .children(VIEW_KINDS)
                    .selected_index(gallery.view)
                    .on_change(cx.listener(|this, value, _, cx| {
                        this.gallery.view = Some(*value);
                        this.gallery.last = format!(
                            "görünüm: {}",
                            VIEW_KINDS.get(*value).copied().unwrap_or("—")
                        );
                        cx.notify();
                    })),
            )
    }
}

impl Render for ThemeGalleryApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .text_size(px(14.))
            .child(
                TitleBar::new().child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(div().size_2().rounded_full().bg(self.primary_brush()))
                        .child(APP_NAME),
                ),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .p_4()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_6()
                            .child(self.theme_column(cx))
                            .child(self.controls_column(cx))
                            .child(self.widget_column(cx)),
                    ),
            )
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}
