#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod generator;
pub mod patient;
pub mod simulation;

use eframe::egui;
use egui_extras::{Column, TableBody, TableBuilder};
use patient::Patient;
use simulation::Simulation;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    data: Simulation,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            data: Simulation::new(),
        }
    }
}

fn create_patient_row(body: &mut TableBody, patient: &Patient) {
    body.row(20.0, |mut row| {
        row.col(|ui| {
            ui.label(patient.name.clone());
        });
        row.col(|ui| {
            ui.label(patient.state.to_string());
        });
        row.col(|ui| {
            ui.label(patient.lep.to_string());
        });
    });
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Patient");
                    });
                    header.col(|ui| {
                        ui.heading("Status");
                    });
                    header.col(|ui| {
                        ui.heading("LeP");
                    });
                })
                .body(|mut body| {
                    for patient in &self.data.patients {
                        create_patient_row(&mut body, patient)
                    }
                });
        });
    }
}
