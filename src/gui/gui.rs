use crate::AlsData;
use eframe::egui::{self, widgets::Spinner, Align, Label, SelectableLabel};
use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

pub struct Gui {
    receiver: mpsc::Receiver<Vec<AlsData>>,
    all_als: Option<Vec<AlsData>>,
    selected_als: Option<usize>,

    frame_time: Duration,
}

impl Gui {
    fn new(receiver: mpsc::Receiver<Vec<AlsData>>) -> Self {
        Self {
            receiver,
            all_als: None,
            selected_als: None,

            frame_time: Duration::new(0, 0),
        }
    }
    pub fn run(rx: mpsc::Receiver<Vec<AlsData>>) {
        eframe::run_native(
            "Palsa",
            eframe::NativeOptions::default(),
            Box::new(|_cc| Box::new(Gui::new(rx))),
        )
        .expect("Failed to run eframe app!");
    }

    /// An info bar which displays cool info such as the number of files loaded and the frametime
    fn info_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("info bar").show(ctx, |ui| {
            // Left to right side ui elements
            ui.with_layout(egui::Layout::left_to_right(Align::TOP), |ui| {
                if let Some(ref all_als) = self.all_als {
                    ui.add(Label::new(
                        egui::RichText::new(format!("Loaded {} als files :)", all_als.len()))
                            .size(15.0),
                    ));
                }

                // Displays frame time of the application
                ui.with_layout(egui::Layout::right_to_left(Align::TOP), |ui| {
                    ui.add(Label::new(
                        egui::RichText::new(format!("{:?}", self.frame_time)).size(15.0),
                    ));
                });
            });
        });
    }

    fn als_panel(&self, ctx: &egui::Context, all_als: &Vec<AlsData>) -> Option<usize> {
        let mut selected_index = self.selected_als;

        egui::SidePanel::left("Als files panel").show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (i, als) in all_als.iter().enumerate() {
                        let is_selected = self.selected_als == Some(i);
                        let response = ui.add(SelectableLabel::new(is_selected, &als.name));

                        if response.clicked() {
                            if is_selected {
                                selected_index = None;
                            } else {
                                selected_index = Some(i);
                            }
                        }
                    }
                });
        });
        selected_index
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame_start = Instant::now();

        if let Ok(received) = self.receiver.try_recv() {
            self.all_als = Some(received);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(all_als) = &self.all_als {
                if let Some(selected_als) = self.selected_als {
                    ui.add(Label::new(format!("{:?}", all_als[selected_als])));
                }
                self.selected_als = self.als_panel(ctx, all_als);
            } else {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        ui.add(Spinner::new().size(150.0));
                        ui.add(Label::new("Loading..."));
                    },
                );
            }
        });

        self.info_bar(ctx);
        self.frame_time = frame_start.elapsed();
        ctx.request_repaint();
    }
}
