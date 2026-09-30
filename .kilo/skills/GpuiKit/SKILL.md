---
name: GpuiKit
description: gpui-kit 0.6 component patterns as used in OzdemirGPUIThemePack (application bootstrap, Root, TitleBar/CSD, theme access, widgets, events, Fluent bridge). Use when building new UI with gpui-kit in this codebase.
---

# GpuiKit — component patterns in this codebase

Reference: https://gpui-kit.com/ (crate `gpui-kit = "0.6"`, internal modules re-exported from `gpui_component`). Working examples live in `src/ThemeGallery/src/main.rs`.

## 1. Application bootstrap

```rust
gpui_kit::application()
    .with_assets(gpui_kit::assets::Assets)
    .run(|cx| {
        gpui_kit::init(cx);               // MUST be the first line
        // theme setup, then:
        cx.open_window(options, |window, cx| {
            let view: Entity<T> = cx.new(|cx| T::new(window, cx));
            cx.new(|cx| Root::new(AnyView::from(view), window, cx))
        })
        .expect("pencere açılamadı");
    });
```

- The view returned to `open_window` must be wrapped in `Root`.
- In `render`, add the overlay layers at the end of the root `div`:
  `Root::render_dialog_layer(window, cx)`, `Root::render_sheet_layer(...)`, `Root::render_notification_layer(...)`.

## 2. Window options & client-side title bar

```rust
let mut options = TitleBar::window_options();
options.window_bounds = Some(WindowBounds::Windowed(Bounds::centered(None, size(px(800.), px(600.)), cx)));
options.window_decorations = Some(WindowDecorations::Client); // no WM title bar (X11 needs a compositor; auto-fallback otherwise)
if let Some(tb) = options.titlebar.as_mut() { tb.title = Some(APP_NAME.into()); }
```

`TitleBar` draws its own min/max/close controls on Linux and handles dragging + double-click maximize. Custom close behavior: `TitleBar::new().on_close_window(|_, window, cx| …)`.

## 3. Theme access

- `use gpui_kit::component::ActiveTheme as _;` then `cx.theme().background`, `cx.theme().primary`, …
- Applying a whole theme: `Theme::global_mut(cx)` slots + `Theme::change(mode, window, cx)` — this is what `fluentui::theme::apply()` does. Don't hand-mutate scattered color fields.

## 4. Widgets used here (import paths)

| Widget | Import | Notes |
|---|---|---|
| Button | `gpui_kit::component::button::{Button, ButtonVariants as _}` | `.label()`, variants `.ghost()`, `.link()`, `.primary()`; `.tooltip("…")`, `.on_click(cx.listener(...))` |
| Checkbox | `gpui_kit::component::checkbox::Checkbox` | `.label().checked(bool).on_change(cx.listener(|this, v: &bool, _, cx| …))` |
| Switch | `gpui_kit::component::switch::Switch` | same on_change shape |
| RadioGroup | `gpui_kit::component::radio::RadioGroup` | `.children(["a","b"])` (`&str: Into<Radio>`), `.selected_index(Option<usize>)`, `.on_change(cx.listener(|this, ix: &usize, _, cx| …))` |
| Input | `gpui_kit::component::input::{Input, InputState}` | `let state = cx.new(|cx| InputState::new(window, cx).placeholder("…")); Input::new(&state)` |
| Select | `gpui_kit::component::select::{Select, SelectEvent, SelectState}` | delegate = `Vec<SharedString>`; `SelectState::new(items, Some(IndexPath::new(0)), window, cx)`; subscribe `SelectEvent::Confirm(Option<SharedString>)` |
| Slider | `gpui_kit::component::slider::{Slider, SliderEvent, SliderState}` | state: `SliderState::new().min(0.).max(100.)`; `SliderEvent::Change(SliderValue)` → `value.start()` |
| Progress | `gpui_kit::component::progress::Progress` | `Progress::new(id).value(0.0..=1.0)` |
| Tooltip | `.tooltip("…")` on buttons | text-based |
| TitleBar | `gpui_kit::component::TitleBar` | `.child(div().child("…"))` |

## 5. Events: `cx.listener` vs `cx.subscribe_in`

- Component callbacks (`on_click`, `on_change`) take `Fn(&E, &mut Window, &mut App)` — produce them with `cx.listener(|this, event, window, cx| …)` (gives you `&mut ThemeGalleryApp` + `Context<Self>`; call `cx.notify()` after state changes).
- Entity events (Select/Slider/Input): subscribe once in `new()` and store the `Subscription`:

```rust
cx.subscribe_in(&channel, window, |this, _, event, _, cx| {
    if let SelectEvent::Confirm(Some(value)) = event { /* … */ cx.notify(); }
});
```

## 6. Layout idioms

- `div().flex().flex_col().size_full().bg(cx.theme().background).text_color(cx.theme().foreground)` as the app shell; Tailwind-style spacing (`p_4`, `gap_2`, `size_4`, `w(px(220.))`).
- Root-level `.text_size(px(14.))` for Fluent body size.
- `TitleBar` height is fixed (34px) — put page content in a sibling `div().flex_1().min_h_0()`.

## 7. Gotchas seen in this repo

- `Window` has an inherent `window_handle()` (returns `AnyWindowHandle`); for the raw X11 handle call `raw_window_handle::HasWindowHandle::window_handle(window)` explicitly.
- `Option`s in `ThemeConfig` reset to gpui-kit defaults when absent — partial configs are the norm.
- Wayland: no window positioning by protocol; CSD behaves differently — don't assume X11 behavior.
