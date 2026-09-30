# AGENTS.md — Katılımcı & Ajan Rehberi

Bu depoda çalışan insanlar ve kodlama ajanları için temel kurallar. İngilizce aslı: [../AGENTS.md](../AGENTS.md).

## Bu proje nedir

**OzdemirGPUIThemePack** — gpui-kit 0.6 için bir **Fluent UI tema paketi**: Rust-öncelikli token kütüphanesi (`ozdemirgpuithemepack`), crate içine gömülü CSS tema sayfaları (`resources/themes/*.css`) ve canlı önizlemeli galeri uygulaması (`ThemeGallery`). Ürün tanımı ve kararlar için [PRD.md](PRD.md).

Geçmiş: depo iptal edilmiş bir dosya yöneticisi ürünü olarak başladı; motor kodu silindi. Dosya yöneticisi/motor işini geri getirmeyin.

## Komutlar

| İş | Komut |
|---|---|
| Her şeyi derle | `cargo build --workspace` |
| Galeriyi çalıştır | `./run.sh` veya `cargo run -p ThemeGallery` |
| Testler | `cargo test --workspace` |
| Lint | `cargo clippy --workspace --all-targets` (uyarısız olmalı) |
| Tema CSS'ini yeniden üret | `cargo run -p ozdemirgpuithemepack --bin GenThemes` |

Sistem bağımlılıkları (bir kez): `sudo apt install libfontconfig1-dev libxkbcommon-dev libxkbcommon-x11-dev libxcb1-dev`.

## Mimari harita

- `src/ozdemirgpuithemepack` — **kütüphane crate'i**.
  - `src/fluentui/` — **token doğruluk kaynağı**: `colors.rs` (Fluent 2 tokenleri, ARGB u32 → gpui `Rgba`), `typography.rs`, `fonts.rs` (Segoe UI aile zinciri), `theme.rs` (tokenler → gpui-kit `ThemeConfig`; `Theme::change` üzerinden `apply`/`apply_config`; deterministik CSS üreticisi `theme_css`).
  - `src/css.rs` — CSS tema dosyası yükleyici (lightningcss): `:root` custom properties → `ThemeConfig`; renkler `#RRGGBBAA`'ya normalleşir.
  - `src/bin/GenThemes.rs` — `resources/themes/Fluent{Light|Dark}{Accent}.css` üretir.
- `src/ThemeGallery` — galeri uygulaması (AtlantaFX sampler düzeni): `layout/` (canlı tema anahtarı + arama paleti içeren başlık çubuğu, yan menü gezinmesi) ve `pages/` (kayıt + `general/` + `components/`, her gpui-kit bileşen ailesi için bir sayfa); İngilizce UI dizgileri.
- `src/ozdemirgpuithemepack/resources/themes/` — 16 üretilmiş dosya (`Fluent{Light|Dark}{Accent}.css`), `include_str!` ile gömülü ve `src/resources.rs` üzerinden sunulur. **Üretilir, elle asla düzenlenmez.**

## Dil ve araç kuralları

1. **Gelen prompt hangi dildeyse o dilde yanıt verin.**
2. **LSP, debugger MCP ve headless-web-browser MCP zorunlu araçlardır.** Oturumda gerekli araç/MCP yoksa bunu açıkça belirtin ve elden geldiğince devam edin.
3. **Bu global bir projedir: kod ve isimlendirme İngilizcedir.** Kullanıcıya görünür UI dizgileri de İngilizcedir.
4. **Web araştırması headless tarayıcı üzerinden yapılır**; sorgular **hem İngilizce hem Çince** yazılır.

## Katı kurallar

1. **Önce Rust, en son CSS.** Tüm Fluent token değişiklikleri `src/ozdemirgpuithemepack/src/fluentui/` içinde başlar. CSS tema dosyaları **en son adımda** `GenThemes` çalıştırılarak üretilir. `themes_are_in_sync` testi bayt eşitliğini zorunlu kılar — asla bypass etmeyin.
2. **Yeniden adlandırma/taşıma öncesi dry-run.** Önce etkilenen referansları listeleyin, gösterin, sonra uygulayın, sonra derleyin + test edin.
3. Tasarım kararları hakkında kullanıcıyla görüşürken **aynı anda tek soru**.
4. **JSON tema dosyası yok.** gpui-kit'in `ThemeRegistry`'si yalnızca JSON okur; bu paket JSON'ı bilinçli olarak `ozdemirgpuithemepack::css` üzerinden yüklenen CSS ile değiştirdi (PRD karar 1). Yeni bir kullanıcı kararı olmadan JSON tema çıktısını, `register_theme_dir`'ı veya doğrudan `ThemeSet` serileştirmesini geri getirmeyin.
5. **UI dizgileri İngilizcedir** (galeride; global proje — kullanıcıya görünür dizgiler UI katmanında İngilizce kalır).
6. **Pencere/başlık çubuğu**: gpui-kit `TitleBar` + `WindowOptions { window_decorations: Some(WindowDecorations::Client), ..TitleBar::window_options() }`. Sistem başlık çubuğunu geri getirmeyin.
7. **Commit'ler**: mesajlar **İngilizce**; depo-yerel git kimliği `omer <omer@localhost>` (global git yapılandırması asla değiştirilmez). Yalnızca kullanıcı istediğinde commit yapın.
8. **GPL bağımlılık alınmaz.** Bağımsız projeler yalnızca esin kaynağıdır; bağımlılık eklerken crate başına lisans doğrulayın (`lightningcss` MPL-2.0, saf Rust, pinli `=1.0.0-alpha.72`; gpui-kit yığını Apache-2.0). Projenin kendi lisansı AGPL-3.0-or-later (`LICENSE` dosyasına bakın).

## Ortam notları (Linux/X11)

- gpui, X11/wayland'a doğrudan bağlanır (winit yok). İstemci taraflı dekorasyonlar **çalışan bir compositor** + `_GTK_FRAME_EXTENTS` desteği ister; compositor yoksa gpui sessizce sunucu tarafı dekorasyonlara geri döner. `xprop -root _NET_WM_CM_S0` ile kontrol edin.
- Wayland'da protokol gereği istemci tarafı pencere konumlandırma imkânsızdır; "her zaman ortalanmış" mantığı denemeyin.
- gpui kendi wgpu çizicisini (blade/wgpu) gömer; ek GPU kurulumu gerekmez.

## Tema iş akışı (kısa sürüm)

Ayrıntılar: [skills/FluentTheme.md](skills/FluentTheme.md). Özet: `colors.rs`/`theme.rs` düzenle → `cargo test -p ozdemirgpuithemepack` → CSS'i `GenThemes` ile yeniden üret → Rust + CSS'i birlikte commit et.

## Bir şey tıkandığında

Bir gereksinim gpui/gpui-kit yetenekleriyle çelişiyorsa, vendored kaynaklara karşı doğrulayın (`~/.cargo/registry/src/*/gpui-component-0.6.6/…` okuyun) ya da https://gpui-kit.com belgelerine bakın ve bağımlılıkları yamalayan geçici çözümler uydurmak yerine kullanıcıya ödünleşimleri rapor edin.
