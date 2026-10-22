use android_activity::AndroidApp;
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};
use std::time::Instant;

/// Glitchy Gecko — Pure-Rust Immediate Mode Boilerplate
pub struct GeckoApp {
    start_time: Instant,
    click_count: u64,
    text_buffer: String,
    speed_slider: f32,
    sine_phase: f32,
}

impl GeckoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            start_time: Instant::now(),
            click_count: 0,
            text_buffer: "Hello from Pure Rust on Android!".into(),
            speed_slider: 1.0,
            sine_phase: 0.0,
        }
    }
}

impl eframe::App for GeckoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Advance animation phase
        self.sine_phase += 0.05 * self.speed_slider;
        let uptime_secs = self.start_time.elapsed().as_secs_f32();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(32.0); // Safe area inset for status bar

            ui.heading("🦎 Glitchy Gecko — Egui Raw APK");
            ui.label(format!("Uptime: {:.1}s | Frames: 60 FPS", uptime_secs));
            ui.separator();

            // 1. Interactive Custom Painter Canvas (Animated Sine Wave / Bars)
            let canvas_width = ui.available_width();
            let (rect, _response) =
                ui.allocate_exact_size(Vec2::new(canvas_width, 140.0), Sense::hover());

            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, 8.0, Color32::from_rgb(13, 17, 23));
            painter.rect_stroke(
                rect,
                8.0,
                Stroke::new(1.0_f32, Color32::from_rgb(48, 54, 61)),
                egui::StrokeKind::Inside,
            );

            let num_bars = 32;
            let bar_w = rect.width() / (num_bars as f32);
            for i in 0..num_bars {
                let norm = (i as f32) / (num_bars as f32);
                let wave = ((norm * 6.28 + self.sine_phase).sin() * 0.5 + 0.5).clamp(0.05, 1.0);
                let bar_h = wave * (rect.height() - 20.0);

                let bar_rect = Rect::from_min_max(
                    Pos2::new(rect.left() + (i as f32) * bar_w + 1.0, rect.bottom() - 10.0 - bar_h),
                    Pos2::new(rect.left() + ((i + 1) as f32) * bar_w - 1.0, rect.bottom() - 10.0),
                );

                let color = Color32::from_rgb(
                    (88.0 + norm * 140.0) as u8,
                    (166.0 - norm * 60.0) as u8,
                    255,
                );
                painter.rect_filled(bar_rect, 2.0, color);
            }

            ui.add_space(16.0);

            // 2. Interactive Controls
            ui.horizontal(|ui| {
                if ui.button("⚡ Click Me!").clicked() {
                    self.click_count += 1;
                    log::info!("Gecko clicked! Total: {}", self.click_count);
                }
                ui.label(format!("Clicks: {}", self.click_count));
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.speed_slider, 0.1..=4.0));
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Message:");
                ui.text_edit_singleline(&mut self.text_buffer);
            });

            ui.add_space(16.0);
            ui.separator();
            ui.label(format!("Echo: \"{}\"", self.text_buffer));
        });

        // Request continuous redraw for smooth 60 FPS animation
        ctx.request_repaint();
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    log::info!("Starting Glitchy Gecko NativeActivity...");

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "GlitchyGecko",
        options,
        Box::new(|cc| Ok(Box::new(GeckoApp::new(cc)))),
    )
    .unwrap();
}
