use chrono::{Local, Utc};
use eframe::egui::{self, Color32, Frame, Margin, RichText, Stroke, Vec2};
use std::time::Duration;

struct DigitalClockApp {
    use_24h_format: bool,
    show_seconds: bool,
    show_date: bool,
}

impl Default for DigitalClockApp {
    fn default() -> Self {
        Self {
            use_24h_format: true,
            show_seconds: true,
            show_date: true,
        }
    }
}

impl eframe::App for DigitalClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Rafraîchissement périodique fluide pour l'affichage des secondes
        ctx.request_repaint_after(Duration::from_millis(200));

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

        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::from_rgb(15, 17, 23)).inner_margin(Margin::same(16.0)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("⏱ HORLOGE DIGITALE")
                            .color(Color32::from_rgb(220, 225, 235))
                            .strong(),
                    );
                    ui.add_space(8.0);

                    // Cadre Heure Locale
                    render_clock_card(
                        ui,
                        "HEURE LOCALE",
                        &format!("Fuseau : UTC{}", local_tz_offset),
                        &local_time_str,
                        &local_date_str,
                        self.show_date,
                        Color32::from_rgb(50, 215, 150), // Vert digital
                    );

                    ui.add_space(10.0);

                    // Cadre Heure UTC
                    render_clock_card(
                        ui,
                        "TEMPS UNIVERSEL COORDONNÉ (UTC)",
                        "Référence : UTC+00:00 (Z)",
                        &utc_time_str,
                        &utc_date_str,
                        self.show_date,
                        Color32::from_rgb(60, 180, 255), // Bleu / Cyan digital
                    );

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Contrôles utilisateur
                    ui.horizontal_wrapped(|ui| {
                        ui.checkbox(&mut self.use_24h_format, "Format 24h");
                        ui.add_space(12.0);
                        ui.checkbox(&mut self.show_seconds, "Secondes");
                        ui.add_space(12.0);
                        ui.checkbox(&mut self.show_date, "Date");
                    });
                });
            });
    }
}

fn render_clock_card(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    time_str: &str,
    date_str: &str,
    show_date: bool,
    digit_color: Color32,
) {
    Frame::none()
        .fill(Color32::from_rgb(25, 28, 38))
        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(45, 50, 68)))
        .rounding(8.0_f32)
        .inner_margin(Margin::symmetric(16.0_f32, 10.0_f32))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(title)
                            .size(13.0)
                            .color(Color32::from_rgb(180, 185, 200))
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(subtitle)
                                .size(11.0)
                                .color(Color32::from_rgb(130, 135, 150)),
                        );
                    });
                });

                ui.add_space(4.0);

                // Affichage digital principal (police monospace et grande taille)
                ui.label(
                    RichText::new(time_str)
                        .font(egui::FontId::monospace(38.0))
                        .color(digit_color)
                        .strong(),
                );

                if show_date {
                    ui.add_space(2.0);
                    ui.label(
                        RichText::new(date_str)
                            .size(12.0)
                            .color(Color32::from_rgb(160, 165, 180)),
                    );
                }
            });
        });
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(480.0, 360.0))
            .with_min_inner_size(Vec2::new(400.0, 300.0))
            .with_title("Horloge Digitale - UTC & Locale"),
        ..Default::default()
    };

    eframe::run_native(
        "digital-clock",
        options,
        Box::new(|_cc| Ok(Box::new(DigitalClockApp::default()))),
    )
}
