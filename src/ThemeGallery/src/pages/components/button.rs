//! Button family: variants, sizes, states, icon buttons, groups, toggles,
//! and dropdown buttons.

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{
    Button, ButtonGroup, ButtonVariants as _, DropdownButton, Toggle,
};
use gpui_kit::component::menu::PopupMenuItem;
use gpui_kit::component::{
    Disableable as _, Icon, Sizable as _,
};
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct ButtonPage {
    toggle: bool,
    last: String,
}

impl ButtonPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self {
            toggle: false,
            last: "—".into(),
        }
    }
}

impl Render for ButtonPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Button.title(), PageKind::Button.description(), cx);

        // Variants.
        page = page.child(
            common::section("Variants", cx).child(
                common::example(cx)
                    .child(Button::new("v-primary").label("Primary").primary())
                    .child(Button::new("v-secondary").label("Secondary").secondary())
                    .child(Button::new("v-danger").label("Danger").danger())
                    .child(Button::new("v-warning").label("Warning").warning())
                    .child(Button::new("v-success").label("Success").success())
                    .child(Button::new("v-info").label("Info").info())
                    .child(Button::new("v-ghost").label("Ghost").ghost())
                    .child(Button::new("v-link").label("Link").link())
                    .child(Button::new("v-outline").label("Outline").secondary().outline()),
            ),
        );

        // Sizes.
        page = page.child(
            common::section("Sizes", cx).child(
                common::example(cx)
                    .child(Button::new("s-xs").label("X-Small").xsmall())
                    .child(Button::new("s-sm").label("Small").small())
                    .child(Button::new("s-md").label("Medium"))
                    .child(Button::new("s-lg").label("Large").large()),
            ),
        );

        // States.
        page = page.child(
            common::section("States", cx).child(
                common::example(cx)
                    .child(Button::new("st-disabled").label("Disabled").disabled(true))
                    .child(Button::new("st-loading").label("Loading").loading(true))
                    .child(
                        Button::new("st-icon")
                            .icon(Icon::new(IconName::Plus))
                            .tooltip("Add"),
                    )
                    .child(
                        Button::new("st-icon-label")
                            .label("Save")
                            .icon(Icon::new(IconName::Check)),
                    ),
            ),
        );

        // Click feedback + groups.
        let last = self.last.clone();
        page = page.child(
            common::section("Groups and actions", cx)
                .child(
                    common::example(cx)
                        .child(
                            Button::new("grp").child(format!("Son eylem: {last}")).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.last = "single button".into();
                                    cx.notify();
                                }),
                            ),
                        )
                        .child(
                            ButtonGroup::new("btn-group")
                                .child(Button::new("g-1").label("One"))
                                .child(Button::new("g-2").label("Two"))
                                .child(Button::new("g-3").label("Three")),
                        )
                        .child(
                            DropdownButton::new("dropdown")
                                .button(Button::new("dd-main").label("Menu"))
                                .dropdown_menu(|menu, _, _| {
                                    menu.label("Yeni")
                                        .separator()
                                        .item(PopupMenuItem::label("Open"))
                                        .item(PopupMenuItem::label("Save"))
                                }),
                        ),
                ),
        );

        // Toggles.
        let toggle = self.toggle;
        page = page.child(
            common::section("Toggle", cx).child(
                common::example(cx)
                    .child(
                        Toggle::new("toggle-1")
                            .label("Standalone")
                            .checked(toggle)
                            .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                this.toggle = *checked;
                                this.last = format!("toggle: {checked}");
                                cx.notify();
                            })),
                    )
                    .child(
                        Toggle::new("toggle-disabled")
                            .label("Disabled")
                            .checked(false)
                            .disabled(true),
                    ),
            ),
        );

        page
    }
}
