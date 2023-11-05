#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod git_handler;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut command_text= "h".to_owned();
    let mut age = 42;

    eframe::run_simple_native("Git Chat", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("git chat");
            ui.horizontal(|ui| {
                let name_label = ui.label("message: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                let command_label = ui.label("output: ");
                ui.text_edit_singleline(&mut command_text)
                    .labelled_by(command_label.id);
            });

            if ui.button("Send Message").clicked() {
                command_text = git_handler::send_message(&name);
            }
        });
    })
}