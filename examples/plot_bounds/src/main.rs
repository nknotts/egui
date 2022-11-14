#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{
    self,
    plot::{Line, Plot, PlotPoints},
};

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let num_row: f32 = 3.0;
            let row_height = ui.available_height() / (num_row) - 20.0;

            let x_range = 0..1000;

            let sin: PlotPoints = x_range
                .clone()
                .map(|i| {
                    let x = i as f64 * 0.01;
                    [x, x.sin()]
                })
                .collect();

            let single1: PlotPoints = x_range
                .clone()
                .map(|i| {
                    let x = i as f64 * 0.01;
                    [x, 1.0]
                })
                .collect();

            let single2: PlotPoints = x_range
                .clone()
                .map(|i| {
                    let x = i as f64 * 0.01;
                    [x, 2.0]
                })
                .collect();

            ui.label("Sine Wave: Working");
            Plot::new("plot1")
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .height(row_height)
                .show(ui, |plot_ui| plot_ui.line(Line::new(sin)));

            ui.label("All values == 1, visible, barely");
            Plot::new("plot2")
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .height(row_height)
                .show(ui, |plot_ui| plot_ui.line(Line::new(single1)));

            ui.label("All values == 2, completely missing");
            Plot::new("plot3")
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .height(row_height)
                .show(ui, |plot_ui| plot_ui.line(Line::new(single2)));
        });
    }
}
