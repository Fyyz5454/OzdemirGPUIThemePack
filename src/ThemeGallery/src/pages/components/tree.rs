//! Tree demos: nested items with expand/collapse.

use gpui_kit::assets::IconName;
use gpui_kit::component::list::ListItem;
use gpui_kit::component::tree::{Tree, TreeItem, TreeState};
use gpui_kit::component::Icon;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Entity, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

fn sample_tree() -> Vec<TreeItem> {
    vec![
        TreeItem::new("src", "src").child(
            TreeItem::new("src-fluentui", "fluentui")
                .child(TreeItem::new("colors-rs", "colors.rs"))
                .child(TreeItem::new("theme-rs", "theme.rs")),
        ),
        TreeItem::new("themes", "themes")
            .child(TreeItem::new("light", "FluentLight*.css"))
            .child(TreeItem::new("dark", "FluentDark*.css")),
        TreeItem::new("readme", "README.md"),
    ]
}

pub struct TreePage {
    state: Entity<TreeState>,
}

impl TreePage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let state = cx.new(|cx| TreeState::new(cx).items(sample_tree()));
        Self { state }
    }
}

impl Render for TreePage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Tree.title(), PageKind::Tree.description(), cx);

        page = page.child(
            common::section("File tree", cx).child(
                gpui_kit::div()
                    .w(px(340.))
                    .child(Tree::new(&self.state, |ix, entry, _selected, _window, _cx| {
                        let item = entry.item();
                        let depth = entry.depth();
                        let icon = if item.children.is_empty() {
                            Icon::new(IconName::File).size_4()
                        } else {
                            Icon::new(IconName::Folder).size_4()
                        };
                        ListItem::new(("tree-row", ix))
                            .child(icon)
                            .child(item.label.clone())
                            .pl(px(8. + 16. * depth as f32))
                    })),
            ),
        );

        page
    }
}
