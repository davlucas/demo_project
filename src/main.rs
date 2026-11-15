use chrono::{Local, Utc};
use chrono_tz::Tz;
use eframe::egui::{self, Color32, Frame, Margin, RichText, ScrollArea, Stroke, Vec2};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum AppLanguage {
    French,
    English,
    Japanese,
    Chinese,
    Spanish,
}

impl AppLanguage {
    fn code(&self) -> &'static str {
        match self {
            AppLanguage::French => "fr",
            AppLanguage::English => "en",
            AppLanguage::Japanese => "ja",
            AppLanguage::Chinese => "zh",
            AppLanguage::Spanish => "es",
        }
    }

    fn from_code(code: &str) -> Self {
        match code {
            "en" => AppLanguage::English,
            "ja" => AppLanguage::Japanese,
            "zh" => AppLanguage::Chinese,
            "es" => AppLanguage::Spanish,
            _ => AppLanguage::French,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            AppLanguage::French => "🇫🇷 Français",
            AppLanguage::English => "🇬🇧 English",
            AppLanguage::Japanese => "🇯🇵 日本語",
            AppLanguage::Chinese => "🇨🇳 中文",
            AppLanguage::Spanish => "🇪🇸 Español",
        }
    }
}

struct I18n {
    app_title: &'static str,
    format_24h: &'static str,
    show_seconds: &'static str,
    show_date: &'static str,
    language_label: &'static str,
    local_time_title: &'static str,
    utc_time_title: &'static str,
    tz_offset_prefix: &'static str,
    utc_reference: &'static str,
    add_tz_title: &'static str,
    step_continent: &'static str,
    step_search_city: &'static str,
    selection_label: &'static str,
    already_added: &'static str,
    add_and_save_button: &'static str,
    no_city_found: &'static str,
    save_notice: &'static str,
    remove_tooltip: &'static str,
    config_label: &'static str,
}

impl I18n {
    fn get(lang: AppLanguage) -> Self {
        match lang {
            AppLanguage::French => Self {
                app_title: "⏱ HORLOGE DIGITALE MULTI-FUSEAUX",
                format_24h: "Format 24h",
                show_seconds: "Secondes",
                show_date: "Date",
                language_label: "Langue :",
                local_time_title: "📍 HEURE LOCALE",
                utc_time_title: "🌐 TEMPS UNIVERSEL (UTC)",
                tz_offset_prefix: "Fuseau : UTC",
                utc_reference: "Référence : UTC+00:00 (Z)",
                add_tz_title: "➕ AJOUTER UN FUSEAU HORAIRE",
                step_continent: "1. Continent :",
                step_search_city: "2. Recherche ville / pays :",
                selection_label: "Sélection :",
                already_added: "✔ Déjà affiché dans l'horloge",
                add_and_save_button: "➕ Ajouter et Sauvegarder",
                no_city_found: "Aucune ville trouvée.",
                save_notice: "💾 Les fuseaux ajoutés sont automatiquement enregistrés dans USERDIR/.digital_clock.",
                remove_tooltip: "Supprimer et sauvegarder",
                config_label: "Config :",
            },
            AppLanguage::English => Self {
                app_title: "⏱ MULTI-TIMEZONE DIGITAL CLOCK",
                format_24h: "24h format",
                show_seconds: "Seconds",
                show_date: "Date",
                language_label: "Language:",
                local_time_title: "📍 LOCAL TIME",
                utc_time_title: "🌐 UNIVERSAL TIME (UTC)",
                tz_offset_prefix: "Zone: UTC",
                utc_reference: "Reference: UTC+00:00 (Z)",
                add_tz_title: "➕ ADD A TIMEZONE",
                step_continent: "1. Continent:",
                step_search_city: "2. Search city / country:",
                selection_label: "Selection:",
                already_added: "✔ Already displayed in the clock",
                add_and_save_button: "➕ Add and Save",
                no_city_found: "No city found.",
                save_notice: "💾 Added timezones are automatically saved in USERDIR/.digital_clock.",
                remove_tooltip: "Delete and save",
                config_label: "Config:",
            },
            AppLanguage::Japanese => Self {
                app_title: "⏱ マルチタイムゾーン デジタル時計",
                format_24h: "24時間表示",
                show_seconds: "秒表示",
                show_date: "日付",
                language_label: "言語:",
                local_time_title: "📍 現地時間 (ローカル)",
                utc_time_title: "🌐 協定世界時 (UTC)",
                tz_offset_prefix: "タイムゾーン: UTC",
                utc_reference: "基準: UTC+00:00 (Z)",
                add_tz_title: "➕ タイムゾーンを追加",
                step_continent: "1. 大陸・地域:",
                step_search_city: "2. 都市 / 国を検索:",
                selection_label: "選択:",
                already_added: "✔ すでに表示されています",
                add_and_save_button: "➕ 追加して保存",
                no_city_found: "都市が見つかりません。",
                save_notice: "💾 追加されたタイムゾーンは USERDIR/.digital_clock に自動保存されます。",
                remove_tooltip: "削除して保存",
                config_label: "設定ファイル:",
            },
            AppLanguage::Chinese => Self {
                app_title: "⏱ 多时区数字时钟",
                format_24h: "24小时制",
                show_seconds: "显示秒",
                show_date: "显示日期",
                language_label: "语言:",
                local_time_title: "📍 本地时间",
                utc_time_title: "🌐 协调世界时 (UTC)",
                tz_offset_prefix: "时区: UTC",
                utc_reference: "基准: UTC+00:00 (Z)",
                add_tz_title: "➕ 添加时区",
                step_continent: "1. 大洲 / 区域:",
                step_search_city: "2. 搜索城市 / 国家:",
                selection_label: "已选:",
                already_added: "✔ 已在时钟中显示",
                add_and_save_button: "➕ 添加并保存",
                no_city_found: "未找到匹配城市。",
                save_notice: "💾 添加的时区会自动保存在 USERDIR/.digital_clock 中。",
                remove_tooltip: "删除并保存",
                config_label: "配置文件:",
            },
            AppLanguage::Spanish => Self {
                app_title: "⏱ RELOJ DIGITAL MULTIZONA",
                format_24h: "Formato 24h",
                show_seconds: "Segundos",
                show_date: "Fecha",
                language_label: "Idioma:",
                local_time_title: "📍 HORA LOCAL",
                utc_time_title: "🌐 TIEMPO UNIVERSAL (UTC)",
                tz_offset_prefix: "Zona: UTC",
                utc_reference: "Referencia: UTC+00:00 (Z)",
                add_tz_title: "➕ AÑADIR ZONA HORARIA",
                step_continent: "1. Continente:",
                step_search_city: "2. Buscar ciudad / país:",
                selection_label: "Selección:",
                already_added: "✔ Ya se muestra en el reloj",
                add_and_save_button: "➕ Añadir y Guardar",
                no_city_found: "No se encontró ninguna ciudad.",
                save_notice: "💾 Las zonas horarias añadidas se guardan automáticamente en USERDIR/.digital_clock.",
                remove_tooltip: "Eliminar y guardar",
                config_label: "Configuración:",
            },
        }
    }

    fn continent_name(&self, key: &str, lang: AppLanguage) -> &'static str {
        match (key, lang) {
            ("Africa", AppLanguage::French) => "Afrique (Africa)",
            ("Africa", AppLanguage::English) => "Africa",
            ("Africa", AppLanguage::Japanese) => "アフリカ (Africa)",
            ("Africa", AppLanguage::Chinese) => "非洲 (Africa)",
            ("Africa", AppLanguage::Spanish) => "África (Africa)",

            ("America", AppLanguage::French) => "Amérique (America)",
            ("America", AppLanguage::English) => "America",
            ("America", AppLanguage::Japanese) => "アメリカ (America)",
            ("America", AppLanguage::Chinese) => "美洲 (America)",
            ("America", AppLanguage::Spanish) => "América (America)",

            ("Antarctica", AppLanguage::French) => "Antarctique (Antarctica)",
            ("Antarctica", AppLanguage::English) => "Antarctica",
            ("Antarctica", AppLanguage::Japanese) => "南極 (Antarctica)",
            ("Antarctica", AppLanguage::Chinese) => "南极洲 (Antarctica)",
            ("Antarctica", AppLanguage::Spanish) => "Antártida (Antarctica)",

            ("Asia", AppLanguage::French) => "Asie (Asia)",
            ("Asia", AppLanguage::English) => "Asia",
            ("Asia", AppLanguage::Japanese) => "アジア (Asia)",
            ("Asia", AppLanguage::Chinese) => "亚洲 (Asia)",
            ("Asia", AppLanguage::Spanish) => "Asia",

            ("Atlantic", AppLanguage::French) => "Atlantique (Atlantic)",
            ("Atlantic", AppLanguage::English) => "Atlantic",
            ("Atlantic", AppLanguage::Japanese) => "大西洋 (Atlantic)",
            ("Atlantic", AppLanguage::Chinese) => "大西洋 (Atlantic)",
            ("Atlantic", AppLanguage::Spanish) => "Atlántico (Atlantic)",

            ("Australia", AppLanguage::French) => "Australie (Australia)",
            ("Australia", AppLanguage::English) => "Australia",
            ("Australia", AppLanguage::Japanese) => "オーストラリア (Australia)",
            ("Australia", AppLanguage::Chinese) => "大洋洲 / 澳大利亚 (Australia)",
            ("Australia", AppLanguage::Spanish) => "Australia",

            ("Europe", AppLanguage::French) => "Europe",
            ("Europe", AppLanguage::English) => "Europe",
            ("Europe", AppLanguage::Japanese) => "ヨーロッパ (Europe)",
            ("Europe", AppLanguage::Chinese) => "欧洲 (Europe)",
            ("Europe", AppLanguage::Spanish) => "Europa (Europe)",

            ("Indian", AppLanguage::French) => "Océan Indien (Indian)",
            ("Indian", AppLanguage::English) => "Indian Ocean",
            ("Indian", AppLanguage::Japanese) => "インド洋 (Indian Ocean)",
            ("Indian", AppLanguage::Chinese) => "印度洋 (Indian Ocean)",
            ("Indian", AppLanguage::Spanish) => "Océano Índico (Indian)",

            ("Pacific", AppLanguage::French) => "Pacifique (Pacific)",
            ("Pacific", AppLanguage::English) => "Pacific Ocean",
            ("Pacific", AppLanguage::Japanese) => "太平洋 (Pacific Ocean)",
            ("Pacific", AppLanguage::Chinese) => "太平洋 (Pacific Ocean)",
            ("Pacific", AppLanguage::Spanish) => "Océano Pacífico (Pacific)",

            _ => "Autre",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TzItem {
    display_name: String,
    raw_name: String,
    tz: Tz,
}

struct TimezoneCatalog {
    // raw continent key -> Vec<TzItem>
    continents: BTreeMap<String, Vec<TzItem>>,
}

impl TimezoneCatalog {
    fn new() -> Self {
        let mut continents: BTreeMap<String, Vec<TzItem>> = BTreeMap::new();
        let valid_continents = [
            "Africa", "America", "Antarctica", "Asia", "Atlantic",
            "Australia", "Europe", "Indian", "Pacific",
        ];

        for tz in chrono_tz::TZ_VARIANTS {
            let name = tz.name();
            if let Some((raw_continent, location)) = name.split_once('/') {
                if valid_continents.contains(&raw_continent) {
                    let display_location = location.replace('_', " ").replace('/', " - ");
                    continents
                        .entry(raw_continent.to_string())
                        .or_default()
                        .push(TzItem {
                            display_name: display_location,
                            raw_name: name.to_string(),
                            tz,
                        });
                }
            }
        }

        for list in continents.values_mut() {
            list.sort_by(|a, b| a.display_name.cmp(&b.display_name));
            list.dedup_by(|a, b| a.display_name == b.display_name);
        }

        Self { continents }
    }

    fn find_display_name(&self, tz: Tz) -> String {
        let name = tz.name();
        for list in self.continents.values() {
            if let Some(item) = list.iter().find(|it| it.tz == tz) {
                return format!("{} ({})", item.display_name, item.raw_name);
            }
        }
        name.to_string()
    }
}

/// Résolution du chemin multiplateforme :
/// - Linux / macOS : $HOME/.digital_clock
/// - Windows : %USERPROFILE%\.digital_clock
fn get_config_file_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)?;
    Some(home.join(".digital_clock"))
}

/// Chargement des fuseaux et de la langue depuis USERDIR/.digital_clock
fn load_config() -> (AppLanguage, Vec<Tz>) {
    let mut list = Vec::new();
    let mut lang = AppLanguage::French;

    if let Some(path) = get_config_file_path() {
        if let Ok(content) = fs::read_to_string(&path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                if let Some(code) = trimmed.strip_prefix("LANG=") {
                    lang = AppLanguage::from_code(code.trim());
                    continue;
                }
                if let Ok(tz) = Tz::from_str(trimmed) {
                    if !list.contains(&tz) {
                        list.push(tz);
                    }
                }
            }
        }
    }
    (lang, list)
}

/// Sauvegarde persistante des fuseaux et de la langue dans USERDIR/.digital_clock
fn save_config(lang: AppLanguage, clocks: &[CustomClock]) {
    if let Some(path) = get_config_file_path() {
        let mut content = String::from("# Digital Clock - Configuration\n");
        content.push_str(&format!("LANG={}\n", lang.code()));
        for clock in clocks {
            content.push_str(clock.tz.name());
            content.push('\n');
        }
        if let Err(err) = fs::write(&path, content) {
            eprintln!("Avertissement : impossible d'écrire dans {:?} : {}", path, err);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct CustomClock {
    id: usize,
    title: String,
    tz: Tz,
}

enum ClockCardData<'a> {
    Fixed {
        title: &'a str,
        subtitle: String,
        time_str: String,
        date_str: String,
        digit_color: Color32,
    },
    Custom {
        id: usize,
        title: String,
        subtitle: String,
        time_str: String,
        date_str: String,
        digit_color: Color32,
    },
}

struct DigitalClockApp {
    catalog: TimezoneCatalog,
    language: AppLanguage,
    selected_continent_key: String,
    selected_tz_index: usize,
    search_filter: String,
    custom_clocks: Vec<CustomClock>,
    next_clock_id: usize,
    use_24h_format: bool,
    show_seconds: bool,
    show_date: bool,
}

impl Default for DigitalClockApp {
    fn default() -> Self {
        let catalog = TimezoneCatalog::new();
        let first_continent_key = "Europe".to_string();

        let (saved_lang, saved_tzs) = load_config();
        let mut custom_clocks = Vec::new();
        let mut next_id = 1;

        for tz in saved_tzs {
            let title = catalog.find_display_name(tz);
            custom_clocks.push(CustomClock {
                id: next_id,
                title,
                tz,
            });
            next_id += 1;
        }

        Self {
            catalog,
            language: saved_lang,
            selected_continent_key: first_continent_key,
            selected_tz_index: 0,
            search_filter: String::new(),
            custom_clocks,
            next_clock_id: next_id,
            use_24h_format: true,
            show_seconds: true,
            show_date: true,
        }
    }
}

impl eframe::App for DigitalClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(200));

        let t = I18n::get(self.language);

        // Détection de la résolution écran de l'utilisateur
        let monitor_height = ctx.input(|i| {
            i.viewport()
                .monitor_size
                .map(|s| s.y)
                .unwrap_or_else(|| i.screen_rect().height())
        });

        // Contrainte : chaque colonne ne peut pas dépasser la résolution écran
        let max_column_height = (monitor_height - 80.0_f32).max(280.0_f32);

        let now_local = Local::now();
        let now_utc = Utc::now();

        let time_format = if self.use_24h_format {
            if self.show_seconds { "%H:%M:%S" } else { "%H:%M" }
        } else {
            if self.show_seconds { "%I:%M:%S %p" } else { "%I:%M %p" }
        };

        let local_time_str = now_local.format(time_format).to_string();
        let utc_time_str = now_utc.format(time_format).to_string();
        let local_date_str = now_local.format("%A %d %B %Y").to_string();
        let utc_date_str = now_utc.format("%A %d %B %Y").to_string();
        let local_tz_offset = now_local.offset().to_string();

        let mut all_clocks: Vec<ClockCardData> = Vec::new();

        // 1. Horloge Locale
        all_clocks.push(ClockCardData::Fixed {
            title: t.local_time_title,
            subtitle: format!("{}{}", t.tz_offset_prefix, local_tz_offset),
            time_str: local_time_str,
            date_str: local_date_str,
            digit_color: Color32::from_rgb(50, 220, 150),
        });

        // 2. Horloge UTC
        all_clocks.push(ClockCardData::Fixed {
            title: t.utc_time_title,
            subtitle: t.utc_reference.to_string(),
            time_str: utc_time_str,
            date_str: utc_date_str,
            digit_color: Color32::from_rgb(60, 185, 255),
        });

        // 3. Horloges personnalisées
        for clock in &self.custom_clocks {
            let tz_now = now_utc.with_timezone(&clock.tz);
            let tz_time_str = tz_now.format(time_format).to_string();
            let tz_date_str = tz_now.format("%A %d %B %Y").to_string();
            let tz_offset_str = tz_now.offset().to_string();

            all_clocks.push(ClockCardData::Custom {
                id: clock.id,
                title: format!("🌍 {}", clock.title.to_uppercase()),
                subtitle: format!("UTC{}", tz_offset_str),
                time_str: tz_time_str,
                date_str: tz_date_str,
                digit_color: Color32::from_rgb(255, 185, 60),
            });
        }

        let mut clock_to_remove: Option<usize> = None;
        let mut lang_changed = false;

        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::from_rgb(14, 16, 22)).inner_margin(Margin::same(12.0_f32)))
            .show(ctx, |ui| {
                // En-tête : Titre, Sélecteur de Langue et Options
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new(t.app_title)
                            .size(19.0)
                            .color(Color32::from_rgb(230, 235, 245))
                            .strong(),
                    );
                    ui.add_space(4.0);

                    ui.horizontal_wrapped(|ui| {
                        // Sélecteur de langue
                        ui.label(RichText::new(t.language_label).strong());
                        let prev_lang = self.language;
                        egui::ComboBox::from_id_salt("language_select")
                            .selected_text(self.language.label())
                            .width(130.0)
                            .show_ui(ui, |ui| {
                                for l in [
                                    AppLanguage::French,
                                    AppLanguage::English,
                                    AppLanguage::Japanese,
                                    AppLanguage::Chinese,
                                    AppLanguage::Spanish,
                                ] {
                                    ui.selectable_value(&mut self.language, l, l.label());
                                }
                            });

                        if self.language != prev_lang {
                            lang_changed = true;
                        }

                        ui.add_space(10.0);
                        ui.checkbox(&mut self.use_24h_format, t.format_24h);
                        ui.add_space(8.0);
                        ui.checkbox(&mut self.show_seconds, t.show_seconds);
                        ui.add_space(8.0);
                        ui.checkbox(&mut self.show_date, t.show_date);
                        ui.add_space(10.0);

                        if let Some(path) = get_config_file_path() {
                            ui.label(
                                RichText::new(format!("{} {}", t.config_label, path.display()))
                                    .size(10.5)
                                    .color(Color32::from_rgb(120, 130, 145)),
                            );
                        }
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Multi-colonnes : défilement horizontal fluide, chaque colonne bornée par max_column_height
                ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        let card_height_estimate = 90.0_f32;
                        let max_items_per_col = ((max_column_height - 20.0_f32) / card_height_estimate).floor().max(2.0) as usize;

                        let total_clocks = all_clocks.len();
                        let num_columns = (total_clocks + max_items_per_col - 1) / max_items_per_col;
                        let num_columns = num_columns.max(1);

                        for col_idx in 0..num_columns {
                            let start_idx = col_idx * max_items_per_col;
                            let end_idx = (start_idx + max_items_per_col).min(total_clocks);

                            ui.allocate_ui_with_layout(
                                Vec2::new(380.0_f32, max_column_height),
                                egui::Layout::top_down(egui::Align::Center),
                                |ui| {
                                    ScrollArea::vertical()
                                        .id_salt(format!("col_scroll_{}", col_idx))
                                        .max_height(max_column_height)
                                        .show(ui, |ui| {
                                            ui.set_width(370.0_f32);
                                            for clock_data in &all_clocks[start_idx..end_idx] {
                                                match clock_data {
                                                    ClockCardData::Fixed {
                                                        title,
                                                        subtitle,
                                                        time_str,
                                                        date_str,
                                                        digit_color,
                                                    } => {
                                                        render_clock_card(
                                                            ui,
                                                            title,
                                                            subtitle,
                                                            time_str,
                                                            date_str,
                                                            self.show_date,
                                                            *digit_color,
                                                            None,
                                                            t.remove_tooltip,
                                                        );
                                                    }
                                                    ClockCardData::Custom {
                                                        id,
                                                        title,
                                                        subtitle,
                                                        time_str,
                                                        date_str,
                                                        digit_color,
                                                    } => {
                                                        if render_clock_card(
                                                            ui,
                                                            title,
                                                            subtitle,
                                                            time_str,
                                                            date_str,
                                                            self.show_date,
                                                            *digit_color,
                                                            Some(*id),
                                                            t.remove_tooltip,
                                                        ) {
                                                            clock_to_remove = Some(*id);
                                                        }
                                                    }
                                                }
                                                ui.add_space(6.0);
                                            }
                                        });
                                },
                            );

                            ui.add_space(8.0);
                        }

                        // Colonne dédiée à l'ajout de fuseaux horaires
                        ui.allocate_ui_with_layout(
                            Vec2::new(360.0_f32, max_column_height),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                ScrollArea::vertical()
                                    .id_salt("add_tz_col_scroll")
                                    .max_height(max_column_height)
                                    .show(ui, |ui| {
                                        ui.set_width(350.0_f32);
                                        render_add_timezone_section(self, &t, ui);
                                    });
                            },
                        );
                    });
                });
            });

        // Persistance immédiate si changement de langue
        if lang_changed {
            save_config(self.language, &self.custom_clocks);
        }

        // Persistance immédiate si suppression d'horloge
        if let Some(id) = clock_to_remove {
            self.custom_clocks.retain(|c| c.id != id);
            save_config(self.language, &self.custom_clocks);
        }
    }
}

fn render_add_timezone_section(app: &mut DigitalClockApp, t: &I18n, ui: &mut egui::Ui) {
    Frame::none()
        .fill(Color32::from_rgb(22, 25, 35))
        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(45, 52, 72)))
        .rounding(8.0_f32)
        .inner_margin(Margin::same(12.0_f32))
        .show(ui, |ui| {
            ui.label(
                RichText::new(t.add_tz_title)
                    .size(13.5)
                    .color(Color32::from_rgb(220, 225, 240))
                    .strong(),
            );
            ui.add_space(6.0);

            // 1. Sélection du Continent
            ui.label(RichText::new(t.step_continent).strong());
            let prev_continent = app.selected_continent_key.clone();
            let current_continent_label = t.continent_name(&app.selected_continent_key, app.language);

            egui::ComboBox::from_id_salt("continent_select")
                .selected_text(current_continent_label)
                .width(ui.available_width() - 8.0_f32)
                .show_ui(ui, |ui| {
                    for key in app.catalog.continents.keys() {
                        let label = t.continent_name(key, app.language);
                        ui.selectable_value(
                            &mut app.selected_continent_key,
                            key.clone(),
                            label,
                        );
                    }
                });

            if app.selected_continent_key != prev_continent {
                app.selected_tz_index = 0;
                app.search_filter.clear();
            }

            ui.add_space(8.0);

            // 2. Recherche et Sélection du Pays / Ville
            if let Some(tz_list) = app.catalog.continents.get(&app.selected_continent_key) {
                let filtered: Vec<(usize, &TzItem)> = tz_list
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| {
                        if app.search_filter.trim().is_empty() {
                            true
                        } else {
                            item.display_name
                                .to_lowercase()
                                .contains(&app.search_filter.to_lowercase())
                        }
                    })
                    .collect();

                ui.label(RichText::new(t.step_search_city).strong());
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut app.search_filter);
                    if !app.search_filter.is_empty() && ui.small_button("✖").clicked() {
                        app.search_filter.clear();
                    }
                });

                ui.add_space(6.0);

                if !filtered.is_empty() {
                    if !filtered.iter().any(|(orig_idx, _)| *orig_idx == app.selected_tz_index) {
                        app.selected_tz_index = filtered[0].0;
                    }

                    ui.label(RichText::new(t.selection_label).size(11.5));
                    let current_tz = &tz_list[app.selected_tz_index];
                    egui::ComboBox::from_id_salt("city_select")
                        .selected_text(&current_tz.display_name)
                        .width(ui.available_width() - 8.0_f32)
                        .show_ui(ui, |ui| {
                            for (orig_idx, item) in &filtered {
                                ui.selectable_value(
                                    &mut app.selected_tz_index,
                                    *orig_idx,
                                    &item.display_name,
                                );
                            }
                        });

                    ui.add_space(10.0);

                    // Bouton Ajouter & Sauvegarder
                    let chosen = &tz_list[app.selected_tz_index];
                    let already_added = app.custom_clocks.iter().any(|c| c.tz == chosen.tz);

                    if already_added {
                        ui.colored_label(Color32::from_rgb(180, 180, 100), t.already_added);
                    } else if ui.button(RichText::new(t.add_and_save_button).strong()).clicked() {
                        app.custom_clocks.push(CustomClock {
                            id: app.next_clock_id,
                            title: format!("{} ({})", chosen.display_name, chosen.raw_name),
                            tz: chosen.tz,
                        });
                        app.next_clock_id += 1;
                        save_config(app.language, &app.custom_clocks);
                    }
                } else {
                    ui.colored_label(Color32::from_rgb(200, 150, 100), t.no_city_found);
                }
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(4.0);
            ui.label(
                RichText::new(t.save_notice)
                    .size(10.5)
                    .color(Color32::from_rgb(140, 145, 160)),
            );
        });
}

fn render_clock_card(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    time_str: &str,
    date_str: &str,
    show_date: bool,
    digit_color: Color32,
    removable_id: Option<usize>,
    remove_tooltip: &str,
) -> bool {
    let mut delete_clicked = false;

    Frame::none()
        .fill(Color32::from_rgb(24, 27, 37))
        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(42, 47, 65)))
        .rounding(8.0_f32)
        .inner_margin(Margin::symmetric(12.0_f32, 6.0_f32))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(title)
                            .size(12.0)
                            .color(Color32::from_rgb(185, 190, 205))
                            .strong(),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if removable_id.is_some() {
                            if ui
                                .button(RichText::new("✖").color(Color32::from_rgb(230, 80, 80)).size(11.0))
                                .on_hover_text(remove_tooltip)
                                .clicked()
                            {
                                delete_clicked = true;
                            }
                        }
                        ui.label(
                            RichText::new(subtitle)
                                .size(10.5)
                                .color(Color32::from_rgb(130, 135, 150)),
                        );
                    });
                });

                ui.add_space(2.0);

                ui.label(
                    RichText::new(time_str)
                        .font(egui::FontId::monospace(32.0))
                        .color(digit_color)
                        .strong(),
                );

                if show_date {
                    ui.add_space(1.0);
                    ui.label(
                        RichText::new(date_str)
                            .size(11.0)
                            .color(Color32::from_rgb(155, 160, 175)),
                    );
                }
            });
        });

    delete_clicked
}

/// Configuration des polices avec fallback multiplateforme pour le rendu des caractères CJK (Japonais, Chinois)
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Candidats de polices CJK courantes selon le système d'exploitation
    let candidate_paths = [
        // Linux (Arch / Debian / Ubuntu / Fedora)
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Bold.ttc",
        "/usr/share/fonts/OTF/ipagp.ttf",
        "/usr/share/fonts/wenquanyi/wqy-microhei-lite/wqy-microhei-lite.ttc",
        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/TTF/IBMPlexSansJP-Text.ttf",
        // Windows
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\meiryo.ttc",
        "C:\\Windows\\Fonts\\simsun.ttc",
        // macOS
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        "/Library/Fonts/Arial Unicode.ttf",
    ];

    for path in candidate_paths {
        if let Ok(font_bytes) = fs::read(path) {
            fonts.font_data.insert(
                "cjk_fallback".to_owned(),
                egui::FontData::from_owned(font_bytes),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .push("cjk_fallback".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("cjk_fallback".to_owned());
            break;
        }
    }

    ctx.set_fonts(fonts);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(820.0, 580.0))
            .with_min_inner_size(Vec2::new(420.0, 320.0))
            .with_title("Horloge Digitale Multi-Fuseaux (UTC & Local)"),
        ..Default::default()
    };

    eframe::run_native(
        "digital-clock",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(DigitalClockApp::default()))
        }),
    )
}
