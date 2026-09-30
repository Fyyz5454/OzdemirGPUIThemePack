# FluentTheme — Fluent varyant iş akışı

> Bu belge [.kilo/skills/FluentTheme/SKILL.md](../.kilo/skills/FluentTheme/SKILL.md) dosyasının Türkçe aynasıdır.

Fluent tema sistemi **Rust önceliklidir**: tokenler `src/ozdemirgpuithemepack/src/fluentui/` içinde yaşar, CSS dosyaları türetilmiş artefaktlardır. `themes/*.css` dosyalarını elle asla düzenlemeyin.

## 1. Nesneler nerede?

| Dosya | Rolü |
|---|---|
| `src/ozdemirgpuithemepack/src/fluentui/colors.rs` | Fluent 2 token sabitleri (`ResourceDictionary::light()/dark()`), accent paletleri (`palette::*`), `Accent` enum'u (8 renk), yardımcılar `c(argb)`, `hex_string`, `with_alpha` |
| `src/ozdemirgpuithemepack/src/fluentui/theme.rs` | Tokenler → gpui-kit `ThemeConfig`: `theme_config(accent, dark)`, `color_entries()` (şema anahtarları — Rust ve CSS için tek kaynak), `theme_css()` (dosya baytları), `apply()`/`apply_config()` (çalışma zamanı, `Theme::change` üzerinden), `variants()` (16) |
| `src/ozdemirgpuithemepack/src/css.rs` | lightningcss yükleyici: `parse(&str)`/`load(path)` → `ThemeConfig` (katı `:root` dilbilgisi, renkler `#RRGGBBAA`'ya normalleşir) |
| `src/ozdemirgpuithemepack/src/fluentui/fonts.rs` | Yazı tipi aile önceliği: "Segoe UI Variable Text" → "Segoe UI Variable" → "Segoe UI" (çalışma zamanında çözümlenir, yoksa sessizce geri düşer) |
| `src/ozdemirgpuithemepack/src/bin/GenThemes.rs` | `themes/Fluent{Light|Dark}{Accent}.css` yazar |
| `themes/*.css` | Üretilmiş çıktı — dosya başına bir `:root` custom-property sayfası, toplam 16 |

## 2. Renk biçimleri — iki tuzak

- Tokenler **ARGB** u32 olarak yazılır (`0xAARRGGBB`). gpui'nin `rgba()` işlevi **RGBA bayt sırası** bekler (`0xRRGGBBAA`). ARGB sabitlerini asla doğrudan `rgba()`'ya vermeyin — mutlaka `colors.rs` içindeki `c()` üzerinden geçin; kanalları o takas eder.
- `hex_string(color)` `#RRGGBBAA` üretir — gpui-kit'in `try_parse_color` işlevinin 8 haneli hex için kabul ettiği tek biçim (`#RRGGBB` da kabul edilir). CSS yükleyicisi, girdi `#fff`/`rgba()` olsa bile her zaman bu kanonik biçimi yeniden üretir.

## 3. CSS tema dosyası dilbilgisi

- Dosya başına tek `:root` kuralı; birden çok `:root` kuralı çözümleme hatasıdır.
- Meta anahtarlar: `--name` (tırnaklı dizgi), `--mode` (`light`\|`dark`), `--radius`/`--radius-lg` (`Npx`), `--shadow` (`none` = düz Fluent denetimleri, `drop` = etkin).
- gpui-kit `ThemeConfigColors` şema anahtarı `primary.hover.background` → `--primary-hover-background` (tire↔nokta). `link.hover` → `--link-hover`.
- `font.family` dosyalarda bilinçli olarak yoktur (gpui-kit bilinmeyen aile adlarını atar; `fonts.rs` çalışma zamanında çözer).
- Bilinmeyen `--anahtarlar` yüklerken uyarır; `--name`/`--mode`/`--background`/`--foreground` eksikse hata verir.

## 4. Yeni accent rengi ekleme (uçtan uca)

1. `colors.rs`: 7 adımlı paletle `pub fn mycolor() -> AccentColor` ekleyin (`AccentColor::swatch(darkest, darker, dark, normal, light, lighter, lightest)`).
2. `Accent` enum'unu genişletin: varyant + `ALL` dizisi girdisi + `color()`, `name()` (tema adı ve dosya adı), `label()` (Türkçe UI etiketi).
3. `ThemeGallery/src/main.rs`: accent radyo grubu `Accent::ALL`/`label()` üzerinden otomatik çıkar; varyant `Select`'inin 2× varyanda sığdığını kontrol edin.
4. `theme.rs`: başka bir şey gerekmez — eşlemeler paletten türer. Testleri kontrol edin (`variant_count` 16 bekler → yeni sayıya güncelleyin).
5. `cargo test -p ozdemirgpuithemepack` çalıştırın (birim testler) — sonra **son adım**: `cargo run -p ozdemirgpuithemepack --bin GenThemes` ve Rust + CSS'i birlikte commit edin. `themes_are_in_sync` testi unutursanız başarısız olur.

## 5. Çalışma zamanı uygulama yolu

`fluentui::theme::apply(accent, dark, window, cx)` (tokenler) veya `fluentui::theme::apply_config(config, window, cx)` (ör. `css::load` kaynaklı yapılandırma):

1. Çözümlenen Fluent yazı tipi ailesini ekler (kuruluysa).
2. Yapılandırmayı `Theme::global_mut(cx).light_theme`/`dark_theme` yuvasına koyar.
3. `Theme::change(mode, window, cx)` çağırır — gpui-kit yapılandırmayı uygular, temel stilleri eşitler, metin varsayılanlarını kurar ve pencereyi tazeler. Bu yolun dışında asla tema rengi değiştirmeyin.

Başlangıç varsayılanı `Accent::Blue` açık (`ThemeGallery/src/main.rs`). Galerinın vurgu/açık-koyu anahtarı ürünün özelliğidir — canlı geçiş aynı `apply` yolundan gider.

## 6. Radius, gölge, yazı tipleri

- `FLUENT_RADIUS = 4` (denetimler), `FLUENT_RADIUS_LG = 8` (flyout/pencere), `shadow: Some(false)` (Fluent denetimleri düzdür) — `--radius: 4px; --radius-lg: 8px; --shadow: none;` olarak yazılır.
- Yazı tipi aileleri `cx.text_system().all_font_names()` ile yoklanır; CSS tema dosyaları bilinçli olarak `font.family` içermez.

## 7. lightningcss notları

- Pinli `=1.0.0-alpha.72`, MPL-2.0, saf Rust. Tüm kullanım `src/css.rs` içindedir — lightningcss'i başka yerde içe aktarmayın; çözümleyici takas edilebilir kalmalıdır.
- lightningcss `#f3f3f3`'ü `CssColor::RGBA` olarak çözümler ve sıkıştırılmış biçimde geri yazardı (`#f3f3f3`, `#fff`, renk adları). `css.rs` renkleri kendisi `#RRGGBBAA` olarak biçimlendirir; asla lightningcss renk serileştirmesine güvenmeyin.
- `px` uzunlukları `TokenOrValue::Length(LengthValue::Px(f32))` olarak gelir; ident/dizgiler lightningcss'in kendi `Token` enum'u (`Ident`, `String`, `Number`).
