//! Gallery application shell: navigation state, live theme state, and the
//! lazily created per-page views.

use std::collections::HashMap;

use gpui_kit::component::command::CommandState;
use gpui_kit::component::select::{SelectEvent, SelectState};
use gpui_kit::component::{ActiveTheme as _, IndexPath, Root, h_flex};
use gpui_kit::{
    AnyView, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, SharedString,
    Styled as _, Subscription, Window, px,
};
use ozdemirgpuithemepack::fluentui::Accent;
use ozdemirgpuithemepack::fluentui::theme as fluent_theme;

use crate::layout;
use crate::pages::{self, PageKind};

pub const APP_NAME: &str = "OzdemirGPUIThemePack";

pub struct GalleryApp {
    pub page: PageKind,
    pub accent: Accent,
    pub dark: bool,
    pub sidebar_collapsed: bool,
    pub search_open: bool,
    pub search: Entity<CommandState>,
    pub accent_select: Entity<SelectState<Vec<SharedString>>>,
    pub pages: HashMap<PageKind, AnyView>,
    _subscriptions: Vec<Subscription>,
}

impl GalleryApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search = cx.new(|cx| CommandState::new(window, cx));

        let accent_names: Vec<SharedString> =
            Accent::ALL.iter().map(|accent| accent.name().into()).collect();
        let initial_ix = Accent::ALL
            .iter()
            .position(|accent| *accent == Accent::Blue)
            .unwrap_or(0);
        let accent_select = cx.new(|cx| {
            SelectState::new(accent_names, Some(IndexPath::new(initial_ix)), window, cx)
        });
        let accent_sub = cx.subscribe_in(
            &accent_select,
            window,
            |this, _, event, window, cx| {
                if let SelectEvent::Confirm(Some(value)) = event
                    && let Some(accent) = Accent::ALL.iter().find(|a| a.name() == value.as_ref())
                {
                    this.accent = *accent;
                    this.apply_theme(window, cx);
                }
            },
        );

        Self {
            page: PageKind::Overview,
            accent: Accent::Blue,
            dark: false,
            sidebar_collapsed: false,
            search_open: false,
            search,
            accent_select,
            pages: HashMap::new(),
            _subscriptions: vec![accent_sub],
        }
    }

    pub fn apply_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        fluent_theme::apply(self.accent, self.dark, Some(window), cx);

        // Keep the title-bar accent dropdown in sync when the accent changes
        // from elsewhere (sun/moon toggle, Tema page cards). set_selected_index
        // does not emit Confirm, so this cannot re-enter the subscription.
        if let Some(ix) = Accent::ALL.iter().position(|a| *a == self.accent) {
            self.accent_select.update(cx, |state, cx| {
                state.set_selected_index(Some(IndexPath::new(ix)), window, cx);
            });
        }

        cx.notify();
    }

    pub fn theme_label(&self) -> String {
        fluent_theme::theme_name(self.accent, self.dark)
    }

    fn page_view(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyView {
        let kind = self.page;
        self.pages
            .entry(kind)
            .or_insert_with(|| pages::make(kind, window, cx))
            .clone()
    }
}

impl Render for GalleryApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let page_view = self.page_view(window, cx);

        div_root(cx)
            .child(layout::titlebar::render(self, cx))
            .child(
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .child(layout::sidebar::render(self, cx))
                    .child(
                        gpui_kit::div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w_0()
                            .min_h_0()
                            .child(page_view),
                    ),
            )
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}

fn div_root(cx: &Context<GalleryApp>) -> gpui_kit::Div {
    gpui_kit::div()
        .flex()
        .flex_col()
        .size_full()
        .bg(cx.theme().background)
        .text_color(cx.theme().foreground)
        .text_size(px(14.))
}
